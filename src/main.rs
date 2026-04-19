use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use serde::Serialize;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

    /// 使用するLLM CLI
    #[arg(long, value_enum, default_value_t = LlmProvider::Codex)]
    provider: LlmProvider,

    /// 使用するモデル名（CLI側の既定値を使う場合は省略可）
    #[arg(long)]
    model: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
enum LlmProvider {
    Codex,
    Kiro,
    Gemini,
    GithubCopilot,
}

impl LlmProvider {
    fn display_name(&self) -> &'static str {
        match self {
            Self::Codex => "Codex CLI",
            Self::Kiro => "Kiro CLI",
            Self::Gemini => "Gemini CLI",
            Self::GithubCopilot => "GitHub Copilot CLI",
        }
    }

    fn command(&self) -> String {
        match self {
            Self::Codex => std::env::var("BOOK_SUMMARY_AGENT_CODEX_CMD")
                .unwrap_or_else(|_| "codex".to_string()),
            Self::Kiro => std::env::var("BOOK_SUMMARY_AGENT_KIRO_CMD")
                .unwrap_or_else(|_| "kiro-cli".to_string()),
            Self::Gemini => std::env::var("BOOK_SUMMARY_AGENT_GEMINI_CMD")
                .unwrap_or_else(|_| "gemini".to_string()),
            Self::GithubCopilot => std::env::var("BOOK_SUMMARY_AGENT_GITHUB_COPILOT_CMD")
                .unwrap_or_else(|_| "copilot".to_string()),
        }
    }
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

struct PreparedCommand {
    program: String,
    args: Vec<String>,
    output_path: Option<PathBuf>,
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

    eprintln!(
        "🤖 {} がISBNを検索して概要を生成中... (ISBN: {})",
        cli.provider.display_name(),
        isbn
    );
    let book = search_and_summarize(&isbn, &cli.provider, cli.model.as_deref()).await?;
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

// ─── LLM CLIでWeb検索→書籍情報取得→要約生成 ───────────────────────────
async fn search_and_summarize(
    isbn: &str,
    provider: &LlmProvider,
    model: Option<&str>,
) -> Result<BookInfo> {
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

    let prepared = prepare_command(provider, model, &prompt)?;
    let output = tokio::process::Command::new(&prepared.program)
        .args(&prepared.args)
        .output()
        .await
        .with_context(|| {
            format!(
                "{} の実行に失敗しました。`{}` コマンドがPATHに存在するか確認してください",
                provider.display_name(),
                prepared.program
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("{} エラー: {}", provider.display_name(), stderr.trim());
    }

    let text = read_command_output(&prepared, output.stdout)
        .await
        .with_context(|| format!("{} の出力読み取りに失敗しました", provider.display_name()))?;
    let json_str = extract_json(text.trim())?;
    let parsed: serde_json::Value =
        serde_json::from_str(&json_str).context("LLM CLIレスポンスのJSONパースに失敗")?;

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

fn prepare_command(
    provider: &LlmProvider,
    model: Option<&str>,
    prompt: &str,
) -> Result<PreparedCommand> {
    let program = provider.command();
    let mut args = Vec::new();
    let mut output_path = None;

    match provider {
        LlmProvider::Codex => {
            let path = temp_output_path("codex-last-message.txt");
            args.extend([
                "exec".to_string(),
                "--skip-git-repo-check".to_string(),
                "--sandbox".to_string(),
                "read-only".to_string(),
                "--output-last-message".to_string(),
                path.display().to_string(),
            ]);
            if let Some(model) = model {
                args.extend(["--model".to_string(), model.to_string()]);
            }
            args.push(prompt.to_string());
            output_path = Some(path);
        }
        LlmProvider::Gemini => {
            args.extend(["-p".to_string(), prompt.to_string()]);
            if let Some(model) = model {
                args.extend(["--model".to_string(), model.to_string()]);
            }
        }
        LlmProvider::Kiro => {
            if model.is_some() {
                anyhow::bail!(
                    "Kiro CLI はモデルの都度指定をサポートしていません。`kiro-cli settings chat.defaultModel <MODEL>` で既定モデルを設定してください"
                );
            }
            args.extend([
                "chat".to_string(),
                "--no-interactive".to_string(),
                "--trust-all-tools".to_string(),
                prompt.to_string(),
            ]);
        }
        LlmProvider::GithubCopilot => {
            args.extend([
                "-p".to_string(),
                prompt.to_string(),
                "-s".to_string(),
                "--no-ask-user".to_string(),
            ]);
            if let Some(model) = model {
                args.extend(["--model".to_string(), model.to_string()]);
            }
        }
    }

    Ok(PreparedCommand {
        program,
        args,
        output_path,
    })
}

async fn read_command_output(prepared: &PreparedCommand, stdout: Vec<u8>) -> Result<String> {
    let stdout_text = String::from_utf8(stdout).context("CLIの標準出力がUTF-8ではありません")?;

    if let Some(path) = &prepared.output_path {
        match tokio::fs::read_to_string(path).await {
            Ok(text) if !text.trim().is_empty() => {
                let _ = tokio::fs::remove_file(path).await;
                return Ok(text);
            }
            _ => {
                let _ = tokio::fs::remove_file(path).await;
            }
        }
    }

    Ok(stdout_text)
}

fn temp_output_path(file_name: &str) -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default();
    std::env::temp_dir().join(format!(
        "book-summary-agent-{}-{}-{}",
        std::process::id(),
        millis,
        file_name
    ))
}

// ─── JSONの抽出（```json ... ``` ブロックに対応） ──────────────────────
fn extract_json(text: &str) -> Result<String> {
    let cleaned = strip_ansi_escape_sequences(text);
    let trimmed = cleaned.trim();

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
    if let Some(candidate) = find_json_object(trimmed) {
        return Ok(candidate);
    }

    anyhow::bail!(
        "JSONを抽出できませんでした: {}",
        &trimmed[..trimmed.len().min(100)]
    )
}

fn strip_ansi_escape_sequences(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            if matches!(chars.peek(), Some('[')) {
                chars.next();
                while let Some(next) = chars.next() {
                    if ('@'..='~').contains(&next) {
                        break;
                    }
                }
                continue;
            }
        }
        result.push(ch);
    }

    result
}

fn find_json_object(text: &str) -> Option<String> {
    let bytes = text.as_bytes();

    for start in text.match_indices('{').map(|(idx, _)| idx) {
        let mut depth = 0usize;
        let mut in_string = false;
        let mut escaped = false;

        for (offset, byte) in bytes[start..].iter().enumerate() {
            let ch = *byte as char;

            if in_string {
                if escaped {
                    escaped = false;
                    continue;
                }
                match ch {
                    '\\' => escaped = true,
                    '"' => in_string = false,
                    _ => {}
                }
                continue;
            }

            match ch {
                '"' => in_string = true,
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        let candidate = &text[start..start + offset + 1];
                        if serde_json::from_str::<serde_json::Value>(candidate).is_ok() {
                            return Some(candidate.to_string());
                        }
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_isbn() {
        assert_eq!(normalize_isbn("978-4-87311-903-8"), "9784873119038");
        assert_eq!(normalize_isbn("4-87311-903-X"), "487311903X");
    }

    #[test]
    fn extracts_plain_json() {
        let text = r#"{"title":"test"}"#;
        assert_eq!(extract_json(text).unwrap(), text);
    }

    #[test]
    fn extracts_fenced_json() {
        let text = "```json\n{\"title\":\"test\"}\n```";
        assert_eq!(extract_json(text).unwrap(), "{\"title\":\"test\"}");
    }

    #[test]
    fn extracts_json_with_leading_progress_text() {
        let text = "Searching the web for: ISBN 4478120501\n{\"title\":\"test\"}";
        assert_eq!(extract_json(text).unwrap(), "{\"title\":\"test\"}");
    }

    #[test]
    fn extracts_json_with_ansi_sequences() {
        let text = "\u{1b}[1mSearching\u{1b}[0m\n{\"title\":\"test\"}\n";
        assert_eq!(extract_json(text).unwrap(), "{\"title\":\"test\"}");
    }
}
