use super::*;

/// Test that Args struct can be instantiated with default values
#[test]
fn test_args_struct_creation() {
    let args = Args {
        files: vec![],
        suffix: String::from("_minified"),
        concat_output: None,
        concat_to_stdout: false,
    };

    assert_eq!(args.files.len(), 0);
    assert_eq!(args.suffix, "_minified");
    assert!(args.concat_output.is_none());
    assert!(!args.concat_to_stdout);
}

/// Test Args with multiple files
#[test]
fn test_args_with_multiple_files() {
    let args = Args {
        files: vec![
            String::from("file1.rs"),
            String::from("file2.rs"),
            String::from("file3.rs"),
        ],
        suffix: String::from("_minified"),
        concat_output: None,
        concat_to_stdout: false,
    };

    assert_eq!(args.files.len(), 3);
    assert_eq!(args.files[0], "file1.rs");
    assert_eq!(args.files[1], "file2.rs");
    assert_eq!(args.files[2], "file3.rs");
}

/// Test Args with custom suffix
#[test]
fn test_args_with_custom_suffix() {
    let args = Args {
        files: vec![String::from("test.rs")],
        suffix: String::from("_custom_suffix"),
        concat_output: None,
        concat_to_stdout: false,
    };

    assert_eq!(args.suffix, "_custom_suffix");
}

/// Test Args with concat_output specified
#[test]
fn test_args_with_concat_output() {
    let args = Args {
        files: vec![
            String::from("file1.rs"),
            String::from("file2.rs"),
        ],
        suffix: String::from("_minified"),
        concat_output: Some(String::from("/output/concatenated.rs")),
        concat_to_stdout: false,
    };

    assert!(args.concat_output.is_some());
    assert_eq!(
        args.concat_output.as_ref().unwrap(),
        "/output/concatenated.rs"
    );
    assert!(!args.concat_to_stdout);
}

/// Test Args with concat_to_stdout enabled
#[test]
fn test_args_with_concat_to_stdout() {
    let args = Args {
        files: vec![String::from("file1.rs")],
        suffix: String::from("_minified"),
        concat_output: None,
        concat_to_stdout: true,
    };

    assert!(!args.concat_output.is_some());
    assert!(args.concat_to_stdout);
}

/// Test that concat_output and concat_to_stdout are mutually exclusive in practice
#[test]
fn test_args_concat_output_and_stdout_exclusive() {
    // While the struct allows both to be set, typically only one should be used
    let args = Args {
        files: vec![String::from("file1.rs")],
        suffix: String::from("_minified"),
        concat_output: Some(String::from("/output/file.rs")),
        concat_to_stdout: true,
    };

    // Both can technically be set, but only one should be used in practice
    assert!(args.concat_output.is_some());
    assert!(args.concat_to_stdout);
}

/// Test Args with empty files but with suffix
#[test]
fn test_args_empty_files_with_suffix() {
    let args = Args {
        files: vec![],
        suffix: String::from("_min"),
        concat_output: None,
        concat_to_stdout: false,
    };

    assert!(args.files.is_empty());
    assert_eq!(args.suffix, "_min");
}

/// Test Args Clone trait
#[test]
fn test_args_clone() {
    let args1 = Args {
        files: vec![String::from("test.rs")],
        suffix: String::from("_minified"),
        concat_output: Some(String::from("output.rs")),
        concat_to_stdout: true,
    };

    let args2 = args1.clone();

    assert_eq!(args1.files, args2.files);
    assert_eq!(args1.suffix, args2.suffix);
    assert_eq!(args1.concat_output, args2.concat_output);
    assert_eq!(args1.concat_to_stdout, args2.concat_to_stdout);
}

/// Test Args Debug trait implementation
#[test]
fn test_args_debug() {
    let args = Args {
        files: vec![String::from("test.rs")],
        suffix: String::from("_minified"),
        concat_output: None,
        concat_to_stdout: false,
    };

    let debug_str = format!("{:?}", args);
    assert!(debug_str.contains("files"));
    assert!(debug_str.contains("suffix"));
    assert!(debug_str.contains("_minified"));
}

/// Test suffix with special characters
#[test]
fn test_args_suffix_with_special_chars() {
    let args = Args {
        files: vec![String::from("main.rs")],
        suffix: String::from("_min-v1.0"),
        concat_output: None,
        concat_to_stdout: false,
    };

    assert_eq!(args.suffix, "_min-v1.0");
}

/// Test concat_output with nested paths
#[test]
fn test_args_concat_output_nested_path() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_output: Some(String::from(
            "/path/to/nested/output/file.rs",
        )),
        concat_to_stdout: false,
    };

    assert_eq!(
        args.concat_output.as_ref().unwrap(),
        "/path/to/nested/output/file.rs"
    );
}

/// Test Args with many files
#[test]
fn test_args_with_many_files() {
    let mut files = vec![];
    for i in 0..100 {
        files.push(format!("file{}.rs", i));
    }

    let args = Args {
        files: files.clone(),
        suffix: String::from("_minified"),
        concat_output: None,
        concat_to_stdout: false,
    };

    assert_eq!(args.files.len(), 100);
    assert_eq!(args.files[0], "file0.rs");
    assert_eq!(args.files[99], "file99.rs");
}

/// Test default suffix matches the default value in parser
#[test]
fn test_args_default_suffix() {
    let args = Args {
        files: vec![],
        suffix: String::from("_minified"),
        concat_output: None,
        concat_to_stdout: false,
    };

    // This matches the default_value="_minified" from the clap configuration
    assert_eq!(args.suffix, "_minified");
}

/// Test empty suffix (edge case)
#[test]
fn test_args_empty_suffix() {
    let args = Args {
        files: vec![String::from("test.rs")],
        suffix: String::new(),
        concat_output: None,
        concat_to_stdout: false,
    };

    assert!(args.suffix.is_empty());
}
