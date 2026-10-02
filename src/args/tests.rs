use super::*;

// ============================================================================
// Suffix Tests
// ============================================================================

#[test]
fn test_args_default_suffix() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert_eq!(args.suffix, "_minified");
}

#[test]
fn test_args_custom_suffix() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_min".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert_eq!(args.suffix, "_min");
}

#[test]
fn test_args_custom_suffix_with_special_chars() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_v2.0".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert_eq!(args.suffix, "_v2.0");
}

#[test]
fn test_args_empty_suffix() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: String::new(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert!(args.suffix.is_empty());
}

// ============================================================================
// Files Tests
// ============================================================================

#[test]
fn test_args_single_file() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert_eq!(args.files.len(), 1);
    assert_eq!(args.files[0], "test.rs");
}

#[test]
fn test_args_multiple_files() {
    let args = Args {
        files: vec![
            "test1.rs".to_string(),
            "test2.rs".to_string(),
            "test3.rs".to_string(),
        ],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert_eq!(args.files.len(), 3);
    assert_eq!(args.files[0], "test1.rs");
    assert_eq!(args.files[1], "test2.rs");
    assert_eq!(args.files[2], "test3.rs");
}

#[test]
fn test_args_no_files() {
    let args = Args {
        files: vec![],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert!(args.files.is_empty());
}

#[test]
fn test_args_file_with_path() {
    let args = Args {
        files: vec!["/path/to/file.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert_eq!(args.files[0], "/path/to/file.rs");
}

// ============================================================================
// Concat Output Tests
// ============================================================================

#[test]
fn test_args_concat_output() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: Some("output.rs".to_string()),
        concat_to_stdout: false,
    };
    assert!(args.concat_output.is_some());
    assert_eq!(args.concat_output.unwrap(), "output.rs");
}

#[test]
fn test_args_concat_output_with_path() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: Some("/tmp/output.rs".to_string()),
        concat_to_stdout: false,
    };
    assert_eq!(args.concat_output.unwrap(), "/tmp/output.rs");
}

#[test]
fn test_args_no_concat_output() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert!(args.concat_output.is_none());
}

// ============================================================================
// Concat to Stdout Tests
// ============================================================================

#[test]
fn test_args_concat_to_stdout_enabled() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: true,
    };
    assert!(args.concat_to_stdout);
}

#[test]
fn test_args_concat_to_stdout_disabled() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert!(!args.concat_to_stdout);
}

// ============================================================================
// Combined Configuration Tests
// ============================================================================

#[test]
fn test_args_concat_output_and_concat_to_stdout_conflict() {
    // Both flags set (should not happen in practice, but test the struct)
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: Some("output.rs".to_string()),
        concat_to_stdout: true,
    };
    assert!(args.concat_output.is_some());
    assert!(args.concat_to_stdout);
}

#[test]
fn test_args_multiple_files_with_custom_suffix() {
    let args = Args {
        files: vec!["file1.rs".to_string(), "file2.rs".to_string()],
        suffix: "_optimized".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };
    assert_eq!(args.files.len(), 2);
    assert_eq!(args.suffix, "_optimized");
}

#[test]
fn test_args_clone_implementation() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: Some("output.rs".to_string()),
        concat_to_stdout: true,
    };
    let cloned = args.clone();
    assert_eq!(args.files, cloned.files);
    assert_eq!(args.suffix, cloned.suffix);
    assert_eq!(args.concat_output, cloned.concat_output);
    assert_eq!(args.concat_to_stdout, cloned.concat_to_stdout);
}

#[test]
fn test_args_debug_output() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: Some("output.rs".to_string()),
        concat_to_stdout: false,
    };
    let debug_str = format!("{:?}", args);
    assert!(debug_str.contains("test.rs"));
    assert!(debug_str.contains("_minified"));
}
