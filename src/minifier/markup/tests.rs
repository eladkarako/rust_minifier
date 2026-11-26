//! Unit tests for the markup minifier.

use super::*;

fn minify(source: &str) -> String {
    minify_markup(source, "html", &MinifierOptions::default())
}

#[test]
fn removes_html_comments() {
    assert_eq!(
        minify("<!-- header --><div>hello</div>"),
        "<div>hello</div>"
    );
}

#[test]
fn removing_comment_between_text_does_not_add_space() {
    assert_eq!(
        minify("<div>a<!-- hidden -->b</div>"),
        "<div>ab</div>"
    );
    assert_eq!(
        minify("<div>a <!-- hidden --> b</div>"),
        "<div>a b</div>"
    );
}

#[test]
fn collapses_text_whitespace() {
    assert_eq!(
        minify("<p>hello   world</p>"),
        "<p>hello world</p>"
    );
}

#[test]
fn collapses_newlines_between_tags() {
    assert_eq!(
        minify("<div>\n  <p>a</p>\n  <p>b</p>\n</div>"),
        "<div><p>a</p><p>b</p></div>"
    );
}

#[test]
fn keeps_space_between_inline_elements() {
    assert_eq!(
        minify("<span>a</span> <span>b</span>"),
        "<span>a</span> <span>b</span>"
    );
}

#[test]
fn removes_space_between_block_elements() {
    assert_eq!(
        minify("<div>a</div>\n<div>b</div>"),
        "<div>a</div><div>b</div>"
    );
}

#[test]
fn preserves_pre_content() {
    let source = "<pre>  a\n  b  </pre>";
    assert_eq!(minify(source), source);
}

#[test]
fn preserves_textarea_content() {
    let source = "<textarea>  keep\n  this  </textarea>";
    assert_eq!(minify(source), source);
}

#[test]
fn removes_comments_inside_pre() {
    assert_eq!(
        minify("<pre>a<!-- hidden -->b</pre>"),
        "<pre>ab</pre>"
    );
}

#[test]
fn minifies_inline_script() {
    assert_eq!(
        minify("<script>// c\nconst x = 1;</script>"),
        "<script>const x=1;</script>"
    );
}

#[test]
fn minifies_inline_style() {
    assert_eq!(
        minify("<style>/* c */ .a { color: red; }</style>"),
        "<style>.a{color:red}</style>"
    );
}

#[test]
fn minifies_script_json_payload() {
    assert_eq!(
        minify("<script type=\"application/ld+json\">{ \"a\": 1 }</script>"),
        "<script type=\"application/ld+json\">{\"a\":1}</script>"
    );
}

#[test]
fn minifies_typescript_script_block() {
    assert_eq!(
        minify("<script lang=\"ts\">// c\nconst x: number = 1;</script>"),
        "<script lang=\"ts\">const x:number=1;</script>"
    );
}

#[test]
fn minifies_scss_style_block() {
    assert_eq!(
        minify("<style lang=\"scss\">// c\n.a { .b { color: red; } }</style>"),
        "<style lang=\"scss\">.a{.b{color:red}}</style>"
    );
}

#[test]
fn keeps_doctype() {
    assert_eq!(
        minify("<!DOCTYPE html><html></html>"),
        "<!DOCTYPE html><html></html>"
    );
}

#[test]
fn normalizes_self_closing_tags() {
    assert_eq!(minify("<br />"), "<br/>");
    assert_eq!(minify("<input disabled />"), "<input disabled/>");
}

#[test]
fn normalizes_attribute_spacing() {
    assert_eq!(
        minify("<div  class = \"x\"  id = 'y' >"),
        "<div class=\"x\" id='y'>"
    );
}

#[test]
fn preserves_attribute_values_with_spaces() {
    assert_eq!(
        minify("<div class=\"a  b\" title=\"a = b\"></div>"),
        "<div class=\"a  b\" title=\"a = b\"></div>"
    );
}

#[test]
fn preserves_template_tags() {
    assert_eq!(
        minify("<div>{{ x }}</div>"),
        "<div>{{ x }}</div>"
    );
}

#[test]
fn preserves_twig_tags() {
    assert_eq!(
        minify("<div>{% if x %}a{% endif %}</div>"),
        "<div>{% if x %}a{% endif %}</div>"
    );
}

#[test]
fn removes_twig_comments() {
    assert_eq!(
        minify("<div>{# hidden #}a</div>"),
        "<div>a</div>"
    );
}

#[test]
fn removes_handlebars_comments() {
    assert_eq!(
        minify("<div>{{!-- hidden --}}a</div>"),
        "<div>a</div>"
    );
}

#[test]
fn removes_ejs_comments() {
    assert_eq!(
        minify("<div><%# hidden %>a</div>"),
        "<div>a</div>"
    );
}

#[test]
fn preserves_ejs_code() {
    assert_eq!(
        minify("<div><%= name %></div>"),
        "<div><%= name %></div>"
    );
}

#[test]
fn preserves_razor_comments_removed() {
    assert_eq!(
        minify("<div>@* hidden *@a</div>"),
        "<div>a</div>"
    );
}

#[test]
fn preserves_xml_case() {
    let source = "<LinearLayout><TextView /></LinearLayout>";
    assert_eq!(
        minify(source),
        "<LinearLayout><TextView/></LinearLayout>"
    );
}

#[test]
fn preserves_cdata() {
    let source = "<root><![CDATA[a < b && c]]></root>";
    assert_eq!(minify(source), source);
}

#[test]
fn preserves_conditional_comments() {
    let source = "<!--[if IE]><p>x</p><![endif]-->";
    assert_eq!(minify(source), source);
}

#[test]
fn preserves_processing_instructions() {
    let source = "<?xml version=\"1.0\"?><root></root>";
    assert_eq!(minify(source), source);
}

#[test]
fn handles_astro_frontmatter() {
    let source = "---\nconst x = 1;\n---\n<div>{x}</div>";
    assert_eq!(
        minify_markup(source, "astro", &MinifierOptions::default()),
        "---\nconst x = 1;\n---\n<div>{x}</div>"
    );
}

#[test]
fn handles_vue_component() {
    let source = "<template>\n  <div>{{ msg }}</div>\n</template>\n<script>// c\nexport default {}</script>";
    assert_eq!(
        minify_markup(source, "vue", &MinifierOptions::default()),
        "<template><div>{{ msg }}</div></template><script>export default{}</script>"
    );
}

#[test]
fn keeps_text_next_to_inline_tags() {
    assert_eq!(
        minify("<p>Hello <b>world</b>!</p>"),
        "<p>Hello <b>world</b>!</p>"
    );
}

#[test]
fn removes_bom() {
    assert_eq!(
        minify("\u{feff}<div>a</div>"),
        "<div>a</div>"
    );
}

#[test]
fn returns_empty_for_empty_input() {
    assert_eq!(minify("   \n\t "), "");
}

#[test]
fn handles_script_without_closing_tag() {
    assert_eq!(
        minify("<script>// c\nconst x = 1;"),
        "<script>const x=1;"
    );
}

#[test]
fn keeps_script_string_containing_tag() {
    assert_eq!(
        minify("<script>const s = \"</div>\";</script>"),
        "<script>const s=\"</div>\";</script>"
    );
}

#[test]
fn svg_style_is_minified() {
    assert_eq!(
        minify("<svg><style>.a { fill: red; }</style></svg>"),
        "<svg><style>.a{fill:red}</style></svg>"
    );
}
