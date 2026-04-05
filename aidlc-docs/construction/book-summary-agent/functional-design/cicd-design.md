# CI/CD Design — book-summary-agent

## 概要

GitHub Actions による2ワークフロー構成。

```
PR / main push
  │
  └─► ci.yml ─── fmt check → clippy → cargo test

v* tag / manual
  │
  └─► release.yml ─── マルチプラットフォームビルド → GitHub Release
```

## ワークフロー設計

### ci.yml — 継続的インテグレーション

| 項目 | 設定 |
|------|------|
| トリガー | `push` (main), `pull_request` |
| ランナー | ubuntu-latest (単一) |
| ステップ | fmt check → clippy (-D warnings) → cargo test |
| キャッシュ | `~/.cargo/registry`, `~/.cargo/git`, `target/` (Cargo.lock ハッシュキー) |

### release.yml — マルチプラットフォームリリース

| ターゲット | OS | アーカイブ形式 |
|-----------|-----|--------------|
| x86_64-unknown-linux-gnu | ubuntu-latest | tar.gz |
| aarch64-unknown-linux-gnu | ubuntu-latest (クロスコンパイル) | tar.gz |
| x86_64-apple-darwin | macos-latest | zip |
| aarch64-apple-darwin | macos-latest | zip |
| x86_64-pc-windows-msvc | windows-latest | zip |

| 項目 | 設定 |
|------|------|
| トリガー | `push` (v* タグ), `workflow_dispatch` |
| バイナリ名 | `book-summary-agent` |
| Linux aarch64 | `gcc-aarch64-linux-gnu` クロスコンパイラ使用 |
| Windowsパッケージング | PowerShell `Compress-Archive` |
| リリース作成 | `softprops/action-gh-release@v2`、`generate_release_notes: true` |

## ジョブ依存関係

```
release.yml:
  build (matrix: 5並列)
    └── release (needs: build)
```

## 成果物命名規則

```
book-summary-agent-{tag}-{target}.{tar.gz|zip}

例:
  book-summary-agent-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
  book-summary-agent-v0.1.0-aarch64-apple-darwin.zip
  book-summary-agent-v0.1.0-x86_64-pc-windows-msvc.zip
```

## 参考実装

`t-tkm/book-register` リポジトリの `.github/workflows/` を参考に作成。
