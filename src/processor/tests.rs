use super::*;
use std::fs;
use temp_dir::TempDir;

#[test]
fn test_file_type_ordering() {
    let toml_type = FileType::Toml;
    let rust_type = FileType::Rust;
    assert!(toml_type < rust_type);
}

#[test]
fn test_file_type_from_path_toml() {
    let file_type = FileType::from_path("config.toml");
    assert_eq!(file_type, FileType::Toml);
}

#[test]
fn test_file_type_from_path_rust() {
    let file_type = FileType::from_path("main.rs");
    assert_eq!(file_type, FileType::Rust);
}

#[test]
fn test_expand_path_single_file() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn main() {}").unwrap();

    let mut expanded = Vec::new();
    expand_path(test_file.to_str().unwrap(), &mut expanded).unwrap();
    assert_eq!(expanded.len(), 1);
    assert!(expanded[0].ends_with(".rs"));
}

#[test]
fn test_expand_path_nonexistent_file() {
    let mut expanded = Vec::new();
    let result = expand_path("/nonexistent/file.rs", &mut expanded);
    assert!(result.is_err());
}

#[test]
fn test_determine_output_extension_rust_only() {
    let files = vec!["test.rs".to_string(), "main.rs".to_string()];
    let ext = determine_output_extension(&files);
    assert_eq!(ext, ".rs");
}

#[test]
fn test_determine_output_extension_toml_only() {
    let files = vec!["Cargo.toml".to_string()];
    let ext = determine_output_extension(&files);
    assert_eq!(ext, ".toml");
}

#[test]
fn test_determine_output_extension_mixed() {
    let files =
        vec!["main.rs".to_string(), "Cargo.toml".to_string()];
    let ext = determine_output_extension(&files);
    assert_eq!(ext, "._rs_and_toml.txt");
}

#[test]
fn test_format_path_for_output_windows_prefix() {
    let path = "\\\\?\\C:\\Users\\test\\file.rs";
    let formatted = format_path_for_output(path);
    assert!(!formatted.starts_with("\\\\?\\"));
}

#[test]
fn test_format_path_for_output_normal() {
    let path = "/home/user/file.rs";
    let formatted = format_path_for_output(path);
    assert_eq!(formatted, path);
}

#[test]
fn test_prepare_concat_destination_stdout() {
    let result =
        prepare_concat_destination("STDOUT", ".rs").unwrap();
    assert!(result.is_none());
}

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
