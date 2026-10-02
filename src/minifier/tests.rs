use super::*;

// ============================================================================
// Minify Function Tests
// ============================================================================

#[test]
fn test_minify_empty_string() {
    let result = minify("").unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_minify_whitespace_only() {
    let result = minify("   \n\n\t\t  ").unwrap();
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
fn test_minify_removes_comments() {
    let source = "fn main() { // comment\n}";
    let result = minify(source).unwrap();
    assert!(!result.contains("//"));
    assert!(result.contains("fn"));
}

#[test]
fn test_minify_preserves_functionality() {
    let source = "fn add(a: i32, b: i32) -> i32 { a + b }";
    let result = minify(source).unwrap();
    assert!(result.contains("fn"));
    assert!(result.contains("add"));
    assert!(result.contains("a"));
    assert!(result.contains("b"));
    assert!(result.contains("+"));
}

#[test]
fn test_minify_complex_code() {
    let source = r#"
    /// Documentation
    /// More docs
    fn calculate(x: i32) -> i32 {
        // Calculate something
        let result = x * 2; // double it
        result + 1 /* add one */
    }
    "#;
    let result = minify(source).unwrap();
    assert!(!result.contains("///"));
    assert!(!result.contains("//"));
    assert!(!result.contains("/*"));
    assert!(result.contains("fn"));
    assert!(result.contains("calculate"));
}

#[test]
fn test_minify_generic_code() {
    let source = "fn generic<T: Clone>(t: T) -> T { t.clone() }";
    let result = minify(source).unwrap();
    assert!(result.contains("fn"));
    assert!(result.contains("generic"));
}

#[test]
fn test_minify_with_attributes() {
    let source = r#"
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    "#;
    let result = minify(source).unwrap();
    assert!(result.contains("struct"));
    assert!(result.contains("Point"));
}

#[test]
fn test_minify_with_attributes_and_comments() {
    let source = r#"
    /// Point in 2D space
    #[derive(Debug)]
    struct Point {
        x: i32, // x coordinate
        y: i32, // y coordinate
    }
    "#;
    let result = minify(source).unwrap();
    assert!(!result.contains("///"));
    assert!(!result.contains("//"));
    assert!(result.contains("struct"));
}

// ============================================================================
// Comment and Rustdoc Removal Tests
// ============================================================================

#[test]
fn test_remove_single_line_comments() {
    let source = "let x = 5; // comment\nlet y = 10;";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("// comment"));
    assert!(result.contains("let x"));
    assert!(result.contains("let y"));
}

#[test]
fn test_remove_single_line_comments_multiple() {
    let source = r#"
    let a = 1; // first
    let b = 2; // second
    let c = 3; // third
    "#;
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("// first"));
    assert!(!result.contains("// second"));
    assert!(!result.contains("// third"));
}

#[test]
fn test_remove_rustdoc_single_line() {
    let source = "/// This is rustdoc\nfn foo() {}";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("///"));
    assert!(result.contains("fn"));
}

#[test]
fn test_remove_rustdoc_single_line_bang() {
    let source = "//! Module documentation\nfn foo() {}";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("//!"));
    assert!(result.contains("fn"));
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
fn test_remove_multi_line_comments_nested_like() {
    let source = "let x = /* outer /* inner */ outer */ 5;";
    let result = remove_comments_and_rustdocs(source).unwrap();
    // First /* */ closes at the first */
    assert!(result.contains("let x"));
}

#[test]
fn test_remove_rustdoc_multi_line() {
    let source = "/** Documentation */\nfn bar() {}";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("/**"));
    assert!(result.contains("fn"));
}

#[test]
fn test_remove_rustdoc_multi_line_bang() {
    let source = "/*! Module docs */\nfn baz() {}";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(!result.contains("/*!"));
    assert!(result.contains("fn"));
}

#[test]
fn test_remove_comments_preserves_newline_after_single_line() {
    let source = "let x = 5;\n// comment\nlet y = 10;";
    let result = remove_comments_and_rustdocs(source).unwrap();
    // Should preserve the newline structure
    assert!(result.contains("let x"));
    assert!(result.contains("let y"));
}

// ============================================================================
// String Literal Preservation Tests
// ============================================================================

#[test]
fn test_preserve_string_literals() {
    let source = r#"let s = "string with // not a comment";"#;
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(result.contains("// not a comment"));
}

#[test]
fn test_preserve_string_with_escaped_quotes() {
    let source = r#"let s = "string with \" escaped quote";"#;
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(result.contains("escaped quote"));
}

#[test]
fn test_preserve_single_quoted_strings() {
    let source = "let c = 'a'; // comment";
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(result.contains("'a'"));
    assert!(!result.contains("// comment"));
}

#[test]
fn test_preserve_raw_string_simple() {
    let source = r#"let s = r"raw string // not a comment";"#;
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(result.contains("// not a comment"));
}

#[test]
fn test_preserve_raw_string_with_hashes() {
    let source = r##"let s = r#"raw string with # inside"#;"##;
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(result.contains("raw string"));
}

#[test]
fn test_preserve_raw_string_multiple_hashes() {
    let source =
        r###"let s = r##"string with # and ## inside"##;"###;
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(result.contains("string with"));
}

#[test]
fn test_preserve_comment_in_string_with_comment_outside() {
    let source = r#"let s = "has // comment"; // actual comment"#;
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(result.contains("has // comment"));
    assert!(!result.contains("// actual comment"));
}

#[test]
fn test_preserve_escaped_backslash_in_string() {
    let source = r#"let s = "path\\to\\file"; // comment"#;
    let result = remove_comments_and_rustdocs(source).unwrap();
    assert!(result.contains("path"));
    assert!(!result.contains("// comment"));
}

// ============================================================================
// Space Between Tokens Tests
// ============================================================================

#[test]
fn test_needs_space_between_identifiers() {
    assert!(needs_space_between("let", "x"));
    assert!(needs_space_between("fn", "foo"));
    assert!(needs_space_between("x", "y"));
    assert!(needs_space_between("return", "value"));
}

#[test]
fn test_needs_space_between_no_space_required() {
    assert!(!needs_space_between("(", "x"));
    assert!(!needs_space_between("x", ")"));
    assert!(!needs_space_between("[", "x"));
    assert!(!needs_space_between("x", ","));
    assert!(!needs_space_between("{", "x"));
    assert!(!needs_space_between("x", "}"));
}

#[test]
fn test_needs_space_between_operators() {
    assert!(!needs_space_between("+", "x"));
    assert!(!needs_space_between("x", "+"));
    assert!(!needs_space_between("=", "x"));
}

#[test]
fn test_needs_space_between_closing_and_opening() {
    assert!(needs_space_between(")", "fn"));
    assert!(needs_space_between("]", "let"));
    assert!(needs_space_between("}", "fn"));
}

#[test]
fn test_needs_space_between_keyword_and_keyword() {
    assert!(needs_space_between("let", "mut"));
    assert!(needs_space_between("async", "fn"));
    assert!(needs_space_between("pub", "fn"));
}

#[test]
fn test_needs_space_between_number_and_ident() {
    assert!(needs_space_between("123", "abc"));
    assert!(needs_space_between("abc", "123"));
}

#[test]
fn test_needs_space_between_underscore_ident() {
    assert!(needs_space_between("_private", "x"));
    assert!(needs_space_between("x", "_private"));
}

// ============================================================================
// Reconstruct Minimal Tests
// ============================================================================

#[test]
fn test_reconstruct_minimal_simple() {
    let tokens: proc_macro2::TokenStream =
        "let x = 5".parse().unwrap();
    let result = reconstruct_minimal(tokens);
    assert!(result.contains("let"));
    assert!(result.contains("x"));
    assert!(result.contains("5"));
}

#[test]
fn test_reconstruct_minimal_with_proper_spacing() {
    let tokens: proc_macro2::TokenStream =
        "let x = 5".parse().unwrap();
    let result = reconstruct_minimal(tokens);
    // Should have space between let and x
    assert!(result.contains("let x"));
}

#[test]
fn test_reconstruct_minimal_function() {
    let tokens: proc_macro2::TokenStream =
        "fn main() {}".parse().unwrap();
    let result = reconstruct_minimal(tokens);
    assert!(result.contains("fn"));
    assert!(result.contains("main"));
}

#[test]
fn test_reconstruct_minimal_preserves_operators() {
    let tokens: proc_macro2::TokenStream = "a + b".parse().unwrap();
    let result = reconstruct_minimal(tokens);
    assert!(result.contains("a"));
    assert!(result.contains("b"));
    assert!(result.contains("+"));
}

#[test]
fn test_reconstruct_minimal_generic_brackets() {
    let tokens: proc_macro2::TokenStream =
        "Vec<i32>".parse().unwrap();
    let result = reconstruct_minimal(tokens);
    assert!(result.contains("Vec"));
    assert!(result.contains("i32"));
}

#[test]
fn test_reconstruct_minimal_no_extra_spaces() {
    let tokens: proc_macro2::TokenStream =
        "let x=5;".parse().unwrap();
    let result = reconstruct_minimal(tokens);
    // Should be reasonably compact
    assert!(result.len() > 0);
    let space_count = result.matches(' ').count();
    assert!(space_count <= 3); // Only necessary spaces
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_minify_invalid_rust_syntax() {
    let source = "fn main() { invalid syntax here }}}";
    let result = minify(source);
    // Should handle gracefully or error
    assert!(result.is_err() || !result.unwrap().is_empty());
}

#[test]
fn test_remove_comments_unterminated_comment() {
    let source = "let x = 5; /* unterminated";
    let result = remove_comments_and_rustdocs(source);
    // Should handle unterminated comment
    assert!(result.is_ok());
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_minify_preserves_string_and_removes_comments() {
    let source = r#"
    // comment
    let msg = "Hello, World! // not a comment";
    // another comment
    println!("{}", msg);
    "#;
    let result = minify(source).unwrap();
    assert!(result.contains("Hello, World! // not a comment"));
    assert!(!result.contains("// comment"));
    assert!(!result.contains("// another comment"));
}

#[test]
fn test_minify_real_world_example() {
    let source = r#"
    /// Calculates the factorial of a number
    fn factorial(n: u32) -> u32 {
        match n {
            0 | 1 => 1,
            // Recursive case
            _ => n * factorial(n - 1),
        }
    }
    "#;
    let result = minify(source).unwrap();
    assert!(!result.contains("///"));
    assert!(!result.contains("// Recursive case"));
    assert!(result.contains("factorial"));
    assert!(result.contains("match"));
}

#[test]
fn test_minify_trait_impl() {
    let source = r#"
    trait Logger {
        /// Log a message
        fn log(&self, msg: &str);
    }

    impl Logger for MyLogger {
        // Implementation
        fn log(&self, msg: &str) {
            println!("{}", msg);
        }
    }
    "#;
    let result = minify(source).unwrap();
    assert!(!result.contains("///"));
    assert!(!result.contains("// Implementation"));
    assert!(result.contains("trait"));
    assert!(result.contains("impl"));
}
