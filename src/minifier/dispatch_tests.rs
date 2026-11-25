//! Tests that the public dispatcher routes each language to its engine.

use super::*;
use std::path::Path;

fn minify_path(path: &str, source: &str) -> String {
    minify_with_options(
        source,
        Path::new(path),
        &MinifierOptions::default(),
    )
    .unwrap()
}

#[test]
fn dispatches_rust() {
    assert_eq!(minify_path("main.rs", "fn main() { }"), "fn main(){}");
}

#[test]
fn dispatches_toml() {
    assert_eq!(minify_path("Cargo.toml", "a = 1 # c"), "a=1");
}

#[test]
fn dispatches_html() {
    assert_eq!(
        minify_path("index.html", "<!-- c --><p>  hi  </p>"),
        "<p>hi</p>"
    );
}

#[test]
fn dispatches_javascript() {
    assert_eq!(
        minify_path("app.js", "// c\nconst x = 1;"),
        "const x=1;"
    );
}

#[test]
fn dispatches_kotlin() {
    assert_eq!(
        minify_path("Main.kt", "val a = 1\nval b = 2\n"),
        "val a=1;val b=2"
    );
}

#[test]
fn dispatches_css() {
    assert_eq!(
        minify_path("style.css", "/* c */ .a { color: red; }"),
        ".a{color:red}"
    );
}

#[test]
fn dispatches_json() {
    assert_eq!(
        minify_path("package.json", "{ \"a\": 1 }"),
        "{\"a\":1}"
    );
}

#[test]
fn dispatches_sql() {
    assert_eq!(
        minify_path("schema.sql", "SELECT  1;"),
        "SELECT 1;"
    );
}

#[test]
fn dispatches_graphql() {
    assert_eq!(
        minify_path("schema.graphql", "query { a }"),
        "query{a}"
    );
}

#[test]
fn dispatches_yaml() {
    assert_eq!(
        minify_path("ci.yml", "# c\nkey: value\n"),
        "key: value"
    );
}

#[test]
fn dispatches_php() {
    assert_eq!(
        minify_path("index.php", "<?php // c\necho 1; ?>"),
        "<?php echo 1;?>"
    );
}

#[test]
fn dispatches_xml() {
    assert_eq!(
        minify_path("AndroidManifest.xml", "<root>  <a/>  </root>"),
        "<root><a/></root>"
    );
}

#[test]
fn dispatches_scss() {
    assert_eq!(
        minify_path("style.scss", "// c\n.a { .b { color: red; } }"),
        ".a{.b{color:red}}"
    );
}

#[test]
fn dispatches_wat() {
    assert_eq!(
        minify_path("module.wat", ";; c\n(module)\n"),
        "(module)"
    );
}

#[test]
fn dispatches_dockerfile() {
    assert_eq!(
        minify_path("Dockerfile", "# c\nFROM alpine\n"),
        "FROM alpine"
    );
}

#[test]
fn rejects_unsupported_type() {
    let result = minify_with_options(
        "print('hi')",
        Path::new("script.py"),
        &MinifierOptions::default(),
    );
    assert!(result.is_err());
}

#[test]
fn get_minifier_uses_file_name() {
    assert!(get_minifier(Path::new("Makefile")).is_ok());
    assert!(get_minifier(Path::new(".gitignore")).is_ok());
}
