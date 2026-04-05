# book-summary-agent

ISBNから書籍の概要を200文字程度で生成するRust製CLIエージェント。

## アーキテクチャ

```
ISBN入力
  │
  ▼
┌─────────────────────────────┐
│ Claude エージェント          │
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

#### 2. Claude Code CLI

概要生成に `claude` コマンドを使用します。利用には **Claudeのサブスクリプション（ProまたはMaxプラン）** が必要です。

1. [claude.ai](https://claude.ai) でProまたはMaxプランに登録
2. Claude Code CLIをインストール:

```bash
npm install -g @anthropic-ai/claude-code
```

3. 認証を完了させる:

```bash
claude
# ブラウザが開き、ログイン・認証が完了する
```

4. 動作確認:

```bash
claude -p "hello"
```

### ビルド

```bash
cargo build --release
```

バイナリは `target/release/book-summary-agent` に生成されます。

## 使い方

### 基本

```bash
# ISBNを指定して実行
book-summary-agent 9784873119038

# ハイフン付きISBNも可
book-summary-agent 978-4-87311-903-8
```

### オプション

```bash
# JSON形式で出力（パイプライン連携向き）
book-summary-agent 9784873119038 --json

# 使用モデルを変更
book-summary-agent 9784873119038 --model claude-opus-4-6

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
2. **Claude エージェント実行** — `WebSearch` / `WebFetch` ツールを使い、ISBNでWeb検索して書籍情報（タイトル・著者・出版社・出版日・表紙URL）を自律的に収集する
3. **概要生成** — 収集した情報をもとに、読者が読みたくなるような200文字程度の日本語要約を生成する
4. **出力** — テキスト形式 or JSON形式

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
