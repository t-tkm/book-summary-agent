# Requirements Document

## Intent Analysis

**ユーザーの意図**: ISBNを入力するだけで書籍の概要（200文字程度の日本語）を自動生成するCLIツールを提供する。

**対象ユーザー**: Claude Code CLIを持つ開発者・書籍管理者

## Functional Requirements

### FR-01: ISBN入力
- 10桁または13桁のISBNを受け付ける
- ハイフン付きISBN（例: `978-4-87311-903-8`）をサポートする
- 不正なISBNの場合はエラーメッセージを表示して終了する

### FR-02: 書籍情報収集
- Claude Code CLI（`claude`コマンド）を使用する
- `WebSearch` / `WebFetch` ツールによりWebから書籍情報を自律的に収集する
- 収集項目: タイトル、著者、出版社、出版日、表紙URL、200文字要約

### FR-03: 出力形式
- デフォルト: 人間が読みやすいテキスト形式
- `--json` フラグ: JSON形式（パイプライン連携向け）

### FR-04: モデル指定
- `--model` オプションでClaudeモデルを指定可能
- デフォルト: `claude-sonnet-4-20250514`

## Non-Functional Requirements

### NFR-01: 前提条件
- Claude Code CLI（`claude`コマンド）がPATHに存在すること
- Claudeのサブスクリプション（ProまたはMaxプラン）が有効であること

### NFR-02: セキュリティ
- APIキーをコードにハードコードしない
- Claude Code CLIの認証機構に依存する（APIキー不要）

### NFR-03: 互換性
- Rust 1.75以上
- macOS / Linux対応

### NFR-04: 出力品質
- 要約は200文字程度の日本語
- 読者が「読みたい」と思える内容の核心を捉えた紹介文

## Constraints

- Claude Code CLIの利用にはサブスクリプション費用が発生する（無料ではない）
- Web検索結果の品質はClaudeの判断に依存する
- オフライン環境では動作しない

## Out of Scope

- データベースへの書籍情報保存
- バッチ処理（複数ISBNの一括処理）
- GUIインターフェース
