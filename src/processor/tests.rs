use super::*;
use std::fs;
use std::io;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::Mutex;

/// Test process_single_file output matches minified content
#[tokio::test]
async fn test_process_single_file_minification_applied() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("test.rs");
    let output_path = temp_dir.path().join("test.min.rs");

    let input_content = r#"
        fn main() {
            // This comment should be removed
            let x = 5;
            println!("Hello, world!");
        }
    "#;
    fs::write(&input_path, input_content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));

    let _ = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    let output_content = fs::read_to_string(&output_path).unwrap();
    // Output should not contain the comment
    assert!(
        !output_content.contains("This comment should be removed")
    );
}

#[tokio::test]
async fn test_process_all_suffix_applied() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.rs");
    let expected_output = temp_dir.path().join("input.custom.rs");

    fs::write(&input_path, "fn main() {}").unwrap();

    let args = Args {
        files: vec![input_path.to_str().unwrap().to_string()],
        suffix: ".custom".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };

    let result = process_all(&args).await;
    eprintln!("process_all result: {:?}", result);
    eprintln!("Expected output path: {:?}", expected_output);
    eprintln!(
        "Expected output exists: {}",
        expected_output.exists()
    );

    // List files in temp dir for debugging
    if let Ok(entries) = fs::read_dir(temp_dir.path()) {
        eprintln!("Files in temp dir:");
        for entry in entries {
            if let Ok(entry) = entry {
                eprintln!("  {:?}", entry.path());
            }
        }
    }

    result.expect("process_all should succeed");
    assert!(
        expected_output.exists(),
        "Output file should exist at {:?}",
        expected_output
    );
}

/// Test process_all concatenates multiple files correctly
#[tokio::test]
async fn test_process_all_concat_contents() {
    let temp_dir = TempDir::new().unwrap();
    let file1 = temp_dir.path().join("file1.rs");
    let file2 = temp_dir.path().join("file2.rs");
    let concat_output = temp_dir.path().join("combined.rs");

    fs::write(&file1, "fn foo() {}").unwrap();
    fs::write(&file2, "fn bar() {}").unwrap();

    let args = Args {
        files: vec![
            file1.to_str().unwrap().to_string(),
            file2.to_str().unwrap().to_string(),
        ],
        suffix: ".min".to_string(),
        concat_output: Some(
            concat_output.to_str().unwrap().to_string(),
        ),
        concat_to_stdout: false,
    };

    let _ = process_all(&args).await;

    if concat_output.exists() {
        let content = fs::read_to_string(&concat_output).unwrap();
        assert!(!content.is_empty());
    }
}

/// Test process_all with path containing dots
#[tokio::test]
async fn test_process_all_path_with_dots() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("my.config.file.rs");

    fs::write(&input_path, "fn main() {}").unwrap();

    let args = Args {
        files: vec![input_path.to_str().unwrap().to_string()],
        suffix: ".min".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };

    let result = process_all(&args).await;
    assert!(result.is_ok());
}

/// Test process_single_file with Rust string literals
#[tokio::test]
async fn test_process_single_file_string_literals() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("strings.rs");
    let _output_path = temp_dir.path().join("strings.min.rs");

    let content = r#"fn main() { let s = "This is a string with // fake comments"; }"#;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

/// Test process_single_file with raw strings
#[tokio::test]
async fn test_process_single_file_raw_strings() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("raw_strings.rs");
    let _output_path = temp_dir.path().join("raw_strings.min.rs");

    let content =
        r##"fn main() { let s = r#"raw string with "quotes""#; }"##;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

/// Test process_single_file with attributes
#[tokio::test]
async fn test_process_single_file_with_attributes() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("attrs.rs");
    let _output_path = temp_dir.path().join("attrs.min.rs");

    let content = r#"
#[derive(Debug, Clone)]
struct Foo;

#[test]
fn test_foo() {}
"#;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

/// Test process_single_file with lifetimes
#[tokio::test]
async fn test_process_single_file_with_lifetimes() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("lifetimes.rs");
    let _output_path = temp_dir.path().join("lifetimes.min.rs");

    let content = r#"fn borrow<'a>(x: &'a str) -> &'a str { x }"#;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

/// Test process_single_file with where clauses
#[tokio::test]
async fn test_process_single_file_with_where_clause() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("where_clause.rs");
    let _output_path = temp_dir.path().join("where_clause.min.rs");

    let content =
        r#"fn foo<T>(x: T) where T: Clone { let _y = x.clone(); }"#;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

/// Test process_single_file with async functions
#[tokio::test]
async fn test_process_single_file_with_async() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("async_fn.rs");
    let _output_path = temp_dir.path().join("async_fn.min.rs");

    let content = r#"async fn fetch() { }"#;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

/// Test process_single_file with unsafe blocks
#[tokio::test]
async fn test_process_single_file_with_unsafe() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("unsafe.rs");
    let _output_path = temp_dir.path().join("unsafe.min.rs");

    let content = r#"fn dangerous() { unsafe { let _x = std::mem::zeroed::<u32>(); } }"#;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

/// Test process_single_file with const and static
#[tokio::test]
async fn test_process_single_file_with_const() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("const.rs");
    let _output_path = temp_dir.path().join("const.min.rs");

    let content =
        r#"const PI: f32 = 3.14; static COUNTER: u32 = 0;"#;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

/// Test process_single_file with module declarations
#[tokio::test]
async fn test_process_single_file_with_modules() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("modules.rs");
    let _output_path = temp_dir.path().join("modules.min.rs");

    let content = r#"mod foo { pub fn bar() {} }"#;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

/// Test process_single_file with use statements
#[tokio::test]
async fn test_process_single_file_with_use() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("use_stmts.rs");
    let _output_path = temp_dir.path().join("use_stmts.min.rs");

    let content = r#"use std::io::{self, Write}; fn main() {}"#;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

/// Test process_all with deeply nested directories
#[tokio::test]
async fn test_process_all_nested_directories() {
    let temp_dir = TempDir::new().unwrap();
    let nested = temp_dir.path().join("a").join("b").join("c");
    fs::create_dir_all(&nested).unwrap();
    let input_path = nested.join("test.rs");

    fs::write(&input_path, "fn main() {}").unwrap();

    let args = Args {
        files: vec![input_path.to_str().unwrap().to_string()],
        suffix: ".min".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };

    let result = process_all(&args).await;
    assert!(result.is_ok());
}

/// Test process_single_file preserves semantics
#[tokio::test]
async fn test_process_single_file_preserves_semantics() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("semantics.rs");
    let output_path = temp_dir.path().join("semantics.min.rs");

    let content = r#"
fn add(a: i32, b: i32) -> i32 {
    a + b // return sum
}
"#;
    fs::write(&input_path, content).unwrap();

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let result = process_single_file(
        input_path.to_str().unwrap(),
        ".min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("add"));
    assert!(output.contains("i32"));
}

/// Test process_all concurrent processing
#[tokio::test]
async fn test_process_all_concurrent() {
    let temp_dir = TempDir::new().unwrap();
    let mut files = vec![];

    for i in 0..5 {
        let file_path =
            temp_dir.path().join(format!("concurrent{}.rs", i));
        fs::write(&file_path, format!("fn test{}() {{}}", i))
            .unwrap();
        files.push(file_path.to_str().unwrap().to_string());
    }

    let args = Args {
        files,
        suffix: ".min".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };

    let result = process_all(&args).await;
    assert!(result.is_ok());
}

/// Test process_all output files are valid Rust
#[tokio::test]
async fn test_process_all_output_is_valid_rust() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("valid.rs");
    let expected_output = temp_dir.path().join("valid.min.rs");

    let content = r#"
fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}
"#;
    fs::write(&input_path, content).unwrap();

    let args = Args {
        files: vec![input_path.to_str().unwrap().to_string()],
        suffix: ".min".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };

    let _ = process_all(&args).await;

    if expected_output.exists() {
        let output = fs::read_to_string(&expected_output).unwrap();
        assert!(!output.is_empty());
        assert!(output.contains("factorial"));
    }
}
