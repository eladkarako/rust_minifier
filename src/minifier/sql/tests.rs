//! Unit tests for the SQL minifier.

use super::*;

fn minify(source: &str) -> String {
    minify_sql(source, &MinifierOptions::default())
}

#[test]
fn collapses_whitespace() {
    assert_eq!(
        minify("SELECT   *\n  FROM   users;"),
        "SELECT * FROM users;"
    );
}

#[test]
fn removes_line_comments() {
    assert_eq!(
        minify("-- header\nSELECT 1;"),
        "SELECT 1;"
    );
}

#[test]
fn removes_block_comments() {
    assert_eq!(
        minify("SELECT /* c */ 1;"),
        "SELECT 1;"
    );
}

#[test]
fn keeps_comment_markers_inside_strings() {
    assert_eq!(
        minify("SELECT '--not comment';"),
        "SELECT '--not comment';"
    );
}

#[test]
fn keeps_block_comment_markers_inside_strings() {
    assert_eq!(
        minify("SELECT '/* not */';"),
        "SELECT '/* not */';"
    );
}

#[test]
fn handles_doubled_quote_escape() {
    assert_eq!(
        minify("SELECT 'it''s';"),
        "SELECT 'it''s';"
    );
}

#[test]
fn removes_space_before_parenthesis() {
    assert_eq!(
        minify("CREATE TABLE t (id INT);"),
        "CREATE TABLE t(id INT);"
    );
}

#[test]
fn normalizes_insert_statement() {
    assert_eq!(
        minify("INSERT INTO t (a, b) VALUES (1, 2);"),
        "INSERT INTO t(a, b) VALUES(1, 2);"
    );
}

#[test]
fn keeps_quoted_identifiers() {
    assert_eq!(
        minify("SELECT \"col name\" FROM t;"),
        "SELECT \"col name\" FROM t;"
    );
}

#[test]
fn keeps_backtick_identifiers() {
    assert_eq!(
        minify("SELECT `a b` FROM t;"),
        "SELECT `a b` FROM t;"
    );
}

#[test]
fn keeps_bracket_identifiers() {
    assert_eq!(
        minify("SELECT [a b] FROM t;"),
        "SELECT [a b] FROM t;"
    );
}

#[test]
fn keeps_dollar_quoted_strings() {
    assert_eq!(
        minify("SELECT $$a--b$$;"),
        "SELECT $$a--b$$;"
    );
}

#[test]
fn removes_mysql_hash_comments() {
    assert_eq!(
        minify("# header\nSELECT 1;"),
        "SELECT 1;"
    );
}

#[test]
fn keeps_operator_separation() {
    assert_eq!(
        minify("SELECT a - -1;"),
        "SELECT a - -1;"
    );
}

#[test]
fn removes_spaces_around_commas() {
    assert_eq!(
        minify("SELECT a , b , c FROM t;"),
        "SELECT a, b, c FROM t;"
    );
}

#[test]
fn preserves_line_breaks_when_requested() {
    let result = minify_sql(
        "SELECT 1;\nSELECT 2;",
        &MinifierOptions::line_preserving(),
    );
    assert_eq!(result, "SELECT 1;\nSELECT 2;");
}

#[test]
fn returns_empty_for_empty_input() {
    assert_eq!(minify("  \n\t "), "");
}

#[test]
fn removes_bom() {
    assert_eq!(
        minify("\u{feff}SELECT 1;"),
        "SELECT 1;"
    );
}

#[test]
fn handles_cast_operator() {
    assert_eq!(
        minify("SELECT a::int FROM t;"),
        "SELECT a::int FROM t;"
    );
}

#[test]
fn keeps_dotted_names_compact() {
    assert_eq!(
        minify("SELECT t . id FROM schema . t;"),
        "SELECT t.id FROM schema.t;"
    );
}
