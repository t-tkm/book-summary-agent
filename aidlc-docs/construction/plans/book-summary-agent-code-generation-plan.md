# Code Generation Plan — book-summary-agent

## 対象ユニット
- **Unit**: book-summary-agent
- **ファイル**: `src/main.rs` (単一ファイル構成)

## 実装済みコンポーネント

本プロジェクトはBrownfieldのため、コード生成は既存コードの全面リライトとして実施済み。

### 実施した変更

| 変更内容 | 理由 |
|---------|------|
| OpenBD API 呼び出し削除 | Claude エージェントに統合 |
| Anthropic Claude API HTTP呼び出し削除 | Claude CLI サブプロセスに置き換え |
| `reqwest` 依存削除 | HTTP クライアント不要化 |
| `clap` env feature 削除 | API キー環境変数不要化 |
| `search_and_summarize()` 実装 | Claude CLI サブプロセス経由の単一エージェント関数 |
| `extract_json()` 実装 | ```json```, ```, `{` 各ブロック対応のJSON抽出 |

## コード構造

```
src/main.rs
├── struct Cli          — clap CLI引数定義
├── struct BookInfo     — 書籍情報 (serde Serialize)
├── fn main()           — エントリポイント (ISBN正規化・バリデーション・出力)
├── fn normalize_isbn() — ハイフン除去・ASCII数字+Xフィルタ
├── async fn search_and_summarize() — Claude CLIサブプロセス実行・レスポンス解析
└── fn extract_json()   — JSONブロック抽出ロジック
```

## 実装方針

- **シンプルさ優先**: 130行以内の単一ファイル
- **エラー全伝播**: `?` 演算子と `anyhow` で main まで伝播
- **stderr/stdout 分離**: 進捗は stderr、データは stdout
