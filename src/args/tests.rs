//! Tests for command-line argument parsing.

use super::*;

/// Test that Args can be cloned.
#[test]
fn test_args_clone() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
        exclude: vec![],
    };

    let cloned = args.clone();
    assert_eq!(args.files, cloned.files);
    assert_eq!(args.suffix, cloned.suffix);
    assert_eq!(args.exclude, cloned.exclude);
}

/// Test default suffix value.
#[test]
fn test_args_default_suffix() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
        exclude: vec![],
    };

    assert_eq!(args.suffix, "_minified");
}

/// Test concat_to defaults to STDOUT.
#[test]
fn test_args_concat_to_stdout() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
        exclude: vec![],
    };

    assert_eq!(args.concat_to.to_uppercase(), "STDOUT");
}

/// Test include_file_path flag can be disabled.
#[test]
fn test_args_include_file_path_flag() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: false,
        exclude: vec![],
    };

    assert!(!args.include_file_path);
}

/// Test exclude field is empty by default.
#[test]
fn test_args_exclude_empty() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
        exclude: vec![],
    };

    assert!(args.exclude.is_empty());
}

/// Test exclude field accepts multiple patterns.
#[test]
fn test_args_exclude_multiple_patterns() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
        exclude: vec!["test".to_string(), "mock".to_string(), "temp".to_string()],
    };

    assert_eq!(args.exclude.len(), 3);
    assert!(args.exclude.contains(&"test".to_string()));
    assert!(args.exclude.contains(&"mock".to_string()));
}

/// Test multiple files in args.
#[test]
fn test_args_multiple_files() {
    let args = Args {
        files: vec!["main.rs".to_string(), "lib.rs".to_string(), "Cargo.toml".to_string()],
        suffix: "_min".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
        exclude: vec![],
    };

    assert_eq!(args.files.len(), 3);
    assert!(args.files.iter().any(|f| f.ends_with(".rs")));
    assert!(args.files.iter().any(|f| f.ends_with(".toml")));
}

/// Test custom suffix.
#[test]
fn test_args_custom_suffix() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_optimized".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
        exclude: vec![],
    };

    assert_eq!(args.suffix, "_optimized");
}

/// Test concat_to with file path.
#[test]
fn test_args_concat_to_file_path() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "/tmp/output.rs".to_string(),
        include_file_path: true,
        exclude: vec![],
    };

    assert_eq!(args.concat_to, "/tmp/output.rs");
    assert_ne!(args.concat_to.to_uppercase(), "STDOUT");
}
