# Tech Stack Decisions — book-summary-agent

## 言語・ランタイム

| 決定 | 選択 | 理由 |
|------|------|------|
| 実装言語 | Rust 1.75+ | 高速なバイナリ・型安全性・依存関係の明確性 |
| 非同期ランタイム | tokio (full features) | CLIツールとしての `tokio::main` マクロ利用 |
| エディション | 2021 | 現行標準 |

## 依存クレート

| クレート | バージョン | 用途 | 採用理由 |
|---------|-----------|------|---------|
| `clap` | 4.x (derive feature) | CLIパース | デファクトスタンダード、derive マクロで宣言的 |
| `serde` + `serde_json` | 1.x | JSON シリアライズ/デシリアライズ | Rust エコシステムの標準 |
| `anyhow` | 1.x | エラーハンドリング | ライブラリではないCLIツールに適した anyエラー型 |
| `tokio` | 1.x | 非同期ランタイム | `tokio::process::Command` で子プロセス実行 |

## 不採用の判断

| 候補 | 不採用理由 |
|------|-----------|
| `reqwest` | Claude CLIサブプロセス経由のためHTTPクライアント不要 |
| `thiserror` | ライブラリではなくCLIツールのため `anyhow` で十分 |
| `clap` の `env` feature | API キー廃止により環境変数読み込み不要 |
| OpenBD API | Claude エージェントが自律的にWebから情報収集するため不要 |
| Anthropic Rust SDK | Claude Code CLI サブプロセス経由のためSDK不要 |

## 外部ツール依存

| ツール | バージョン | 用途 |
|--------|-----------|------|
| Claude Code CLI (`claude`) | 2.x系 | エージェント呼び出し (WebSearch/WebFetch) |

## アーキテクチャ選択

- **単一バイナリ**: 配布・インストールが容易
- **単一ファイル構成 (`src/main.rs`)**: 小規模ツールとして複数モジュール分割は不要
- **サブプロセス経由**: Anthropic認証をClaude CLIに委譲。APIキー管理コードが不要
