use anyhow::{Context, Result};
use clap::Parser;
use serde::Serialize;

// ─── CLI ───────────────────────────────────────────────────────────────
#[derive(Parser)]
#[command(name = "book-summary-agent")]
#[command(about = "ISBNから書籍の概要を200文字程度で生成するエージェント")]
#[command(version)]
struct Cli {
    /// ISBN (10桁 or 13桁, ハイフン可)
    isbn: String,

    /// JSON形式で出力
    #[arg(long, default_value_t = false)]
    json: bool,

    /// 使用するClaudeモデル
    #[arg(long, default_value = "claude-sonnet-4-20250514")]
    model: String,
}

// ─── 書籍情報 ──────────────────────────────────────────────────────────
#[derive(Debug, Serialize)]
struct BookInfo {
    isbn: String,
    title: String,
    author: String,
    publisher: String,
    pubdate: String,
    cover_url: String,
    summary: String,
}

// ─── メイン処理 ────────────────────────────────────────────────────────
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let isbn = normalize_isbn(&cli.isbn);

    if isbn.len() != 10 && isbn.len() != 13 {
        anyhow::bail!(
            "ISBNは10桁または13桁である必要があります (入力: {} → 正規化後: {}, {}桁)",
            cli.isbn,
            isbn,
            isbn.len()
        );
    }

    eprintln!("🤖 Claude がISBNを検索して概要を生成中... (ISBN: {})", isbn);
    let book = search_and_summarize(&isbn, &cli.model).await?;
    eprintln!("✅ 完了\n");

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&book)?);
    } else {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📚 {}", book.title);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("著者:     {}", book.author);
        println!("出版社:   {}", book.publisher);
        println!("出版日:   {}", book.pubdate);
        if !book.cover_url.is_empty() {
            println!("表紙URL:  {}", book.cover_url);
        }
        println!("───────────────────────────────────────────");
        println!("【概要】");
        println!("{}", book.summary);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }

    Ok(())
}

// ─── ISBNの正規化（ハイフン除去、数字+X のみ残す） ─────────────────────
fn normalize_isbn(isbn: &str) -> String {
    isbn.chars()
        .filter(|c| c.is_ascii_digit() || *c == 'X')
        .collect()
}

// ─── Claude エージェントでWeb検索→書籍情報取得→要約生成 ────────────────
async fn search_and_summarize(isbn: &str, model: &str) -> Result<BookInfo> {
    let prompt = format!(
        "ISBN {} の書籍についてWebで検索し、以下のJSON形式のみで返してください。\
他の文章は一切不要です。\
不明な項目は空文字にしてください。\
summaryは読者が読みたくなるような内容の核心を捉えた200文字程度の日本語にしてください。\n\n\
{{\
  \"title\": \"\",\
  \"author\": \"\",\
  \"publisher\": \"\",\
  \"pubdate\": \"YYYYMMDD形式\",\
  \"cover_url\": \"\",\
  \"summary\": \"\"\
}}",
        isbn
    );

    let output = tokio::process::Command::new("claude")
        .args([
            "-p",
            &prompt,
            "--model",
            model,
            "--allowedTools",
            "WebSearch,WebFetch",
        ])
        .output()
        .await
        .context(
            "claude CLIの実行に失敗しました。`claude`コマンドがPATHに存在するか確認してください",
        )?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("claude CLI エラー: {}", stderr);
    }

    let text = String::from_utf8(output.stdout).context("claude CLIの出力がUTF-8ではありません")?;
    let json_str = extract_json(text.trim())?;
    let parsed: serde_json::Value =
        serde_json::from_str(&json_str).context("claude CLIレスポンスのJSONパースに失敗")?;

    Ok(BookInfo {
        isbn: isbn.to_string(),
        title: parsed["title"].as_str().unwrap_or_default().to_string(),
        author: parsed["author"].as_str().unwrap_or_default().to_string(),
        publisher: parsed["publisher"].as_str().unwrap_or_default().to_string(),
        pubdate: parsed["pubdate"].as_str().unwrap_or_default().to_string(),
        cover_url: parsed["cover_url"].as_str().unwrap_or_default().to_string(),
        summary: parsed["summary"].as_str().unwrap_or_default().to_string(),
    })
}

// ─── JSONの抽出（```json ... ``` ブロックに対応） ──────────────────────
fn extract_json(text: &str) -> Result<String> {
    let trimmed = text.trim();

    if let Some(start) = trimmed.find("```json") {
        let after = &trimmed[start + 7..];
        if let Some(end) = after.find("```") {
            return Ok(after[..end].trim().to_string());
        }
    }
    if let Some(start) = trimmed.find("```") {
        let after = &trimmed[start + 3..];
        if let Some(end) = after.find("```") {
            return Ok(after[..end].trim().to_string());
        }
    }
    if trimmed.starts_with('{') {
        return Ok(trimmed.to_string());
    }

    anyhow::bail!(
        "JSONを抽出できませんでした: {}",
        &trimmed[..trimmed.len().min(100)]
    )
}
