# Unit Test Instructions — book-summary-agent

## テスト実行

```bash
cargo test
```

## テスト対象関数

現在、以下の純粋関数がテスト可能:

### `normalize_isbn()`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_isbn_strips_hyphens() {
        assert_eq!(normalize_isbn("978-4-297-12783-1"), "9784297127831");
    }

    #[test]
    fn test_normalize_isbn_strips_spaces() {
        assert_eq!(normalize_isbn("978 4297127831"), "9784297127831");
    }

    #[test]
    fn test_normalize_isbn_isbn10_with_x() {
        assert_eq!(normalize_isbn("4-06-123456-X"), "406123456X");
    }
}
```

### `extract_json()`

```rust
    #[test]
    fn test_extract_json_from_json_block() {
        let input = "```json\n{\"title\": \"本\"}\n```";
        assert!(extract_json(input).is_ok());
    }

    #[test]
    fn test_extract_json_from_bare_object() {
        let input = "{\"title\": \"本\"}";
        assert_eq!(extract_json(input).unwrap(), input);
    }

    #[test]
    fn test_extract_json_fails_on_plain_text() {
        assert!(extract_json("JSONではないテキスト").is_err());
    }
```

## 注意事項

- `search_and_summarize()` は Claude CLI に依存するためユニットテスト対象外
- 外部依存のある関数のテストはインテグレーションテストに委ねる
