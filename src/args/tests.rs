//! Tests for command-line argument parsing.

use super::*;

/// Helper function to create Args with default values.
fn default_args() -> Args {
    Args {
        files: vec!["test.rs".to_string()],
        forced_type: None,
        preserve_line_breaks: false,
        list_types: false,
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
        no_include_file_path: false,
        exclude: vec![],
        soft_split: "unlimited".to_string(),
    }
}

/// Test that Args can be cloned.
#[test]
fn test_args_clone() {
    let args = default_args();
    let cloned = args.clone();

    assert_eq!(args.files, cloned.files);
    assert_eq!(args.suffix, cloned.suffix);
    assert_eq!(args.exclude, cloned.exclude);
    assert_eq!(args.soft_split, cloned.soft_split);
}

/// Test default suffix value.
#[test]
fn test_args_default_suffix() {
    let args = default_args();
    assert_eq!(args.suffix, "_minified");
}

/// Test concat_to defaults to STDOUT.
#[test]
fn test_args_concat_to_stdout() {
    let args = default_args();
    assert_eq!(args.concat_to.to_uppercase(), "STDOUT");
}

/// Test concat_to with file path.
#[test]
fn test_args_concat_to_file_path() {
    let mut args = default_args();
    args.concat_to = "/tmp/output.rs".to_string();

    assert_eq!(args.concat_to, "/tmp/output.rs");
    assert_ne!(args.concat_to.to_uppercase(), "STDOUT");
}

/// Test include_file_path flag can be enabled.
#[test]
fn test_args_include_file_path_enabled() {
    let args = default_args();
    assert!(args.include_file_path);
}

/// Test include_file_path flag can be disabled.
#[test]
fn test_args_include_file_path_disabled() {
    let mut args = default_args();
    args.include_file_path = false;

    assert!(!args.include_file_path);
}

/// Test exclude field is empty by default.
#[test]
fn test_args_exclude_empty_default() {
    let args = default_args();
    assert!(args.exclude.is_empty());
}

/// Test exclude field accepts single pattern.
#[test]
fn test_args_exclude_single_pattern() {
    let mut args = default_args();
    args.exclude = vec!["test".to_string()];

    assert_eq!(args.exclude.len(), 1);
    assert!(args.exclude.contains(&"test".to_string()));
}

/// Test exclude field accepts multiple patterns.
#[test]
fn test_args_exclude_multiple_patterns() {
    let mut args = default_args();
    args.exclude = vec![
        "test".to_string(),
        "mock".to_string(),
        "temp".to_string(),
    ];

    assert_eq!(args.exclude.len(), 3);
    assert!(args.exclude.contains(&"test".to_string()));
    assert!(args.exclude.contains(&"mock".to_string()));
    assert!(args.exclude.contains(&"temp".to_string()));
}

/// Test single file in args.
#[test]
fn test_args_single_file() {
    let args = default_args();
    assert_eq!(args.files.len(), 1);
    assert!(args.files[0].ends_with(".rs"));
}

/// Test multiple files in args.
#[test]
fn test_args_multiple_files() {
    let mut args = default_args();
    args.files = vec![
        "main.rs".to_string(),
        "lib.rs".to_string(),
        "Cargo.toml".to_string(),
    ];

    assert_eq!(args.files.len(), 3);
    assert!(args.files.iter().any(|f| f.ends_with(".rs")));
    assert!(args.files.iter().any(|f| f.ends_with(".toml")));
}

/// Test custom suffix.
#[test]
fn test_args_custom_suffix() {
    let mut args = default_args();
    args.suffix = "_optimized".to_string();

    assert_eq!(args.suffix, "_optimized");
}

/// Test suffix with special characters.
#[test]
fn test_args_suffix_with_special_chars() {
    let mut args = default_args();
    args.suffix = "_min-v2.0".to_string();

    assert_eq!(args.suffix, "_min-v2.0");
}

/// Test soft_split defaults to "unlimited".
#[test]
fn test_args_soft_split_default() {
    let args = default_args();
    assert_eq!(args.soft_split, "unlimited");
}

/// Test soft_split with numeric value.
#[test]
fn test_args_soft_split_numeric() {
    let mut args = default_args();
    args.soft_split = "10000".to_string();

    assert_eq!(args.soft_split, "10000");
}

/// Test soft_split with uppercase "UNLIMITED".
#[test]
fn test_args_soft_split_uppercase_unlimited() {
    let mut args = default_args();
    args.soft_split = "UNLIMITED".to_string();

    assert_eq!(args.soft_split.to_lowercase(), "unlimited");
}

/// Test soft_split with empty string (edge case).
#[test]
fn test_args_soft_split_empty() {
    let mut args = default_args();
    args.soft_split = String::new();

    assert!(args.soft_split.is_empty());
}

/// Test all fields together in a realistic scenario.
#[test]
fn test_args_complete_configuration() {
    let args = Args {
        files: vec![
            "src/main.rs".to_string(),
            "src/lib.rs".to_string(),
            "Cargo.toml".to_string(),
        ],
        forced_type: Some("rust".to_string()),
        preserve_line_breaks: true,
        list_types: false,
        suffix: "_min".to_string(),
        concat_to: "/output/minified.txt".to_string(),
        include_file_path: true,
        no_include_file_path: false,
        exclude: vec!["test".to_string(), "mock".to_string()],
        soft_split: "50000".to_string(),
    };

    assert_eq!(args.files.len(), 3);
    assert_eq!(args.suffix, "_min");
    assert_eq!(args.concat_to, "/output/minified.txt");
    assert!(args.include_file_path);
    assert!(args.preserve_line_breaks);
    assert_eq!(args.forced_type.as_deref(), Some("rust"));
    assert_eq!(args.exclude.len(), 2);
    assert_eq!(args.soft_split, "50000");
}

/// Test files with various paths.
#[test]
fn test_args_various_file_paths() {
    let mut args = default_args();
    args.files = vec![
        "./src/main.rs".to_string(),
        "../other/lib.rs".to_string(),
        "C:\\Windows\\path\\file.toml".to_string(),
        "/absolute/unix/path/code.rs".to_string(),
    ];

    assert_eq!(args.files.len(), 4);
    assert!(args.files.iter().any(|f| f.starts_with("./")));
    assert!(args.files.iter().any(|f| f.starts_with("../")));
    assert!(args.files.iter().any(|f| f.starts_with("C:\\")));
    assert!(args.files.iter().any(|f| f.starts_with("/")));
}

/// Test exclude patterns with mixed case.
#[test]
fn test_args_exclude_case_sensitivity() {
    let mut args = default_args();
    args.exclude = vec!["Test".to_string(), "MOCK".to_string()];

    // Verify patterns are stored as-is (case sensitivity depends on usage)
    assert_eq!(args.exclude[0], "Test");
    assert_eq!(args.exclude[1], "MOCK");
}

/// Test concat_to with trailing slash.
#[test]
fn test_args_concat_to_directory_path() {
    let mut args = default_args();
    args.concat_to = "/output/".to_string();

    assert!(args.concat_to.ends_with("/"));
}

/// Test concat_to with backslash (Windows-style).
#[test]
fn test_args_concat_to_windows_path() {
    let mut args = default_args();
    args.concat_to = "C:\\output\\minified.rs".to_string();

    assert!(args.concat_to.contains("\\"));
}

/// Test empty files vector (edge case).
#[test]
fn test_args_empty_files_vector() {
    let mut args = default_args();
    args.files = vec![];

    assert!(args.files.is_empty());
}

/// Test args clone independence (mutations don't affect original).
#[test]
fn test_args_clone_independence() {
    let args1 = default_args();
    let mut args2 = args1.clone();

    args2.suffix = "_different".to_string();
    args2.files.push("new.rs".to_string());

    assert_eq!(args1.suffix, "_minified");
    assert_eq!(args1.files.len(), 1);
    assert_eq!(args2.suffix, "_different");
    assert_eq!(args2.files.len(), 2);
}

/// Test that no_include_file_path wins over include_file_path.
#[test]
fn test_args_include_source_path_override() {
    let mut args = default_args();
    assert!(args.include_source_path());

    args.no_include_file_path = true;
    assert!(!args.include_source_path());
}

/// Test minifier options reflect preserve_line_breaks.
#[test]
fn test_args_minifier_options_line_breaks() {
    let mut args = default_args();
    assert!(!args.minifier_options().preserve_line_breaks);

    args.preserve_line_breaks = true;
    assert!(args.minifier_options().preserve_line_breaks);
}

/// Test forced_type is optional by default.
#[test]
fn test_args_forced_type_default() {
    let args = default_args();
    assert!(args.forced_type.is_none());
}

/// Test forced_type accepts a language id.
#[test]
fn test_args_forced_type_value() {
    let mut args = default_args();
    args.forced_type = Some("html".to_string());
    assert_eq!(args.forced_type.as_deref(), Some("html"));
}

/// Test list_types defaults to false.
#[test]
fn test_args_list_types_default() {
    let args = default_args();
    assert!(!args.list_types);
}
