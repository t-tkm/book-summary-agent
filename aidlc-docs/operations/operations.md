# Operations — book-summary-agent

## 概要

book-summary-agent はローカル実行CLIツールであるため、従来の意味でのデプロイ・モニタリングは対象外。
本ドキュメントはユーザー向けの運用手順を記載する。

## インストール・アップグレード

### インストール

```bash
git clone https://github.com/<owner>/book-summary-agent.git
cd book-summary-agent
cargo install --path .
```

### アップグレード

```bash
git pull
cargo install --path .  # 上書きインストール
```

### アンインストール

```bash
cargo uninstall book-summary-agent
```

## 実行環境要件

| 要件 | 詳細 |
|------|------|
| OS | macOS / Linux |
| Rust | 1.75+ (`rustup` でインストール推奨) |
| Claude Code CLI | 2.x系 (要認証) |
| ネットワーク | インターネット接続必須 |

## トラブルシューティング

### `claude: command not found`

Claude Code CLI がインストールされていないか、PATH が通っていない。

```bash
# インストール確認
which claude
claude --version
```

### JSON パースエラー

Claude の応答がJSON形式でない場合に発生。`--model` を変えて再試行。

```bash
book-summary-agent --model claude-opus-4-6 <ISBN>
```

### タイムアウト / ネットワークエラー

WebSearch/WebFetch はインターネット接続必須。接続を確認後、再実行。

## ログ・診断

進捗は stderr に出力される。stdout はデータのみ。

```bash
# stderr と stdout を分離して確認
book-summary-agent <ISBN> 2>error.log
```

## セキュリティ運用上の注意

- API キーの管理は Claude Code CLI に委譲。本ツールはAPIキーを扱わない
- ISBNはハイフン除去後に数字+Xのみフィルタされるためインジェクションリスクなし
