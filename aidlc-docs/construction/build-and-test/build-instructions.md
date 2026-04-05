# Build Instructions — book-summary-agent

## 前提条件

| ツール | バージョン | 確認コマンド |
|--------|-----------|------------|
| Rust | 1.75+ | `rustc --version` |
| Cargo | (Rustに付属) | `cargo --version` |
| Claude Code CLI | 2.x系 | `claude --version` |

## ビルド手順

### 開発ビルド

```bash
cargo build
```

出力: `target/debug/book-summary-agent`

### リリースビルド

```bash
cargo build --release
```

出力: `target/release/book-summary-agent`

### インストール (グローバル)

```bash
cargo install --path .
```

インストール先: `~/.cargo/bin/book-summary-agent`

## 動作確認

```bash
# バージョン表示
book-summary-agent --version

# ヘルプ表示
book-summary-agent --help

# 実行例 (Claude CLI認証済みであること)
book-summary-agent 9784297127831
```

## 既知のビルド上の注意点

- `Cargo.lock` はリポジトリに含まれている（バイナリクレートのベストプラクティス）
- `tokio = { features = ["full"] }` を使用。必要最小限ではないが、単一バイナリのため許容
