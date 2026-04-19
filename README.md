# book-summary-agent

ISBNから書籍の概要を200文字程度で生成するRust製CLIエージェント。
利用する LLM CLI は切り替え可能で、デフォルトは `Codex CLI` です。

## アーキテクチャ

```
ISBN入力
  │
  ▼
┌─────────────────────────────┐
│ LLM CLI エージェント         │
│  1. ISBNでWeb検索            │
│  2. 書籍情報を収集           │
│  3. 200文字要約を生成        │
└────────────┬────────────────┘
             │
             ▼
         概要出力
    (テキスト or JSON)
```

## セットアップ

### 前提条件

#### 1. Rust (1.75+)

[rustup](https://rustup.rs/) でインストールするか、[asdf](https://asdf-vm.com/) を使う場合:

```bash
asdf plugin add rust
asdf install rust latest
asdf set -u rust latest
```

#### 2. 利用したい LLM CLI

デフォルトは `codex` コマンドを使います。必要に応じて `--provider` で切り替えてください。

- Codex CLI: `codex`
- Gemini CLI: `gemini`
- Kiro CLI: `kiro-cli`
- GitHub Copilot CLI: `copilot`

各 CLI のインストールと認証は、それぞれの公式手順に従って事前に完了させてください。

### ビルド

```bash
cargo build --release
```

バイナリは `target/release/book-summary-agent` に生成されます。

## 使い方

### `cargo run` で試す

ビルド済みバイナリを作らずに、そのまま動作確認できます。

```bash
# デフォルトの Codex CLI で実行
cargo run -- 9784873119038

# JSON形式で出力
cargo run -- 9784873119038 --json

# Gemini CLI を使う
cargo run -- 9784873119038 --provider gemini

# モデルを指定する
cargo run -- 9784873119038 --provider codex --model gpt-5.4
```

### 基本

```bash
# ISBNを指定して実行
book-summary-agent 9784873119038

# ハイフン付きISBNも可
book-summary-agent 978-4-87311-903-8

# Gemini CLIを使う
book-summary-agent 9784873119038 --provider gemini
```

### オプション

```bash
# JSON形式で出力（パイプライン連携向き）
book-summary-agent 9784873119038 --json

# 使用するCLIを変更
book-summary-agent 9784873119038 --provider github-copilot

# 使用モデルを変更
book-summary-agent 9784873119038 --provider codex --model gpt-5.4

# ヘルプ
book-summary-agent --help
```

### 出力例（テキスト）

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📚 入門 監視
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
著者:     Mike Julian／松浦隼人
出版社:   オライリー・ジャパン
出版日:   20190117
表紙URL:  https://images-na.ssl-images-amazon.com/...
───────────────────────────────────────────
【概要】
システム運用の要である「監視」を体系的に解説した一冊。
アラート設計やメトリクス収集の基本から、クラウドネイティブ
環境での実践的なモニタリング戦略まで、現場で即活用できる
知見が凝縮されている。
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 出力例（JSON）

```json
{
  "isbn": "9784873119038",
  "title": "入門 監視",
  "author": "Mike Julian／松浦隼人",
  "publisher": "オライリー・ジャパン",
  "pubdate": "20190117",
  "cover_url": "https://images-na.ssl-images-amazon.com/...",
  "summary": "システム運用の要である「監視」を体系的に..."
}
```

## 処理フロー詳細

1. **ISBN正規化** — ハイフン除去、10桁/13桁バリデーション
2. **LLM CLI 実行** — 選択した CLI を使い、ISBNでWeb検索して書籍情報（タイトル・著者・出版社・出版日・表紙URL）を自律的に収集する
3. **概要生成** — 収集した情報をもとに、読者が読みたくなるような200文字程度の日本語要約を生成する
4. **出力** — テキスト形式 or JSON形式

## 対応プロバイダ

- `codex` (`--provider codex`)
- `gemini` (`--provider gemini`)
- `kiro` (`--provider kiro`)
- `github-copilot` (`--provider github-copilot`)

## 注意点

- `--model` は CLI 側が都度指定に対応している場合のみ使えます。
- `Codex CLI` はバージョンによって `exec` の対応オプションが異なります。このツールでは互換性のため `codex exec` の基本オプションのみを使っています。
- `Kiro CLI` はモデルの都度指定に対応していないため、`kiro-cli settings chat.defaultModel <MODEL>` で既定モデルを設定してください。
- `Kiro CLI` はこのツールから非対話で Web 検索を使うため、内部的に `--trust-all-tools` を付けて実行します。
- `GitHub Copilot CLI` のコマンド名が環境内の別ツールと衝突する場合は、`BOOK_SUMMARY_AGENT_GITHUB_COPILOT_CMD` 環境変数で実行コマンドを上書きできます。
- 同様に `BOOK_SUMMARY_AGENT_CODEX_CMD` `BOOK_SUMMARY_AGENT_GEMINI_CMD` `BOOK_SUMMARY_AGENT_KIRO_CMD` でも各 CLI コマンド名を上書きできます。

## Notion連携（応用例）

```bash
# JSON出力をjqでパースしてNotion APIに投げる例
book-summary-agent 9784873119038 --json | jq '{
  parent: {database_id: "YOUR_DB_ID"},
  properties: {
    Title: {title: [{text: {content: .title}}]},
    Author: {rich_text: [{text: {content: .author}}]},
    Summary: {rich_text: [{text: {content: .summary}}]}
  }
}' | curl -X POST https://api.notion.com/v1/pages \
  -H "Authorization: Bearer $NOTION_TOKEN" \
  -H "Notion-Version: 2022-06-28" \
  -H "Content-Type: application/json" \
  -d @-
```

## ライセンス

MIT
