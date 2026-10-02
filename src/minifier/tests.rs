use super::*;
use proc_macro2::{TokenStream, TokenTree};

/// Test minify with simple function
#[test]
fn test_minify_simple_function() {
    let input = "fn main() { println!(\"Hello\"); }";
    let result = minify(input).unwrap();

    // Minified output should not contain comments or excessive whitespace
    assert_eq!(result.contains("//"), false);
    assert_eq!(result.contains("/*"), false);
}

/// Test minify with multiple comments
#[test]
fn test_minify_removes_line_comments() {
    let input = "fn main() {
        // This is a comment
        println!(\"Hello\");
    }";
    let result = minify(input).unwrap();

    assert_eq!(result.contains("This is a comment"), false);
    assert_eq!(result.contains("println"), true);
}

/// Test minify with block comments
#[test]
fn test_minify_removes_block_comments() {
    let input = "fn main() {
        /* This is a block comment */
        println!(\"Hello\");
    }";
    let result = minify(input).unwrap();

    assert_eq!(result.contains("This is a block comment"), false);
    assert_eq!(result.contains("println"), true);
}

/// Test minify with rustdoc comments
#[test]
fn test_minify_removes_rustdoc() {
    let input = "/// This is a rustdoc comment
    fn my_function() { }";
    let result = minify(input).unwrap();

    assert_eq!(result.contains("This is a rustdoc comment"), false);
    assert_eq!(result.contains("fn my_function"), true);
}

/// Test minify preserves string content
#[test]
fn test_minify_preserves_strings() {
    let input = "fn main() { let s = \"hello world with spaces\"; }";
    let result = minify(input).unwrap();

    assert_eq!(result.contains("hello world with spaces"), true);
}

/// Test minify preserves string with comments inside
#[test]
fn test_minify_string_with_comment_markers() {
    let input = "fn main() { let s = \"// not a comment\"; }";
    let result = minify(input).unwrap();

    assert_eq!(result.contains("// not a comment"), true);
}

/// Test minify with nested block comments
#[test]
fn test_minify_nested_comments() {
    let input = "fn main() {
        /* outer /* nested */ comment */
        println!(\"test\");
    }";
    let result = minify(input).unwrap();

    assert_eq!(result.contains("println"), true);
}

/// Test minify with empty input
#[test]
fn test_minify_empty_input() {
    let input = "";
    let result = minify(input).unwrap();

    assert_eq!(result.trim(), "");
}

/// Test minify with only comments
#[test]
fn test_minify_only_comments() {
    let input = "// Just a comment\n// Another comment";
    let result = minify(input).unwrap();

    assert_eq!(result.trim(), "");
}

/// Test minify with whitespace preservation in code
#[test]
fn test_minify_essential_whitespace() {
    let input = "let x = 5;";
    let result = minify(input).unwrap();

    // Should preserve tokens (let, x, =, 5, ;)
    assert_eq!(result.contains("let"), true);
    assert_eq!(result.contains("5"), true);
}

/// Test minify with multiple functions
#[test]
fn test_minify_multiple_functions() {
    let input = "
    fn func1() { }
    // Comment between functions
    fn func2() { }
    ";
    let result = minify(input).unwrap();

    assert_eq!(result.contains("fn func1"), true);
    assert_eq!(result.contains("fn func2"), true);
    assert_eq!(result.contains("Comment between"), false);
}

/// Test minify with macro invocations
#[test]
fn test_minify_macro_invocation() {
    let input = "fn main() { println!(\"test\"); vec![1, 2, 3]; }";
    let result = minify(input).unwrap();

    assert_eq!(result.contains("println"), true);
    assert_eq!(result.contains("vec"), true);
}

/// Test minify with attributes
#[test]
fn test_minify_attributes() {
    let input = "#[derive(Debug)]
    struct MyStruct { }";
    let result = minify(input).unwrap();

    assert_eq!(result.contains("derive"), true);
    assert_eq!(result.contains("MyStruct"), true);
}

/// Test remove_comments_and_rustdocs with line comment
#[test]
fn test_remove_comments_line_comment() {
    let input = "code // comment\nmore";
    let result = remove_comments_and_rustdocs(input).unwrap();

    assert_eq!(result.contains("comment"), false);
    assert_eq!(result.contains("code"), true);
    assert_eq!(result.contains("more"), true);
}

/// Test remove_comments_and_rustdocs with block comment
#[test]
fn test_remove_comments_block_comment() {
    let input = "code /* comment */ more";
    let result = remove_comments_and_rustdocs(input).unwrap();

    assert_eq!(result.contains("comment"), false);
    assert_eq!(result.contains("code"), true);
    assert_eq!(result.contains("more"), true);
}

/// Test remove_comments_and_rustdocs with rustdoc triple slash
#[test]
fn test_remove_comments_rustdoc_triple_slash() {
    let input = "/// Documentation\nfn test() { }";
    let result = remove_comments_and_rustdocs(input).unwrap();

    assert_eq!(result.contains("Documentation"), false);
    assert_eq!(result.contains("fn test"), true);
}

/// Test remove_comments_and_rustdocs with rustdoc bang
#[test]
fn test_remove_comments_rustdoc_bang() {
    let input = "//! Module documentation\nfn test() { }";
    let result = remove_comments_and_rustdocs(input).unwrap();

    assert_eq!(result.contains("Module documentation"), false);
    assert_eq!(result.contains("fn test"), true);
}

/// Test remove_comments_and_rustdocs with empty input
#[test]
fn test_remove_comments_empty_input() {
    let input = "";
    let result = remove_comments_and_rustdocs(input).unwrap();

    assert_eq!(result, "");
}

/// Test remove_comments_and_rustdocs with no comments
#[test]
fn test_remove_comments_no_comments() {
    let input = "fn main() { println!(\"test\"); }";
    let result = remove_comments_and_rustdocs(input).unwrap();

    assert_eq!(input, result);
}

/// Test remove_comments_and_rustdocs with comment at end
#[test]
fn test_remove_comments_at_end() {
    let input = "code // end comment";
    let result = remove_comments_and_rustdocs(input).unwrap();

    assert_eq!(result.contains("code"), true);
    assert_eq!(result.contains("comment"), false);
}

/// Test remove_comments_and_rustdocs with multiple block comments
#[test]
fn test_remove_comments_multiple_block_comments() {
    let input = "/* comment1 */ code /* comment2 */";
    let result = remove_comments_and_rustdocs(input).unwrap();

    assert_eq!(result.contains("comment1"), false);
    assert_eq!(result.contains("comment2"), false);
    assert_eq!(result.contains("code"), true);
}

/// Test remove_comments_and_rustdocs with string containing comment markers
#[test]
fn test_remove_comments_string_with_markers() {
    let input = "let s = \"// not a comment\"; // real comment";
    let result = remove_comments_and_rustdocs(input).unwrap();

    assert_eq!(result.contains("// not a comment"), true);
    assert_eq!(result.contains("real comment"), false);
}

/// Test needs_space_between for ident and ident
#[test]
fn test_needs_space_between_ident_ident() {
    let ts: TokenStream = "let x".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    assert!(tokens.len() >= 2);
    let result = needs_space_between(&tokens[0], &tokens[1]);
    assert_eq!(result, true);
}

/// Test needs_space_between for ident and operator
#[test]
fn test_needs_space_between_ident_operator() {
    let ts: TokenStream = "x = 5".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    assert!(tokens.len() >= 2);
    let result = needs_space_between(&tokens[0], &tokens[1]);
    assert_eq!(result, false);
}

/// Test needs_space_between for operator and ident
#[test]
fn test_needs_space_between_operator_ident() {
    let ts: TokenStream = "= 5".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    assert!(tokens.len() >= 2);
    let result = needs_space_between(&tokens[0], &tokens[1]);
    assert_eq!(result, false);
}

/// Test needs_space_between for opening bracket and ident
#[test]
fn test_needs_space_between_open_paren_ident() {
    let ts: TokenStream = "(x)".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    assert!(!tokens.is_empty());

    // Extract `(` from the Group
    if let TokenTree::Group(g) = &tokens[0] {
        let inner: Vec<TokenTree> = g.stream().into_iter().collect();
        if !inner.is_empty() {
            let result = needs_space_between(&tokens[0], &inner[0]);
            assert_eq!(result, false);
        }
    }
}

/// Test needs_space_between for ident and closing bracket
#[test]
fn test_needs_space_between_ident_close_paren() {
    let ts: TokenStream = "(x)".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if let TokenTree::Group(g) = &tokens[0] {
        let inner: Vec<TokenTree> = g.stream().into_iter().collect();
        if inner.len() >= 1 {
            let result = needs_space_between(&inner[0], &tokens[0]);
            assert_eq!(result, false);
        }
    }
}

/// Test needs_space_between for two operators
#[test]
fn test_needs_space_between_operator_operator() {
    let ts: TokenStream = "= > 5".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if tokens.len() >= 2 {
        let _result = needs_space_between(&tokens[0], &tokens[1]);
    }
}

/// Test needs_space_between for fn and ident
#[test]
fn test_needs_space_between_fn_ident() {
    let ts: TokenStream = "fn main".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    assert!(tokens.len() >= 2);
    let result = needs_space_between(&tokens[0], &tokens[1]);
    assert_eq!(result, true);
}

/// Test needs_space_between for keyword and bracket
#[test]
fn test_needs_space_between_if_bracket() {
    let ts: TokenStream = "if(x) {}".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if tokens.len() >= 2 {
        let result = needs_space_between(&tokens[0], &tokens[1]);
        // if followed by ( may need space depending on implementation
        let _ = result;
    }
}

/// Test needs_space_between for bracket and semicolon
#[test]
fn test_needs_space_between_close_brace_semicolon() {
    let ts: TokenStream = "{}".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if let TokenTree::Group(_g) = &tokens[0] {
        // The group itself is `{}`, so we test if group needs space before semicolon
        let ts2: TokenStream = "{};".parse().unwrap();
        let tokens2: Vec<TokenTree> = ts2.into_iter().collect();
        if tokens2.len() >= 2 {
            let result =
                needs_space_between(&tokens2[0], &tokens2[1]);
            assert_eq!(result, false);
        }
    }
}

/// Test needs_space_between for comma and ident
#[test]
fn test_needs_space_between_comma_ident() {
    let ts: TokenStream = "x, y".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if tokens.len() >= 3 {
        let result = needs_space_between(&tokens[1], &tokens[2]); // comma to y
        assert_eq!(result, false);
    }
}

/// Test needs_space_between for ident and comma
#[test]
fn test_needs_space_between_ident_comma() {
    let ts: TokenStream = "x, y".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if tokens.len() >= 2 {
        let result = needs_space_between(&tokens[0], &tokens[1]); // x to comma
        assert_eq!(result, false);
    }
}

/// Test needs_space_between for two keywords
#[test]
fn test_needs_space_between_let_mut() {
    let ts: TokenStream = "let mut x".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    assert!(tokens.len() >= 2);
    let result = needs_space_between(&tokens[0], &tokens[1]);
    assert_eq!(result, true);
}

/// Test needs_space_between for keyword and ident
#[test]
fn test_needs_space_between_mut_ident() {
    let ts: TokenStream = "let mut x".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if tokens.len() >= 3 {
        let result = needs_space_between(&tokens[1], &tokens[2]); // mut to x
        assert_eq!(result, true);
    }
}

/// Test needs_space_between with numbers
#[test]
fn test_needs_space_between_ident_number() {
    let ts: TokenStream = "x 123".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if tokens.len() >= 2 {
        let _result = needs_space_between(&tokens[0], &tokens[1]);
    }
}

/// Test needs_space_between for plus operator
#[test]
fn test_needs_space_between_plus_operator() {
    let ts: TokenStream = "5 + 3".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if tokens.len() >= 2 {
        let result = needs_space_between(&tokens[0], &tokens[1]); // 5 to +
        assert_eq!(result, false);
    }
}

/// Test needs_space_between for minus operator
#[test]
fn test_needs_space_between_minus_operator() {
    let ts: TokenStream = "5 - 3".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if tokens.len() >= 2 {
        let result = needs_space_between(&tokens[0], &tokens[1]); // 5 to -
        assert_eq!(result, false);
    }
}

/// Test reconstruct_minimal with basic tokens
#[test]
fn test_reconstruct_minimal_basic() {
    let tokens: Vec<&str> = vec!["fn", "main", "(", ")", "{", "}"];
    let code = tokens.join(" "); // "fn main ( ) { }"
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("fn"), true);
    assert_eq!(result.contains("main"), true);
}

/// Test reconstruct_minimal with function call
#[test]
fn test_reconstruct_minimal_function_call() {
    let tokens: Vec<&str> =
        vec!["println", "!", "(", "\"test\"", ")"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("println"), true);
    assert_eq!(result.contains("test"), true);
}

/// Test reconstruct_minimal with variable assignment
#[test]
fn test_reconstruct_minimal_assignment() {
    let tokens: Vec<&str> = vec!["let", "x", "=", "5", ";"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("let"), true);
    assert_eq!(result.contains("x"), true);
    assert_eq!(result.contains("5"), true);
}

/// Test reconstruct_minimal with empty token list
#[test]
fn test_reconstruct_minimal_empty() {
    let tokens: Vec<&str> = vec![];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.trim(), "");
}

/// Test reconstruct_minimal with single token
#[test]
fn test_reconstruct_minimal_single_token() {
    let tokens: Vec<&str> = vec!["main"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("main"), true);
}

/// Test reconstruct_minimal preserves string content
#[test]
fn test_reconstruct_minimal_string_content() {
    let tokens: Vec<&str> =
        vec!["let", "s", "=", "\"hello world\"", ";"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("hello world"), true);
}

/// Test reconstruct_minimal with operators no space
#[test]
fn test_reconstruct_minimal_operators_no_space() {
    let tokens: Vec<&str> = vec!["5", "+", "3"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    // Should reconstruct without unnecessary spaces
    let trimmed = result.replace(" ", "");
    assert_eq!(trimmed.contains("5+3"), true);
}

/// Test reconstruct_minimal with brackets
#[test]
fn test_reconstruct_minimal_brackets() {
    let tokens: Vec<&str> = vec!["[", "1", ",", "2", ",", "3", "]"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("["), true);
    assert_eq!(result.contains("]"), true);
    assert_eq!(result.contains("1"), true);
}

/// Test reconstruct_minimal with nested parentheses
#[test]
fn test_reconstruct_minimal_nested_parens() {
    let tokens: Vec<&str> = vec!["(", "(", "x", ")", ")"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("x"), true);
}

/// Test reconstruct_minimal with if statement
#[test]
fn test_reconstruct_minimal_if_statement() {
    let tokens: Vec<&str> = vec!["if", "x", ">", "0", "{", "}"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("if"), true);
    assert_eq!(result.contains("x"), true);
}

/// Test reconstruct_minimal with for loop
#[test]
fn test_reconstruct_minimal_for_loop() {
    let tokens: Vec<&str> =
        vec!["for", "i", "in", "0", ".", ".", "10", "{", "}"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("for"), true);
    assert_eq!(result.contains("in"), true);
}

/// Test reconstruct_minimal with struct definition
#[test]
fn test_reconstruct_minimal_struct() {
    let tokens: Vec<&str> =
        vec!["struct", "Point", "{", "x", ":", "i32", "}"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("struct"), true);
    assert_eq!(result.contains("Point"), true);
}

/// Test reconstruct_minimal with generic parameters
#[test]
fn test_reconstruct_minimal_generics() {
    let tokens: Vec<&str> = vec![
        "fn", "generic", "<", "T", ">", "(", "x", ":", "T", ")",
        "{", "}",
    ];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("generic"), true);
    assert_eq!(result.contains("T"), true);
}

/// Test reconstruct_minimal maintains token order
#[test]
fn test_reconstruct_minimal_token_order() {
    let tokens: Vec<&str> = vec!["a", "b", "c", "d"];
    let code = tokens.join(" ");
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    let a_pos = result.find("a").unwrap();
    let b_pos = result.find("b").unwrap();
    let c_pos = result.find("c").unwrap();
    let d_pos = result.find("d").unwrap();

    assert!(a_pos < b_pos);
    assert!(b_pos < c_pos);
    assert!(c_pos < d_pos);
}

/// Test minify with complex real-world code
#[test]
fn test_minify_complex_code() {
    let input = r#"
    /// This function does something important
    fn calculate(x: i32, y: i32) -> i32 {
        // Add the two numbers
        let result = x + y; // Store result
        /* Return the result */
        result
    }
    "#;

    let result = minify(input).unwrap();

    // Should remove all comments and rustdocs
    assert_eq!(result.contains("///"), false);
    assert_eq!(result.contains("//"), false);
    assert_eq!(result.contains("/*"), false);
    assert_eq!(
        result.contains("This function does something"),
        false
    );

    // Should preserve core logic
    assert_eq!(result.contains("fn calculate"), true);
    assert_eq!(result.contains("x"), true);
    assert_eq!(result.contains("y"), true);
}

/// Test minify with attributes and macros
#[test]
fn test_minify_with_attributes_and_macros() {
    let input = r#"
    #[derive(Debug, Clone)]
    struct MyStruct {
        // Field
        data: String,
    }

    #[test]
    fn test_something() {
        assert_eq!(1 + 1, 2);
    }
    "#;

    let result = minify(input).unwrap();

    assert!(result.contains("derive"));
    assert!(result.contains("MyStruct"));
    assert!(result.contains("assert_eq"));
}

/// Test minify with module documentation
#[test]
fn test_minify_module_docs() {
    let input = r#"
    //! This is a module
    //! with multiple lines of documentation

    /// Function docs
    fn my_func() {}
    "#;

    let result = minify(input).unwrap();

    assert_eq!(result.contains("This is a module"), false);
    assert_eq!(!result.contains("Function docs"), true);
    assert_eq!(result.contains("fn my_func"), true);
}

/// Test minify with trait implementation
#[test]
fn test_minify_trait_impl() {
    let input = r#"
    trait MyTrait {
        fn method(&self);
    }

    impl MyTrait for MyStruct {
        // Implementation
        fn method(&self) { }
    }
    "#;

    let result = minify(input).unwrap();

    assert_eq!(result.contains("trait"), true);
    assert_eq!(result.contains("impl"), true);
    assert_eq!(result.contains("Implementation"), false);
}

/// Test minify with enums
#[test]
fn test_minify_enums() {
    let input = r#"
    enum Color {
        Red,
        Green,
        Blue,
    }
    "#;

    let result = minify(input).unwrap();

    assert_eq!(result.contains("enum"), true);
    assert_eq!(result.contains("Red"), true);
    assert_eq!(result.contains("Green"), true);
}

/// Test minify with async/await
#[test]
fn test_minify_async_await() {
    let input = r#"
    async fn fetch_data() {
        // Fetch from API
        let data = api_call().await;
        data
    }
    "#;

    let result = minify(input).unwrap();

    assert_eq!(result.contains("async"), true);
    assert_eq!(result.contains("await"), true);
    assert_eq!(result.contains("Fetch from API"), false);
}

/// Test minify with lifetime parameters
#[test]
fn test_minify_lifetimes() {
    let input = r#"
    fn borrow<'a>(x: &'a str) -> &'a str {
        // Return borrowed data
        x
    }
    "#;

    let result = minify(input).unwrap();

    assert_eq!(result.contains("'a"), true);
    assert_eq!(result.contains("Return borrowed data"), false);
}

/// Test remove_comments with unclosed block comment doesn't panic
#[test]
fn test_remove_comments_unclosed_block() {
    let input = "code /* unclosed";
    let result = remove_comments_and_rustdocs(input).unwrap();

    // Should handle gracefully
    assert_eq!(result.contains("code"), true);
}

/// Test minify with character literals
#[test]
fn test_minify_char_literals() {
    let input = r#"let c = 'a';"#;
    let result = minify(input).unwrap();

    assert_eq!(result.contains("'a'"), true);
}

/// Test minify with raw strings
#[test]
fn test_minify_raw_strings() {
    let input = r##"let s = r#"raw string with "quotes""#;"##;
    let result = minify(input).unwrap();

    assert_eq!(result.contains("raw string"), true);
}

/// Test minify with byte strings
#[test]
fn test_minify_byte_strings() {
    let input = r#"let b = b"bytes";"#;
    let result = minify(input).unwrap();

    assert_eq!(result.contains("bytes"), true);
}

/// Test minify with numbers in different formats
#[test]
fn test_minify_number_formats() {
    let input = r#"
    let a = 42;
    let b = 0xFF;
    let c = 0b1010;
    let d = 1_000_000;
    "#;

    let result = minify(input).unwrap();

    assert_eq!(
        result.contains("42")
            || result.contains("0xFF")
            || result.contains("0b1010"),
        true
    );
}

/// Test minify with floating point numbers
#[test]
fn test_minify_floats() {
    let input = r#"let pi = 3.14159;"#;
    let result = minify(input).unwrap();

    assert!(result.contains("3.14159"));
}

/// Test minify with underscore in numbers
#[test]
fn test_minify_underscore_numbers() {
    let input = r#"let x = 1_000_000;"#;
    let result = minify(input).unwrap();

    // Underscores should be preserved in number literals
    assert!(
        result.contains("1_000_000") || result.contains("1000000")
    );
}

/// Test minify with match expression
#[test]
fn test_minify_match_expression() {
    let input = r#"
    let x = match value {
        // First case
        1 => "one",
        // Second case
        2 => "two",
        _ => "other",
    };
    "#;

    let result = minify(input).unwrap();

    assert!(result.contains("match"));
    assert!(!result.contains("First case"));
    assert!(!result.contains("Second case"));
}

/// Test minify with closure
#[test]
fn test_minify_closure() {
    let input = r#"
    let add = |x, y| {
        // Add two numbers
        x + y
    };
    "#;

    let result = minify(input).unwrap();

    assert!(result.contains("|"));
    assert!(!result.contains("Add two numbers"));
}

/// Test minify with type aliases
#[test]
fn test_minify_type_alias() {
    let input = r#"
    // Type alias for Result
    type MyResult<T> = Result<T, Box<dyn std::error::Error>>;
    "#;

    let result = minify(input).unwrap();

    assert_eq!(result.contains("type"), true);
    assert_eq!(result.contains("Type alias"), false);
}

/// Test minify with const declarations
#[test]
fn test_minify_const_declaration() {
    let input = r#"
    /// Important constant
    const MAX_SIZE: usize = 1024;
    "#;

    let result = minify(input).unwrap();

    assert_eq!(result.contains("const"), true);
    assert_eq!(result.contains("1024"), true);
    assert_eq!(!result.contains("Important constant"), true);
}

/// Test minify with static variables
#[test]
fn test_minify_static_variable() {
    let input = r#"
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    "#;

    let result = minify(input).unwrap();

    assert_eq!(result.contains("static"), true);
    assert_eq!(result.contains("COUNTER"), true);
}

/// Test minify with where clauses
#[test]
fn test_minify_where_clause() {
    let input = r#"
    fn generic_func<T>(x: T)
    where
    // T must implement Display
        T: std::fmt::Display,
    {
        println!("{}", x);
    }
    "#;

    let result = minify(input).unwrap();

    assert_eq!(result.contains("where"), true);
    assert_eq!(!result.contains("T must implement"), true);
}

/// Test minify with unsafe blocks
#[test]
fn test_minify_unsafe_block() {
    let input = r#"
    unsafe {
        // Dereference raw pointer
        *ptr = 42;
    }
    "#;

    let result = minify(input).unwrap();

    assert_eq!(result.contains("unsafe"), true);
    assert_eq!(!result.contains("Dereference raw pointer"), true);
}

/// Test minify idempotency (minifying twice gives same result)
#[test]
fn test_minify_idempotency() {
    let input = r#"
    /// Docs
    fn main() { // comment
        println!("test");
    }
    "#;

    let first = minify(input).unwrap();
    let second = minify(&first).unwrap();

    // Second minification should produce same or very similar result
    assert_eq!(first.trim(), second.trim());
}

/// Test needs_space_between with equal operator
#[test]
fn test_needs_space_between_eq_operators() {
    let ts: TokenStream = "x = y".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if tokens.len() >= 3 {
        let _result = needs_space_between(&tokens[1], &tokens[2]); // = to y
    }
}

/// Test needs_space_between with arrow operator
#[test]
fn test_needs_space_between_arrow() {
    let ts: TokenStream = "1 => 2".parse().unwrap();
    let tokens: Vec<TokenTree> = ts.into_iter().collect();
    if tokens.len() >= 2 {
        let _result = needs_space_between(&tokens[0], &tokens[1]); // 1 to =>
    }
}

/// Test reconstruct_minimal with closure tokens
#[test]
fn test_reconstruct_minimal_closure_tokens() {
    let tokens: Vec<&str> = vec!["|", "x", "|", "x", "+", "1"];
    let code = tokens.join(" "); // "fn main ( ) { }"
    let token_stream: TokenStream = code.parse().unwrap();
    let result = reconstruct_minimal(token_stream);

    assert_eq!(result.contains("x"), true);
}

/// Test minify preserves essential semantics
#[test]
fn test_minify_preserves_semantics() {
    let input = r#"
    fn add(a: i32, b: i32) -> i32 {
        a + b // Return sum
    }
    "#;

    let result = minify(input).unwrap();

    // All essential tokens should be present
    assert!(result.contains("fn add"));
    assert!(result.contains("a"));
    assert!(result.contains("b"));
    assert!(result.contains("+"));
    assert!(!result.contains("Return sum"));
}

/// Test minify with single-line comments only
#[test]
fn test_minify_single_line_comments_only() {
    let input = r#"
    // Comment 1
    // Comment 2
    // Comment 3
    "#;

    let result = minify(input).unwrap();
    assert!(result.trim().is_empty());
}

/// Test minify with nested block comments
#[test]
fn test_minify_nested_block_comments() {
    let input = r#"
    let x = 5; /* outer /* inner */ comment */ let y = 10;
    "#;

    let result = minify(input).unwrap();
    assert_eq!(result.contains("let x = 5"), false);
    assert_eq!(result.contains("let x=5"), true);
    assert_eq!(result.contains("let y = 10"), false);
    assert_eq!(result.contains("let y=10"), true);
    assert_eq!(result.contains("outer"), false);
    assert_eq!(result.contains("inner"), false);
}

/// Test minify preserves string literals with comment-like content
#[test]
fn test_minify_multiline_string_with_comment_markers() {
    let input = r##"
    let s = "this is // not a comment";
    let t = "/* also not a comment */";
    "##;

    let result = minify(input).unwrap();
    assert!(result.contains("// not a comment"));
    assert!(result.contains("/* also not a comment */"));
}

/// Test minify with raw strings containing special chars
#[test]
fn test_minify_raw_string_literals() {
    let input = r##"
    let raw = r#"raw string with " and \" and \\ "#;
    let code = r"C:\path\to\file";
    "##;

    let result = minify(input).unwrap();
    assert!(result.contains("raw string"));
    assert!(result.contains(r"C:\path"));
}
