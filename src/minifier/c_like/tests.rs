//! Unit tests for the configurable C-like minifier.

use super::*;
use crate::minifier::c_like::profile;

fn minify(
    source: &str,
    profile: &'static CLikeProfile,
) -> String {
    minify_c_like(source, profile, &MinifierOptions::default())
}

#[test]
fn removes_javascript_line_and_block_comments() {
    let source = "// header\nconst x = 1; /* inline */\n";
    assert_eq!(minify(source, &profile::JAVASCRIPT), "const x=1;");
}

#[test]
fn removes_comments_before_joining_lines() {
    let source = "const a = 1;\n// between\nconst b = 2;\n";
    assert_eq!(
        minify(source, &profile::JAVASCRIPT),
        "const a=1;const b=2;"
    );
}

#[test]
fn inserts_semicolon_for_javascript_asi() {
    let source = "const a = 1\nconst b = 2\n";
    assert_eq!(
        minify(source, &profile::JAVASCRIPT),
        "const a=1;const b=2"
    );
}

#[test]
fn inserts_semicolon_after_return_for_javascript_asi() {
    let source = "function f() {\n  return\n  5\n}\n";
    assert_eq!(
        minify(source, &profile::JAVASCRIPT),
        "function f(){return;5}"
    );
}

#[test]
fn does_not_insert_semicolon_before_binary_operator() {
    let source = "let a = 1\n+ 2\n";
    assert_eq!(minify(source, &profile::JAVASCRIPT), "let a=1+2");
}

#[test]
fn does_not_insert_semicolon_before_call_parenthesis() {
    let source = "foo\n(bar)\n";
    assert_eq!(minify(source, &profile::JAVASCRIPT), "foo(bar)");
}

#[test]
fn inserts_semicolon_before_increment_on_new_line() {
    let source = "a\n++b\n";
    assert_eq!(minify(source, &profile::JAVASCRIPT), "a;++b");
}

#[test]
fn keeps_comment_markers_inside_javascript_strings() {
    let source = "let s = \"// not a comment\"; // real\n";
    assert_eq!(
        minify(source, &profile::JAVASCRIPT),
        "let s=\"// not a comment\";"
    );
}

#[test]
fn keeps_comment_markers_inside_javascript_regex() {
    let source = "const re = /a\\/\\/b/g; // real\n";
    assert_eq!(
        minify(source, &profile::JAVASCRIPT),
        "const re=/a\\/\\/b/g;"
    );
}

#[test]
fn keeps_javascript_template_literal_newlines() {
    let source = "const text = `a\nb`;\n";
    assert_eq!(
        minify(source, &profile::JAVASCRIPT),
        "const text=`a\nb`;"
    );
}

#[test]
fn removes_jsx_comment_containers() {
    let source = "const view = <div>{/* hello */}</div>;\n";
    assert_eq!(
        minify(source, &profile::JAVASCRIPT),
        "const view=<div>{}</div>;"
    );
}

#[test]
fn joins_java_lines_without_adding_semicolons() {
    let source = "class A {\n  int x = 1;\n}\n";
    assert_eq!(minify(source, &profile::JAVA), "class A{int x=1;}");
}

#[test]
fn preserves_java_text_blocks() {
    let source = "String s = \"\"\"\nhello\n\"\"\";\n";
    assert_eq!(
        minify(source, &profile::JAVA),
        "String s=\"\"\"\nhello\n\"\"\";"
    );
}

#[test]
fn inserts_semicolon_for_kotlin_newline_statements() {
    let source = "val a = 1\nval b = 2\n";
    assert_eq!(minify(source, &profile::KOTLIN), "val a=1;val b=2");
}

#[test]
fn does_not_insert_semicolon_before_kotlin_else() {
    let source = "if (x) {\n  a()\n}\nelse {\n  b()\n}\n";
    assert_eq!(
        minify(source, &profile::KOTLIN),
        "if(x){a()}else{b()}"
    );
}

#[test]
fn removes_nested_kotlin_block_comments() {
    let source = "/* outer /* inner */ still */ val x = 1\n";
    assert_eq!(minify(source, &profile::KOTLIN), "val x=1");
}

#[test]
fn keeps_preprocessor_lines_on_their_own_line() {
    let source = "#include <stdio.h>\nint main() { return 0; }\n";
    assert_eq!(
        minify(source, &profile::C),
        "#include <stdio.h>\nint main(){return 0;}"
    );
}

#[test]
fn keeps_csharp_verbatim_strings() {
    let source = "var s = @\"C:\\path\"; // c\n";
    assert_eq!(
        minify(source, &profile::CSHARP),
        "var s=@\"C:\\path\";"
    );
}

#[test]
fn keeps_lua_long_bracket_strings() {
    let source = "--[[ block\ncomment ]]\nlocal s = [[a\nb]]\n";
    assert_eq!(minify(source, &profile::LUA), "local s=[[a\nb]]");
}

#[test]
fn keeps_go_raw_strings() {
    let source = "var s = `a\nb`\n";
    assert_eq!(minify(source, &profile::GO), "var s=`a\nb`");
}

#[test]
fn separates_dangerous_adjacent_operators() {
    let source = "let a = x - -y;\n";
    assert_eq!(minify(source, &profile::JAVASCRIPT), "let a=x- -y;");
}

#[test]
fn keeps_space_between_word_and_string() {
    let source = "return \"x\";\n";
    assert_eq!(
        minify(source, &profile::JAVASCRIPT),
        "return \"x\";"
    );
}

#[test]
fn preserves_line_breaks_when_requested() {
    let source = "const a = 1\nconst b = 2\n";
    let result = minify_c_like(
        source,
        &profile::JAVASCRIPT,
        &MinifierOptions::line_preserving(),
    );
    assert_eq!(result, "const a=1\nconst b=2");
}

#[test]
fn returns_empty_for_empty_source() {
    assert_eq!(minify("   \n\t\n", &profile::JAVASCRIPT), "");
}

#[test]
fn removes_bom_before_lexing() {
    let source = "\u{feff}

const x = 1;\n";
    assert_eq!(minify(source, &profile::JAVASCRIPT), "const x=1;");
}

#[test]
fn handles_swift_raw_hash_strings() {
    let source = "let s = #\"a // b\"# // real\n";
    assert_eq!(
        minify(source, &profile::SWIFT),
        "let s=#\"a // b\"#"
    );
}

#[test]
fn handles_dart_raw_strings() {
    let source = "var s = r\"a // b\"; // real\n";
    assert_eq!(minify(source, &profile::DART), "var s=r\"a // b\";");
}
