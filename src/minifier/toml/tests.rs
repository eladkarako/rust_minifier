use super::*;

#[test]
fn test_removes_equals_spacing() {
    let input = "authors = [\"Elad\"]";
    let result = TomlMinifier.minify(input).unwrap();
    assert_eq!(result, "authors=[\"Elad\"]");
}

#[test]
fn test_removes_comma_spacing() {
    let input =
        "categories = [\"command-line\", \"text-processing\"]";
    let result = TomlMinifier.minify(input).unwrap();
    assert_eq!(
        result,
        "categories=[\"command-line\",\"text-processing\"]"
    );
}

#[test]
fn test_removes_brace_spacing() {
    let input =
        "clap = { version = \"4.6.7\", features = [\"derive\"] }";
    let result = TomlMinifier.minify(input).unwrap();
    assert_eq!(
        result,
        "clap={version=\"4.6.7\",features=[\"derive\"]}"
    );
}

#[test]
fn test_removes_leading_whitespace() {
    let input = "  \t  key = \"value\"";
    let result = TomlMinifier.minify(input).unwrap();
    assert_eq!(result, "key=\"value\"");
}

#[test]
fn test_removes_trailing_whitespace_after_comment() {
    let input = "key = \"value\"          # comment";
    let result = TomlMinifier.minify(input).unwrap();
    assert_eq!(result, "key=\"value\"");
}

#[test]
fn test_preserves_strings() {
    let input = "description = \"# not a comment\"";
    let result = TomlMinifier.minify(input).unwrap();
    assert!(result.contains("# not a comment"));
}

#[test]
fn test_removes_inline_comments() {
    let input = "key = \"value\" # this is a comment";
    let result = TomlMinifier.minify(input).unwrap();
    assert!(!result.contains("comment"));
    assert!(result.contains("key="));
}

#[test]
fn test_removes_blank_lines() {
    let input = "key1 = \"value1\"\n\nkey2 = \"value2\"";
    let result = TomlMinifier.minify(input).unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 2);
}

#[test]
fn test_preserves_sections() {
    let input = "[package]\nname = \"app\"";
    let result = TomlMinifier.minify(input).unwrap();
    assert!(result.contains("[package]"));
}

#[test]
fn test_preserves_array_of_tables() {
    let input = "[[bin]]\nname = \"main\"";
    let result = TomlMinifier.minify(input).unwrap();
    assert!(result.contains("[[bin]]"));
}

#[test]
fn test_handles_tabs() {
    let input = "key\t=\t\"value\"";
    let result = TomlMinifier.minify(input).unwrap();
    assert_eq!(result, "key=\"value\"");
}
