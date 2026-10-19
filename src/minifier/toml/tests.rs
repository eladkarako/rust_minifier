use super::*;

#[test]
fn test_minify_removes_comments() {
    let minifier = TomlMinifier;
    let source = "name = \"test\" # this is a comment";
    let result = minifier.minify(source).unwrap();
    assert!(!result.contains("#"));
    assert!(result.contains("name"));
}

#[test]
fn test_minify_removes_empty_lines() {
    let minifier = TomlMinifier;
    let source = "name = \"test\"\n\nversion = \"1.0\"";
    let result = minifier.minify(source).unwrap();
    assert!(!result.contains("\n\n"));
}

#[test]
fn test_minify_normalizes_spacing() {
    let minifier = TomlMinifier;
    let source = "name   =   \"test\"";
    let result = minifier.minify(source).unwrap();
    assert!(result.contains("name="));
}

#[test]
fn test_minify_handles_strings_with_hashes() {
    let minifier = TomlMinifier;
    let source = "description = \"This is a #hashtag\"";
    let result = minifier.minify(source).unwrap();
    assert!(result.contains("#hashtag"));
}

#[test]
fn test_minify_complex_toml() {
    let minifier = TomlMinifier;
    let source = "[package]\nname = \"rust_minifier\" # package name\nversion = \"1.0.0\" # version";
    let result = minifier.minify(source).unwrap();
    assert!(result.contains("[package]"));
    assert!(!result.contains("#"));
}

#[test]
fn test_minify_empty_source() {
    let minifier = TomlMinifier;
    let result = minifier.minify("").unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_minify_array_formatting() {
    let minifier = TomlMinifier;
    let source = "items = [ 1 , 2 , 3 ]";
    let result = minifier.minify(source).unwrap();
    assert!(result.contains("["));
    assert!(result.contains("]"));
}
