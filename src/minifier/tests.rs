use super::*;

#[test]
fn test_minify_empty_string() {
    let result = minify("").unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_minify_simple_code() {
    let source = "fn main() { }";
    let result = minify(source).unwrap();
    assert!(!result.is_empty());
    assert!(result.contains("fn"));
    assert!(result.contains("main"));
}

#[test]
fn test_remove_single_line_comments() {
    let source = "let x = 5; // comment\nlet y = 10;";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("// comment"));
    assert!(result.contains("let x"));
    assert!(result.contains("let y"));
}

#[test]
fn test_remove_multi_line_comments() {
    let source = "let x = 5; /* comment */ let y = 10;";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("/* comment */"));
    assert!(result.contains("let x"));
    assert!(result.contains("let y"));
}

#[test]
fn test_remove_rustdoc() {
    let source = "/// This is rustdoc\nfn foo() {}";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("///"));
    assert!(result.contains("fn"));
}

#[test]
fn test_preserve_string_literals() {
    let source = r#"let s = "string with // not a comment";"#;
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(result.contains("// not a comment"));
}

#[test]
fn test_needs_space_between_identifiers() {
    assert!(needs_space_between("let", "x"));
    assert!(needs_space_between("fn", "foo"));
    assert!(needs_space_between("x", "y"));
}

#[test]
fn test_needs_space_between_no_space_required() {
    assert!(!needs_space_between("(", "x"));
    assert!(!needs_space_between("x", ")"));
    assert!(!needs_space_between("[", "x"));
    assert!(!needs_space_between("x", ","));
}

#[test]
fn test_reconstruct_minimal_with_spaces() {
    let tokens: proc_macro2::TokenStream = "let x = 5".parse().unwrap();
    let result = reconstruct_minimal(tokens);
    assert!(result.contains("let"));
    assert!(result.contains("x"));
    assert!(result.contains("5"));
}

