//! Unit tests for the CSS/SCSS/LESS minifier.

use super::*;

fn minify(source: &str, dialect: CssDialect) -> String {
    minify_css(source, dialect, &MinifierOptions::default())
}

#[test]
fn removes_css_comments() {
    let source = "/* header */ .a { color: red; }";
    assert_eq!(minify(source, CssDialect::Css), ".a{color:red}");
}

#[test]
fn keeps_descendant_selector_space() {
    let source = "div p { color: red; }";
    assert_eq!(minify(source, CssDialect::Css), "div p{color:red}");
}

#[test]
fn keeps_space_before_pseudo_class() {
    let source = "a :hover { color: red; }";
    assert_eq!(minify(source, CssDialect::Css), "a :hover{color:red}");
}

#[test]
fn removes_space_inside_pseudo_class() {
    let source = "a:hover { color: red; }";
    assert_eq!(minify(source, CssDialect::Css), "a:hover{color:red}");
}

#[test]
fn preserves_calc_operator_spaces() {
    let source = ".a { width: calc(100% - 10px); }";
    assert_eq!(
        minify(source, CssDialect::Css),
        ".a{width:calc(100% - 10px)}"
    );
}

#[test]
fn preserves_unquoted_url_contents() {
    let source = ".a { background: url(http://example.com/a.png); }";
    assert_eq!(
        minify(source, CssDialect::Css),
        ".a{background:url(http://example.com/a.png)}"
    );
}

#[test]
fn preserves_comment_markers_inside_strings() {
    let source = ".a { content: \"/* not a comment */\"; }";
    assert_eq!(
        minify(source, CssDialect::Css),
        ".a{content:\"/* not a comment */\"}"
    );
}

#[test]
fn removes_scss_line_and_block_comments() {
    let source = "// header\n/* block */\n.a { color: red; }";
    assert_eq!(minify(source, CssDialect::Scss), ".a{color:red}");
}

#[test]
fn handles_scss_variables() {
    let source = "$color: red;\n.a { color: $color; }";
    assert_eq!(
        minify(source, CssDialect::Scss),
        "$color:red;.a{color:$color}"
    );
}

#[test]
fn handles_scss_nesting() {
    let source = ".a { .b { color: red; } }";
    assert_eq!(minify(source, CssDialect::Scss), ".a{.b{color:red}}");
}

#[test]
fn handles_less_variables() {
    let source = "@color: red;\n.a { color: @color; }";
    assert_eq!(
        minify(source, CssDialect::Less),
        "@color:red;.a{color:@color}"
    );
}

#[test]
fn keeps_media_query_logic_spaces() {
    let source = "@media screen and (min-width: 600px) { .a { display: none; } }";
    assert_eq!(
        minify(source, CssDialect::Css),
        "@media screen and (min-width:600px){.a{display:none}}"
    );
}

#[test]
fn removes_space_before_important() {
    let source = ".a { color: red !important; }";
    assert_eq!(minify(source, CssDialect::Css), ".a{color:red!important}");
}

#[test]
fn handles_attribute_selectors() {
    let source = "a[data-x = \"y\"] { color: red; }";
    assert_eq!(
        minify(source, CssDialect::Css),
        "a[data-x=\"y\"]{color:red}"
    );
}

#[test]
fn keeps_descendant_space_after_attribute_selector() {
    let source = "a[href] .b { color: red; }";
    assert_eq!(
        minify(source, CssDialect::Css),
        "a[href] .b{color:red}"
    );
}

#[test]
fn keeps_space_after_pseudo_function() {
    let source = ":not(.a) .b { color: red; }";
    assert_eq!(
        minify(source, CssDialect::Css),
        ":not(.a) .b{color:red}"
    );
}

#[test]
fn keeps_selector_combinator_spaces() {
    let source = "a + b ~ c > d { color: red; }";
    assert_eq!(
        minify(source, CssDialect::Css),
        "a + b ~ c > d{color:red}"
    );
}

#[test]
fn removes_space_around_colon_in_declaration() {
    let source = ".a { color : red ; }";
    assert_eq!(minify(source, CssDialect::Css), ".a{color:red}");
}

#[test]
fn preserves_line_breaks_when_requested() {
    let source = ".a { color: red; }\n.b { color: blue; }";
    let result = minify_css(
        source,
        CssDialect::Css,
        &MinifierOptions::line_preserving(),
    );
    assert_eq!(result, ".a{color:red}\n.b{color:blue}");
}

#[test]
fn returns_empty_for_empty_source() {
    assert_eq!(minify("  \n\t", CssDialect::Css), "");
}

#[test]
fn removes_bom() {
    let source = "\u{feff}.a { color: red; }";
    assert_eq!(minify(source, CssDialect::Css), ".a{color:red}");
}

#[test]
fn keeps_custom_property_names() {
    let source = ":root { --main-color: red; }";
    assert_eq!(
        minify(source, CssDialect::Css),
        ":root{--main-color:red}"
    );
}
