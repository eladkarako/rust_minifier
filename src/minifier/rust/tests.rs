use super::*;

#[test]
fn test_remove_line_comments() {
    let source = "let x = 5; // this is a comment";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("//"));
    assert!(result.contains("let x = 5;"));
}

#[test]
fn test_remove_block_comments() {
    let source = "let x = 5; /* block comment */ let y = 10;";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("/*"));
    assert!(result.contains("let x = 5;"));
    assert!(result.contains("let y = 10;"));
}

#[test]
fn test_remove_rustdoc_comments() {
    let source = "/// Documentation\nfn foo() {}";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("///"));
    assert!(result.contains("fn foo()"));
}

#[test]
fn test_preserve_strings_with_comments() {
    let source = r#"let s = "// not a comment";"#;
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(result.contains("// not a comment"));
}

#[test]
fn test_minify_basic_function() {
    let minifier = RustMinifier;
    let source = "fn main() { println!(\"hello\"); }";
    let result = minifier.minify(source).unwrap();
    assert_eq!(result.contains("println"), true);
    assert!(result.len() <= source.len());
}

#[test]
fn test_minify_empty_source() {
    let minifier = RustMinifier;
    let result = minifier.minify("").unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_minify_preserves_raw_strings() {
    let minifier = RustMinifier;
    let source = r#"let s = r"raw // string";"#;
    let result = minifier.minify(source).unwrap();
    assert!(result.contains("raw"));
}
