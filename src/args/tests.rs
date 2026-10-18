use super::*;

/// Test that Args struct can be instantiated with default values
#[test]
fn test_args_struct_creation() {
    let args = Args {
        files: vec![],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.files.len(), 0);
    assert_eq!(args.suffix, "_minified");
    assert_eq!(args.concat_to, "STDOUT");
    assert_eq!(args.include_file_path, true);
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
        concat_to: String::from("STDOUT"),
        include_file_path: true,
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
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.suffix, "_custom_suffix");
}

#[test]
fn test_args_with_concat_to_stdout() {
    let args = Args {
        files: vec![String::from("file1.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "STDOUT");
}

#[test]
fn test_args_with_concat_to_stdout_lowercase() {
    let args = Args {
        files: vec![String::from("file1.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("stdout"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "stdout");
}

#[test]
fn test_args_with_concat_to_file_path() {
    let args = Args {
        files: vec![
            String::from("file1.rs"),
            String::from("file2.rs"),
        ],
        suffix: String::from("_minified"),
        concat_to: String::from("/output/concatenated.rs"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "/output/concatenated.rs");
    assert_eq!(args.files.len(), 2);
}

#[test]
fn test_args_with_concat_to_directory_path() {
    let args = Args {
        files: vec![String::from("file1.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("/output/"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "/output/");
}

#[test]
fn test_args_with_concat_to_relative_path() {
    let args = Args {
        files: vec![String::from("file1.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("./dist/minified_output.rs"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "./dist/minified_output.rs");
}

#[test]
fn test_args_empty_files_with_concat_to() {
    let args = Args {
        files: vec![],
        suffix: String::from("_min"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.files.is_empty(), true);
    assert_eq!(args.suffix, "_min");
    assert_eq!(args.concat_to, "STDOUT");
}

#[test]
fn test_args_clone() {
    let args1 = Args {
        files: vec![String::from("test.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("/output/file.rs"),
        include_file_path: true,
    };
    let args2 = args1.clone();
    assert_eq!(args1.files, args2.files);
    assert_eq!(args1.suffix, args2.suffix);
    assert_eq!(args1.concat_to, args2.concat_to);
    assert_eq!(args1.include_file_path, args2.include_file_path);
}

#[test]
fn test_args_debug() {
    let args = Args {
        files: vec![String::from("test.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    let debug_str = format!("{:?}", args);
    assert_eq!(debug_str.contains("files"), true);
    assert_eq!(debug_str.contains("suffix"), true);
    assert_eq!(debug_str.contains("_minified"), true);
    assert_eq!(debug_str.contains("concat_to"), true);
}

#[test]
fn test_args_suffix_with_special_chars() {
    let args = Args {
        files: vec![String::from("main.rs")],
        suffix: String::from("_min-v1.0"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.suffix, "_min-v1.0");
}

#[test]
fn test_args_with_many_files() {
    let mut files = vec![];
    for i in 0..100 {
        files.push(format!("file{}.rs", i));
    }
    let args = Args {
        files: files.clone(),
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.files.len(), 100);
    assert_eq!(args.files[0], "file0.rs");
    assert_eq!(args.files[99], "file99.rs");
}

#[test]
fn test_args_default_concat_to() {
    let args = Args {
        files: vec![],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "STDOUT");
}

#[test]
fn test_args_empty_suffix() {
    let args = Args {
        files: vec![String::from("test.rs")],
        suffix: String::new(),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.suffix.is_empty(), true);
}

#[test]
fn test_args_include_file_path_true() {
    let args = Args {
        files: vec![String::from("test.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.include_file_path, true);
}

#[test]
fn test_args_include_file_path_false() {
    let args = Args {
        files: vec![String::from("test.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: false,
    };
    assert_eq!(args.include_file_path, false);
}

#[test]
fn test_args_with_toml_files() {
    let args = Args {
        files: vec![
            String::from("config.toml"),
            String::from("settings.toml"),
        ],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.files.len(), 2);
    assert!(args.files[0].ends_with(".toml"));
    assert!(args.files[1].ends_with(".toml"));
}

#[test]
fn test_args_with_mixed_file_types() {
    let args = Args {
        files: vec![
            String::from("main.rs"),
            String::from("config.toml"),
            String::from("lib.rs"),
        ],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.files.len(), 3);
}

#[test]
fn test_args_concat_to_with_timestamp_style() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from(
            "./output/minified_20261018153045.rs",
        ),
        include_file_path: true,
    };
    assert_eq!(
        args.concat_to,
        "./output/minified_20261018153045.rs"
    );
}

#[test]
fn test_args_multiple_operations() {
    let args = Args {
        files: vec![
            String::from("src/main.rs"),
            String::from("src/lib.rs"),
            String::from("Cargo.toml"),
        ],
        suffix: String::from("_min"),
        concat_to: String::from("/dist/bundle"),
        include_file_path: true,
    };
    assert_eq!(args.files.len(), 3);
    assert_eq!(args.suffix, "_min");
    assert_eq!(args.concat_to, "/dist/bundle");
    assert_eq!(args.include_file_path, true);
}

#[test]
fn test_args_directory_path_in_files() {
    let args = Args {
        files: vec![
            String::from("./src/"),
            String::from("./config/"),
        ],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.files.len(), 2);
    assert!(args.files[0].ends_with("/"));
    assert!(args.files[1].ends_with("/"));
}

#[test]
fn test_args_default_include_file_path() {
    let args = Args {
        files: vec![String::from("test.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.include_file_path, true);
}

#[test]
fn test_args_all_fields_populated() {
    let args = Args {
        files: vec![
            String::from("file1.rs"),
            String::from("file2.toml"),
        ],
        suffix: String::from("_custom"),
        concat_to: String::from("./output/result.rs"),
        include_file_path: false,
    };
    assert_eq!(args.files.len(), 2);
    assert_eq!(args.suffix, "_custom");
    assert_eq!(args.concat_to, "./output/result.rs");
    assert_eq!(args.include_file_path, false);
}

#[test]
fn test_args_concat_to_file_path() {
    let args = Args {
        files: vec![
            String::from("file1.rs"),
            String::from("file2.rs"),
        ],
        suffix: String::from("_minified"),
        concat_to: String::from("/output/concatenated.rs"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "/output/concatenated.rs");
    assert_eq!(args.files.len(), 2);
}

#[test]
fn test_args_concat_to_windows_path() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("C:\\output\\minified.rs"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "C:\\output\\minified.rs");
}

#[test]
fn test_args_concat_to_windows_directory_path() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("C:\\output\\"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "C:\\output\\");
}

#[test]
fn test_args_single_directory_input() {
    let args = Args {
        files: vec![String::from("./src/")],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.files.len(), 1);
    assert_eq!(args.files[0], "./src/");
}

#[test]
fn test_args_absolute_directory_path() {
    let args = Args {
        files: vec![String::from("/home/user/projects/src")],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.files[0], "/home/user/projects/src");
}

#[test]
fn test_args_concat_to_stdout_default() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "STDOUT");
}

#[test]
fn test_args_concat_to_stdout_lowercase() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("stdout"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "stdout");
}

#[test]
fn test_args_concat_to_mixed_case_stdout() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("StdOut"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "StdOut");
}

#[test]
fn test_args_concat_to_nested_path() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("/path/to/nested/output/file.rs"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "/path/to/nested/output/file.rs");
}

#[test]
fn test_args_concat_to_nested_directory() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("/path/to/nested/output/"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "/path/to/nested/output/");
}

#[test]
fn test_args_clone_with_new_fields() {
    let args1 = Args {
        files: vec![String::from("test.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("/output/file.rs"),
        include_file_path: true,
    };
    let args2 = args1.clone();
    assert_eq!(args1.files, args2.files);
    assert_eq!(args1.suffix, args2.suffix);
    assert_eq!(args1.concat_to, args2.concat_to);
    assert_eq!(args1.include_file_path, args2.include_file_path);
}

#[test]
fn test_args_debug_with_concat_to() {
    let args = Args {
        files: vec![String::from("test.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    let debug_str = format!("{:?}", args);
    assert_eq!(debug_str.contains("files"), true);
    assert_eq!(debug_str.contains("suffix"), true);
    assert_eq!(debug_str.contains("_minified"), true);
    assert_eq!(debug_str.contains("concat_to"), true);
}

#[test]
fn test_args_large_file_list_with_concat() {
    let mut files = vec![];
    for i in 0..50 {
        files.push(format!("file{}.rs", i));
    }
    for i in 0..50 {
        files.push(format!("config{}.toml", i));
    }
    let args = Args {
        files: files.clone(),
        suffix: String::from("_minified"),
        concat_to: String::from("./output/all_minified.txt"),
        include_file_path: true,
    };
    assert_eq!(args.files.len(), 100);
    assert_eq!(args.concat_to, "./output/all_minified.txt");
}

#[test]
fn test_args_concat_to_current_dir() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("./"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "./");
}

#[test]
fn test_args_concat_to_current_dir_no_slash() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("."),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, ".");
}

#[test]
fn test_args_concat_to_parent_dir() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("../"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "../");
}

#[test]
fn test_args_concat_to_with_special_chars_in_filename() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("./output/minified-v1.0-final.rs"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "./output/minified-v1.0-final.rs");
}

#[test]
fn test_args_suffix_with_numbers() {
    let args = Args {
        files: vec![String::from("main.rs")],
        suffix: String::from("_v2024"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.suffix, "_v2024");
}

#[test]
fn test_args_mixed_directories_and_files() {
    let args = Args {
        files: vec![
            String::from("./src/"),
            String::from("./config/"),
            String::from("standalone.rs"),
            String::from("settings.toml"),
        ],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.files.len(), 4);
}

#[test]
fn test_args_with_dots_in_path() {
    let args = Args {
        files: vec![String::from("../../src/")],
        suffix: String::from("_minified"),
        concat_to: String::from("STDOUT"),
        include_file_path: true,
    };
    assert_eq!(args.files[0], "../../src/");
}

#[test]
fn test_args_concat_to_preserves_case() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("My_Output_Dir/Result.RS"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "My_Output_Dir/Result.RS");
}

#[test]
fn test_args_empty_concat_to_string() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::new(),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "");
}

#[test]
fn test_args_whitespace_in_concat_to() {
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("/output/my minified files/"),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, "/output/my minified files/");
}

#[test]
fn test_args_unicode_in_paths() {
    let args = Args {
        files: vec![String::from("файл.rs")],
        suffix: String::from("_minified"),
        concat_to: String::from("./выход/"),
        include_file_path: true,
    };
    assert_eq!(args.files[0], "файл.rs");
    assert_eq!(args.concat_to, "./выход/");
}

#[test]
fn test_args_very_long_path() {
    let long_path = "/very/long/path/to/some/deeply/nested/directory/structure/that/goes/on/and/on/output.rs".to_string();
    let args = Args {
        files: vec![String::from("file.rs")],
        suffix: String::from("_minified"),
        concat_to: long_path.clone(),
        include_file_path: true,
    };
    assert_eq!(args.concat_to, long_path);
}
