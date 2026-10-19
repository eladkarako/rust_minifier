use super::*;

#[test]
fn test_args_clone() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
    };
    let cloned = args.clone();
    assert_eq!(args.files, cloned.files);
    assert_eq!(args.suffix, cloned.suffix);
}

#[test]
fn test_args_default_suffix() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
    };
    assert_eq!(args.suffix, "_minified");
}

#[test]
fn test_args_concat_to_stdout() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: true,
    };
    assert_eq!(args.concat_to.to_uppercase(), "STDOUT");
}

#[test]
fn test_args_include_file_path_flag() {
    let args = Args {
        files: vec!["test.rs".to_string()],
        suffix: "_minified".to_string(),
        concat_to: "STDOUT".to_string(),
        include_file_path: false,
    };
    assert!(!args.include_file_path);
}
