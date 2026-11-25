//! Unit tests for the JSON/JSONC/JSON5 minifier.

use super::*;

fn minify(source: &str) -> String {
    minify_json(source, &JSON, &MinifierOptions::default())
}

fn minify_json5(source: &str) -> String {
    minify_json(source, &JSON5, &MinifierOptions::default())
}

#[test]
fn removes_whitespace_between_tokens() {
    let source = "{\n  \"a\" : 1 ,\n  \"b\" : 2\n}\n";
    assert_eq!(minify(source), "{\"a\":1,\"b\":2}");
}

#[test]
fn removes_line_comments() {
    let source = "{\n  // a comment\n  \"a\": 1\n}\n";
    assert_eq!(minify(source), "{\"a\":1}");
}

#[test]
fn removes_block_comments() {
    let source = "{ /* block */ \"a\": 1 }";
    assert_eq!(minify(source), "{\"a\":1}");
}

#[test]
fn removes_trailing_comma_in_object() {
    let source = "{\"a\": 1, \"b\": 2,}";
    assert_eq!(minify(source), "{\"a\":1,\"b\":2}");
}

#[test]
fn removes_trailing_comma_in_array() {
    let source = "[1, 2, 3,]";
    assert_eq!(minify(source), "[1,2,3]");
}

#[test]
fn keeps_comment_markers_inside_strings() {
    let source = "{\"url\": \"http://example.com/*path*//\"}";
    assert_eq!(
        minify(source),
        "{\"url\":\"http://example.com/*path*//\"}"
    );
}

#[test]
fn keeps_escaped_quotes_inside_strings() {
    let source = "{\"a\": \"he said \\\"hi\\\"\"}";
    assert_eq!(minify(source), "{\"a\":\"he said \\\"hi\\\"\"}");
}

#[test]
fn keeps_escaped_backslashes_inside_strings() {
    let source = "{\"path\": \"C:\\\\temp\\\\file\"}";
    assert_eq!(minify(source), "{\"path\":\"C:\\\\temp\\\\file\"}");
}

#[test]
fn removes_comments_between_values() {
    let source = "[1, /* two */ 2, // three\n 3]";
    assert_eq!(minify(source), "[1,2,3]");
}

#[test]
fn supports_single_quoted_strings() {
    let source = "{'a': 'value'}";
    assert_eq!(minify_json5(source), "{'a':'value'}");
}

#[test]
fn keeps_comment_markers_inside_single_quoted_strings() {
    let source = "{'url': 'http://x/*y*/'}";
    assert_eq!(minify_json5(source), "{'url':'http://x/*y*/'}");
}

#[test]
fn supports_unquoted_json5_keys() {
    let source = "{ key: 1, other: 2 }";
    assert_eq!(minify_json5(source), "{key:1,other:2}");
}

#[test]
fn removes_bom() {
    let source = "\u{feff}{\"a\": 1}";
    assert_eq!(minify(source), "{\"a\":1}");
}

#[test]
fn returns_empty_for_empty_input() {
    assert_eq!(minify(""), "");
    assert_eq!(minify("   \n\t\n"), "");
}

#[test]
fn preserves_nested_structures() {
    let source = "{\n  \"a\": [\n    1,\n    {\"b\": true}\n  ]\n}";
    assert_eq!(minify(source), "{\"a\":[1,{\"b\":true}]}");
}

#[test]
fn preserves_number_formats() {
    let source = "{\"a\": -1.5e10, \"b\": 0.001}";
    assert_eq!(minify(source), "{\"a\":-1.5e10,\"b\":0.001}");
}

#[test]
fn preserves_unicode_content() {
    let source = "{\"greeting\": \"שלום\"}";
    assert_eq!(minify(source), "{\"greeting\":\"שלום\"}");
}

#[test]
fn preserves_line_breaks_when_requested() {
    let source = "{\n  \"a\": 1\n}";
    let result = minify_json(
        source,
        &JSON,
        &MinifierOptions::line_preserving(),
    );
    assert_eq!(result, "{\n\"a\":1\n}");
}

#[test]
fn does_not_confuse_lone_slash_with_comment() {
    let source = "{\"a\": \"x\"} / trailing";
    // A single slash is not a comment, so the text survives (spaces removed).
    assert!(minify(source).contains("/trailing"));
}

#[test]
fn empty_object_and_array() {
    assert_eq!(minify("{}"), "{}");
    assert_eq!(minify("[ ]"), "[]");
    assert_eq!(minify("{ }"), "{}");
}
