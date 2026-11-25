//! Unit tests for the GraphQL minifier.

use super::*;

fn minify(source: &str) -> String {
    minify_graphql(source, &MinifierOptions::default())
}

#[test]
fn removes_comments() {
    assert_eq!(
        minify("# header\nquery { a }"),
        "query{a}"
    );
}

#[test]
fn keeps_word_boundaries() {
    assert_eq!(
        minify("query Foo {\n  bar\n}"),
        "query Foo{bar}"
    );
}

#[test]
fn handles_fragment_spread() {
    assert_eq!(
        minify("fragment F on T { ...G }"),
        "fragment F on T{...G}"
    );
}

#[test]
fn handles_inline_fragment() {
    assert_eq!(
        minify("{ ... on Type { field } }"),
        "{...on Type{field}}"
    );
}

#[test]
fn handles_directives() {
    assert_eq!(
        minify("query { f @include(if: $x) }"),
        "query{f@include(if:$x)}"
    );
}

#[test]
fn removes_space_around_colon() {
    assert_eq!(
        minify("f ( a : 1 )"),
        "f(a:1)"
    );
}

#[test]
fn keeps_comment_markers_inside_strings() {
    assert_eq!(
        minify("query { f(a: \"x # y\") }"),
        "query{f(a:\"x # y\")}"
    );
}

#[test]
fn keeps_escaped_quotes_inside_strings() {
    assert_eq!(
        minify("f(a: \"a\\\"b\")"),
        "f(a:\"a\\\"b\")"
    );
}

#[test]
fn preserves_block_strings() {
    let source = "type X {\n  f: String\n}\n\"\"\"\na\n# not a comment\n\"\"\"";
    let result = minify(source);
    assert!(result.contains("\"\"\"\na\n# not a comment\n\"\"\""));
}

#[test]
fn preserves_line_breaks_when_requested() {
    let result = minify_graphql(
        "query {\n  a\n}",
        &MinifierOptions::line_preserving(),
    );
    assert_eq!(result, "query{\na\n}");
}

#[test]
fn returns_empty_for_empty_input() {
    assert_eq!(minify("  \n\t "), "");
}

#[test]
fn removes_bom() {
    assert_eq!(
        minify("\u{feff}query { a }"),
        "query{a}"
    );
}

#[test]
fn keeps_arguments_compact() {
    assert_eq!(
        minify("f(a: 1, b: 2)"),
        "f(a:1,b:2)"
    );
}

#[test]
fn keeps_list_and_object_values() {
    assert_eq!(
        minify("f(a: [1, 2], b: { c: 3 })"),
        "f(a:[1,2],b:{c:3})"
    );
}
