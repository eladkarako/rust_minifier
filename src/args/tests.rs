use super::*;

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
fn test_args_concat_output() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: Some("output.rs".to_string()),
        concat_to_stdout: false,
    };
    assert!(args.concat_output.is_some());
}

#[test]
fn test_args_concat_to_stdout() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: true,
    };
    assert!(args.concat_to_stdout);
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
}
