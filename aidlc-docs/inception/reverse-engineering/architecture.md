# Architecture Documentation

## Business Overview

book-summary-agentは、ISBN（国際標準図書番号）を入力として書籍の200文字程度の日本語要約を生成するコマンドラインツールです。ユーザーはISBNを指定するだけで、Claude Code CLIを介してWeb検索を行い、書籍情報の収集と要約生成を自動的に実行します。

## System Architecture

```
[ ユーザー ]
    |
    | ISBN入力 (CLI引数)
    v
[ book-summary-agent (Rust binary) ]
    |
    | claude -p "..." --allowedTools WebSearch,WebFetch
    v
[ Claude Code CLI (サブプロセス) ]
    |
    | WebSearch / WebFetch (自律的なWeb検索)
    v
[ Web (書籍情報) ]
    |
    | JSON形式で返答
    v
[ book-summary-agent ]
    |
    | テキスト or JSON出力
    v
[ stdout ]
```

## Technology Stack

| 項目 | 内容 |
|------|------|
| 言語 | Rust 1.94.1 |
| ランタイム | Tokio (非同期) |
| HTTP クライアント | なし（削除済み） |
| CLI フレームワーク | clap 4.x |
| JSON処理 | serde / serde_json |
| エラー処理 | anyhow |
| 外部依存 | Claude Code CLI (`claude`コマンド) |

## Component Inventory

| コンポーネント | 種別 | 説明 |
|----------------|------|------|
| `Cli` struct | データ構造 | CLI引数定義（isbn, json, model） |
| `BookInfo` struct | データ構造 | 書籍情報（isbn, title, author, publisher, pubdate, cover_url, summary） |
| `main()` | エントリポイント | ISBN正規化 → エージェント呼び出し → 出力 |
| `normalize_isbn()` | ユーティリティ | ハイフン除去、ISBN正規化 |
| `search_and_summarize()` | コアロジック | Claudeエージェント呼び出し・JSONパース |
| `extract_json()` | ユーティリティ | レスポンスからJSONブロック抽出 |

## Code Structure

```
book-summary-agent/
├── src/
│   └── main.rs          # 全ロジック (130行)
├── Cargo.toml           # 依存関係定義
├── Cargo.lock           # 依存関係ロック
├── CLAUDE.md            # AI-DLC コアワークフロー
├── README.md            # ユーザー向けドキュメント
└── aidlc-docs/          # AI-DLC ドキュメント
```

## Dependencies

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
clap = { version = "4", features = ["derive"] }
anyhow = "1"
tokio = { version = "1", features = ["full"] }
```

## Interaction Diagram

```
main()
  |
  +-- normalize_isbn(isbn)
  |     └── ハイフン除去・文字フィルタ
  |
  +-- validate ISBN length (10 or 13)
  |
  +-- search_and_summarize(isbn, model)
  |     |
  |     +-- プロンプト構築
  |     +-- tokio::process::Command::new("claude")
  |     |     args: ["-p", prompt, "--model", model,
  |     |            "--allowedTools", "WebSearch,WebFetch"]
  |     +-- stdout解析
  |     +-- extract_json(text)
  |     +-- serde_json::from_str → BookInfo
  |
  +-- 出力 (--json or テキスト形式)
```
