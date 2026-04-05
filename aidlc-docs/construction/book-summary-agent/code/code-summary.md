# Code Summary — book-summary-agent

## ファイル構成

```
src/
└── main.rs   (156行)
```

## 関数・構造体一覧

### `struct Cli`
CLIパラメータ定義。`clap::Parser` derive マクロで自動生成。

| フィールド | 型 | 説明 |
|-----------|-----|------|
| `isbn` | String | 位置引数。ISBN (10/13桁、ハイフン可) |
| `json` | bool | `--json` フラグ。JSON形式出力 |
| `model` | String | `--model` オプション。デフォルト `claude-sonnet-4-20250514` |

### `struct BookInfo`
Claude エージェントから収集した書籍情報。`serde::Serialize` 派生で JSON 出力対応。

| フィールド | 型 |
|-----------|-----|
| `isbn`, `title`, `author`, `publisher`, `pubdate`, `cover_url`, `summary` | String |

### `fn main() -> Result<()>`
エントリポイント。処理フロー:
1. `Cli::parse()` で引数解析
2. `normalize_isbn()` でISBN正規化
3. 桁数バリデーション (10 or 13桁)
4. `search_and_summarize()` 呼び出し
5. `--json` / テキスト形式出力

### `fn normalize_isbn(isbn: &str) -> String`
ハイフン除去。ASCII数字と `X` のみ保持。

### `async fn search_and_summarize(isbn: &str, model: &str) -> Result<BookInfo>`
Claude CLI をサブプロセスとして実行。
- コマンド: `claude -p {prompt} --model {model} --allowedTools WebSearch,WebFetch`
- 終了コード非0時はエラー伝播
- stdout から `extract_json()` でJSON抽出
- `serde_json::Value` にパース後 `BookInfo` に変換

### `fn extract_json(text: &str) -> Result<String>`
優先順位:
1. ` ```json ``` ` ブロック
2. ` ``` ``` ` ブロック
3. `{` で始まるテキスト
4. 上記すべて失敗 → `anyhow::bail!`

## 依存関係グラフ

```
main()
  ├─ normalize_isbn()
  └─ search_and_summarize()
        └─ extract_json()
```

## 行数統計

| セクション | 行数 (概算) |
|-----------|------------|
| Cli 定義 | 21 |
| BookInfo 定義 | 10 |
| main() | 36 |
| normalize_isbn() | 4 |
| search_and_summarize() | 46 |
| extract_json() | 24 |
| コメント・空行 | 15 |
| **合計** | **156** |
