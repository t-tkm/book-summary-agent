# Business Logic Model — book-summary-agent

## Overview

ISBNを受け取り、Claudeエージェントを介してWebから書籍情報を収集し、200文字程度の日本語要約を生成して出力する。

## Core Business Process

```
入力: ISBN文字列
  │
  ├─ [正規化] ハイフン除去・文字フィルタ
  ├─ [バリデーション] 10桁 or 13桁チェック
  │
  ├─ [エージェント呼び出し] claude CLI (WebSearch/WebFetch)
  │     └─ プロンプト: ISBNでWeb検索 → JSON形式で書籍情報+要約を返す
  │
  ├─ [レスポンス解析] JSONブロック抽出 → BookInfo構造体
  │
  └─ [出力] テキスト形式 or JSON形式
出力: 書籍タイトル・著者・要約など
```

## Business Transactions

### BT-01: ISBN正規化
- ハイフン（`-`）を除去する
- ASCII数字と `X`（ISBN-10の最終桁）のみを残す
- 正規化後の桁数を検証する（10桁または13桁）

### BT-02: エージェント呼び出し
- `claude -p {prompt} --model {model} --allowedTools WebSearch,WebFetch` を実行
- Claudeが自律的にWebを検索し、書籍情報と200文字要約をJSON形式で返す
- 標準出力を受け取る

### BT-03: レスポンス解析
1. レスポンステキストから`\`\`\`json ... \`\`\``ブロックを優先抽出
2. 次に`\`\`\` ... \`\`\``ブロックを試みる
3. `{`で始まるならそのまま使用
4. いずれも失敗したらエラーを返す

### BT-04: 出力整形
- `--json`フラグあり: JSONオブジェクトをpretty-print
- `--json`フラグなし: 区切り線付きの人間向けテキスト形式
