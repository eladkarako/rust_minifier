//! Tests for file processing and minification orchestration.

use super::*;
use std::fs;
use temp_dir::TempDir;

/// Test that language categories give a deterministic sort order.
#[test]
fn test_language_category_ordering() {
    let rust = language::find_language("rust").unwrap();
    let toml = language::find_language("toml").unwrap();
    // Rust files are written before config files in concatenated output.
    assert!(rust.category < toml.category);
}

/// Test language detection for a TOML path.
#[test]
fn test_detect_language_toml() {
    let language =
        language::detect_language(Path::new("config.toml")).unwrap();
    assert_eq!(language.id, "toml");
}

/// Test language detection for a Rust path.
#[test]
fn test_detect_language_rust() {
    let language =
        language::detect_language(Path::new("main.rs")).unwrap();
    assert_eq!(language.id, "rust");
}

/// Test that unknown extensions are not detected.
#[test]
fn test_detect_language_unknown() {
    assert!(
        language::detect_language(Path::new("image.png")).is_none()
    );
}

/// Test expanding a single .rs file.
#[test]
fn test_expand_path_single_rust_file() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    let mut expanded = Vec::new();
    expand_path(
        test_file.to_str().unwrap(),
        &mut expanded,
        &[],
        false,
    )
        .unwrap();

    assert_eq!(expanded.len(), 1);
    assert!(expanded[0].ends_with(".rs"));
}

/// Test expanding a single .toml file.
#[test]
fn test_expand_path_single_toml_file() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("Cargo.toml");
    fs::write(&test_file, "[package]\n").unwrap();

    let mut expanded = Vec::new();
    expand_path(
        test_file.to_str().unwrap(),
        &mut expanded,
        &[],
        false,
    )
        .unwrap();

    assert_eq!(expanded.len(), 1);
    assert!(expanded[0].ends_with(".toml"));
}

/// Test expanding a single .html file.
#[test]
fn test_expand_path_single_html_file() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("index.html");
    fs::write(&test_file, "<p>hi</p>").unwrap();

    let mut expanded = Vec::new();
    expand_path(
        test_file.to_str().unwrap(),
        &mut expanded,
        &[],
        false,
    )
        .unwrap();

    assert_eq!(expanded.len(), 1);
    assert!(expanded[0].ends_with(".html"));
}

/// Test that unsupported binary-looking files are ignored.
#[test]
fn test_expand_path_ignores_unsupported_file() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("image.png");
    fs::write(&test_file, [0u8, 1, 2, 3]).unwrap();

    let mut expanded = Vec::new();
    expand_path(
        test_file.to_str().unwrap(),
        &mut expanded,
        &[],
        false,
    )
        .unwrap();

    assert!(expanded.is_empty());
}

/// Test that a forced language accepts an otherwise unsupported extension.
#[test]
fn test_expand_path_forced_type_accepts_any_file() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("template.txt");
    fs::write(&test_file, "<p>hi</p>").unwrap();

    let mut expanded = Vec::new();
    expand_path(
        test_file.to_str().unwrap(),
        &mut expanded,
        &[],
        true,
    )
        .unwrap();

    assert_eq!(expanded.len(), 1);
}

/// Test expanding a nonexistent file returns error.
#[test]
fn test_expand_path_nonexistent_file() {
    let mut expanded = Vec::new();
    let result = expand_path(
        "/nonexistent/file.rs",
        &mut expanded,
        &[],
        false,
    );
    assert!(result.is_err());
}

/// Test expanding a directory discovers Rust files.
#[test]
fn test_expand_path_directory() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("main.rs"), "fn main() {}")
        .unwrap();
    fs::write(temp_dir.path().join("lib.rs"), "pub fn test() {}")
        .unwrap();

    let mut expanded = Vec::new();
    expand_path(
        temp_dir.path().to_str().unwrap(),
        &mut expanded,
        &[],
        false,
    )
        .unwrap();

    assert_eq!(expanded.len(), 2);
}

/// Test should_exclude_file with matching pattern.
#[test]
fn test_should_exclude_file_match() {
    let patterns = vec!["test".to_string()];
    assert!(should_exclude_file("test_helper.rs", &patterns));
}

/// Test should_exclude_file with non-matching pattern.
#[test]
fn test_should_exclude_file_no_match() {
    let patterns = vec!["test".to_string()];
    assert!(!should_exclude_file("main.rs", &patterns));
}

/// Test should_exclude_file with case-insensitive matching.
#[test]
fn test_should_exclude_file_case_insensitive() {
    let patterns = vec!["TEST".to_string()];
    assert!(should_exclude_file("test_helper.rs", &patterns));
}

/// Test should_exclude_file with multiple patterns.
#[test]
fn test_should_exclude_file_multiple_patterns() {
    let patterns = vec![
        "test".to_string(),
        "mock".to_string(),
        "temp".to_string(),
    ];
    assert!(should_exclude_file("mock_data.rs", &patterns));
    assert!(should_exclude_file("temp_file.rs", &patterns));
    assert!(!should_exclude_file("main.rs", &patterns));
}

/// Test should_exclude_file with substring match.
#[test]
fn test_should_exclude_file_substring_match() {
    let patterns = vec!["barhel".to_string()];
    assert!(should_exclude_file("foobarhelloworld.rs", &patterns));
}

/// Test should_exclude_file ignores directory paths.
#[test]
fn test_should_exclude_file_filename_only() {
    let patterns = vec!["src".to_string()];
    // Should match "src" in filename, not in path
    assert!(should_exclude_file("/project/src_test.rs", &patterns));
    assert!(!should_exclude_file("/project/src/main.rs", &patterns));
}

/// Test expand_path respects exclude patterns.
#[test]
fn test_expand_path_with_exclude() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("main.rs"), "fn main() {}")
        .unwrap();
    fs::write(temp_dir.path().join("test.rs"), "fn test() {}")
        .unwrap();

    let mut expanded = Vec::new();
    let patterns = vec!["test".to_string()];
    expand_path(
        temp_dir.path().to_str().unwrap(),
        &mut expanded,
        &patterns,
        false,
    )
        .unwrap();

    assert_eq!(expanded.len(), 1);
    assert!(expanded[0].ends_with("main.rs"));
}

/// Test determine_output_extension for Rust files only.
#[test]
fn test_determine_output_extension_rust_only() {
    let files = vec!["test.rs".to_string(), "main.rs".to_string()];
    let ext = determine_output_extension(&files);
    assert_eq!(ext, ".rs");
}

/// Test determine_output_extension for TOML files only.
#[test]
fn test_determine_output_extension_toml_only() {
    let files = vec!["Cargo.toml".to_string()];
    let ext = determine_output_extension(&files);
    assert_eq!(ext, ".toml");
}

/// Test determine_output_extension for mixed file types.
#[test]
fn test_determine_output_extension_mixed() {
    let files =
        vec!["main.rs".to_string(), "Cargo.toml".to_string()];
    let ext = determine_output_extension(&files);
    assert_eq!(ext, "._rs_and_toml.txt");
}

/// Test determine_output_extension for empty list defaults to .txt.
#[test]
fn test_determine_output_extension_empty() {
    let files = vec![];
    let ext = determine_output_extension(&files);
    assert_eq!(ext, ".txt");
}

/// Test determine_output_extension for a single web language.
#[test]
fn test_determine_output_extension_html_only() {
    let files = vec!["index.html".to_string()];
    let ext = determine_output_extension(&files);
    assert_eq!(ext, ".html");
}

/// Test determine_output_extension for two different languages.
#[test]
fn test_determine_output_extension_two_languages() {
    let files =
        vec!["index.html".to_string(), "style.css".to_string()];
    let ext = determine_output_extension(&files);
    assert_eq!(ext, "._html_and_css.txt");
}

/// Test determine_output_extension for more than two languages.
#[test]
fn test_determine_output_extension_many_languages() {
    let files = vec![
        "index.html".to_string(),
        "style.css".to_string(),
        "app.js".to_string(),
    ];
    let ext = determine_output_extension(&files);
    assert_eq!(ext, "._mixed.txt");
}

/// Test format_path_for_output removes Windows UNC prefix.
#[test]
fn test_format_path_for_output_windows_prefix() {
    let path = "\\\\?\\C:\\Users\\test\\file.rs";
    let formatted = format_path_for_output(path);
    assert!(!formatted.starts_with("\\\\?\\"));
    assert!(formatted.contains("Users"));
}

/// Test format_path_for_output preserves normal paths.
#[test]
fn test_format_path_for_output_normal() {
    let path = "/home/user/file.rs";
    let formatted = format_path_for_output(path);
    assert_eq!(formatted, path);
}

/// Test prepare_concat_destination with STDOUT.
#[test]
fn test_prepare_concat_destination_stdout() {
    let result =
        prepare_concat_destination("STDOUT", ".rs").unwrap();
    assert!(result.is_none());
}

/// Test prepare_concat_destination with uppercase STDOUT.
#[test]
fn test_prepare_concat_destination_stdout_uppercase() {
    let result =
        prepare_concat_destination("STDOUT", ".rs").unwrap();
    assert!(result.is_none());

    let result =
        prepare_concat_destination("stdout", ".rs").unwrap();
    assert!(result.is_none());
}

/// Test prepare_concat_destination with file path.
#[test]
fn test_prepare_concat_destination_file_path() {
    let temp_dir = TempDir::new().unwrap();
    let dest_path = temp_dir.path().join("output.txt");
    let result = prepare_concat_destination(
        dest_path.to_str().unwrap(),
        ".rs",
    )
        .unwrap();
    assert!(result.is_some());
}

/// Test prepare_concat_destination with directory path.
#[test]
fn test_prepare_concat_destination_directory() {
    let temp_dir = TempDir::new().unwrap();
    let result = prepare_concat_destination(
        temp_dir.path().to_str().unwrap(),
        ".rs",
    )
        .unwrap();
    assert!(result.is_some());
}

/// Test prepare_concat_destination with trailing slash creates directory.
#[test]
fn test_prepare_concat_destination_trailing_slash() {
    let temp_dir = TempDir::new().unwrap();
    let path_with_slash =
        format!("{}/", temp_dir.path().to_string_lossy());
    let result =
        prepare_concat_destination(&path_with_slash, ".rs").unwrap();
    assert!(result.is_some());
}

/// Test ProcessedFile cloning.
#[test]
fn test_processed_file_clone() {
    let pf = ProcessedFile {
        file_path: "main.rs".to_string(),
        language: language::find_language("rust").unwrap(),
        minified_content: "fn main(){}".to_string(),
    };

    let cloned = pf.clone();
    assert_eq!(pf.file_path, cloned.file_path);
    assert_eq!(pf.language.id, cloned.language.id);
    assert_eq!(pf.minified_content, cloned.minified_content);
}
