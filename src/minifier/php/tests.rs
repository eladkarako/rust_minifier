//! Unit tests for the PHP mixed HTML/PHP minifier.

use super::*;

fn minify(source: &str) -> String {
    minify_php(source, &MinifierOptions::default())
}

#[test]
fn html_only_uses_markup_rules() {
    assert_eq!(minify("<div>  hi  </div>"), "<div>hi</div>");
}

#[test]
fn removes_php_line_comment() {
    assert_eq!(
        minify("<?php // c\necho 1; ?>"),
        "<?php echo 1;?>"
    );
}

#[test]
fn removes_php_block_comment() {
    assert_eq!(
        minify("<?php /* c */ echo 1; ?>"),
        "<?php echo 1;?>"
    );
}

#[test]
fn mixed_html_and_php() {
    assert_eq!(
        minify("<div><?php echo $x; ?></div>"),
        "<div><?php echo $x;?></div>"
    );
}

#[test]
fn keeps_close_tag_inside_php_string() {
    assert_eq!(
        minify("<?php $s = \"?>\"; echo $s; ?>"),
        "<?php $s=\"?>\";echo $s;?>"
    );
}

#[test]
fn short_echo_tag() {
    assert_eq!(minify("<?= $x ?>"), "<?= $x?>");
}

#[test]
fn removes_html_comment_around_php() {
    assert_eq!(
        minify("<!-- c --><?php echo 1;?>"),
        "<?php echo 1;?>"
    );
}

#[test]
fn unclosed_php_block() {
    assert_eq!(minify("<?php echo 1;"), "<?php echo 1;");
}

#[test]
fn keeps_space_between_text_and_php() {
    assert_eq!(
        minify("<p>Hello <?php echo $name; ?>!</p>"),
        "<p>Hello <?php echo $name;?>!</p>"
    );
}

#[test]
fn multiple_php_blocks() {
    assert_eq!(
        minify("<?php echo 1; ?><?php echo 2; ?>"),
        "<?php echo 1;?><?php echo 2;?>"
    );
}

#[test]
fn keeps_php8_attribute() {
    let result =
        minify("<?php #[Route(\"/\")] function f() {} ?>");
    assert!(result.contains("#[Route"));
    assert!(!result.contains("//"));
}

#[test]
fn php_string_with_comment_markers() {
    assert_eq!(
        minify("<?php echo \"// not a comment\"; ?>"),
        "<?php echo \"// not a comment\";?>"
    );
}

#[test]
fn empty_input_returns_empty() {
    assert_eq!(minify("  \n\t "), "");
}

#[test]
fn removes_bom() {
    assert_eq!(
        minify("\u{feff}<?php echo 1;?>"),
        "<?php echo 1;?>"
    );
}

#[test]
fn php_in_attribute_context() {
    assert_eq!(
        minify("<a href=\"<?php echo $url; ?>\">x</a>"),
        "<a href=\"<?php echo $url;?>\">x</a>"
    );
}
