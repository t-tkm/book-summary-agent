# Domain Entities — book-summary-agent

## BookInfo

書籍情報の統合表現。Claudeエージェントが収集した全情報を保持する。

| フィールド | 型 | 説明 |
|------------|-----|------|
| `isbn` | String | 正規化済みISBN（ハイフンなし） |
| `title` | String | 書籍タイトル |
| `author` | String | 著者名 |
| `publisher` | String | 出版社名 |
| `pubdate` | String | 出版日（YYYYMMDD形式、不明時は空文字） |
| `cover_url` | String | 表紙画像URL（不明時は空文字） |
| `summary` | String | 200文字程度の日本語要約 |

## CLI 入力パラメータ (Cli)

| フィールド | 型 | デフォルト | 説明 |
|------------|-----|-----------|------|
| `isbn` | String | 必須 | ISBN（10桁/13桁、ハイフン可） |
| `json` | bool | false | JSON形式で出力するか |
| `model` | String | `claude-sonnet-4-20250514` | 使用Claudeモデル |
