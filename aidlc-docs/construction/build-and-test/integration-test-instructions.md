# Integration Test Instructions — book-summary-agent

## 前提条件

- Claude Code CLI がインストール・認証済みであること
- インターネット接続があること

## 手動インテグレーションテスト手順

### テスト1: 正常系 — テキスト出力

```bash
book-summary-agent 9784297127831
```

**期待結果**:
- stderr に `🤖 Claude がISBNを検索して概要を生成中...` が表示される
- stdout にタイトル・著者・出版社・概要が表示される
- exit code 0

### テスト2: 正常系 — JSON出力

```bash
book-summary-agent --json 9784297127831
```

**期待結果**:
- stdout が有効なJSONオブジェクト
- `isbn`, `title`, `author`, `publisher`, `pubdate`, `cover_url`, `summary` フィールドが存在する
- exit code 0

### テスト3: 正常系 — ハイフン付きISBN

```bash
book-summary-agent 978-4-297-12783-1
```

**期待結果**: テスト1と同一の書籍情報が返る

### テスト4: 異常系 — 不正ISBN

```bash
book-summary-agent 12345
echo "exit code: $?"
```

**期待結果**:
- stderr にバリデーションエラーメッセージ
- exit code 1

### テスト5: モデル指定

```bash
book-summary-agent --model claude-opus-4-6 9784297127831
```

**期待結果**: 指定モデルで正常動作

## 自動化可能な範囲

現在は手動テストのみ。Claude CLI のモック化が困難なため、CI/CDでの自動実行は対象外。
