# Workflow Plan

## Project Classification
- **Type**: Brownfield（既存コード）
- **Complexity**: Low（単一ファイル、単一ユニット）
- **Depth Level**: Standard

## Execution Plan

### Inception Phase (完了)
- [x] Workspace Detection
- [x] Reverse Engineering（Brownfieldのため実行）
- [x] Requirements Analysis（Standardデプス）
- [x] Workflow Planning
- [-] User Stories（スキップ: 単一CLIツール、マルチユーザー不要）
- [-] Application Design（スキップ: 新コンポーネント不要）
- [-] Units Generation（スキップ: 単一ユニット）

### Construction Phase (オンデマンド実行)
- [ ] Functional Design（条件付き）
- [ ] NFR Requirements（条件付き）
- [ ] Code Generation（必須）
- [ ] Build and Test（必須）

### Operations Phase
- [ ] Operations（Placeholder）

## Unit Definition

| Unit名 | 説明 | 優先度 |
|--------|------|--------|
| `book-summary-agent` | ISBNから書籍概要を生成するRust CLI | High |

## Workflow Visualization

```
[Inception: Complete]
  Workspace Detection --> Reverse Engineering --> Requirements Analysis --> Workflow Planning
                                                                              |
                                                                              v
[Construction: On-demand]
  Functional Design (opt) --> NFR Requirements (opt) --> Code Generation --> Build & Test
                                                                              |
                                                                              v
[Operations: Placeholder]
  Future: Deployment / Monitoring
```

## Decision Log

| 決定事項 | 理由 |
|----------|------|
| User Storiesスキップ | 単一CLIツール。ユーザーは開発者のみ。複数ユーザーペルソナ不要 |
| Application Designスキップ | 既存アーキテクチャで完結。新コンポーネント追加なし |
| Units Generationスキップ | 単一バイナリ、単一ユニット |
| Standardデプス | 明確な要件あり。低リスク変更 |
