use super::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use temp_dir::TempDir;

// ============================================================================
// Get Deduplicated Path Tests
// ============================================================================

#[test]
fn test_get_deduplicated_path_first_file() {
    let temp = TempDir::new().unwrap();
    let original = temp.path().join("test.rs");
    let result =
        get_deduplicated_path(&original, "_minified").unwrap();
    let result_str = result.to_string_lossy();

    assert!(result_str.contains("test"));
    assert!(result_str.contains("_minified"));
    assert!(result_str.ends_with(".rs"));
    assert!(result_str.contains("_1"));
}

#[test]
fn test_get_deduplicated_path_collision_single() {
    let temp = TempDir::new().unwrap();
    let original = temp.path().join("test.rs");

    // Create the base output file to force collision
    fs::File::create(temp.path().join("test_minified.rs")).unwrap();
    let result =
        get_deduplicated_path(&original, "_minified").unwrap();
    let result_str = result.to_string_lossy();

    assert!(result_str.contains("_minified"));
    assert!(result_str.contains("_1"));
    assert_eq!(
        result_str,
        temp.path().join("test_minified_1.rs").to_string_lossy()
    );
}

#[test]
fn test_get_deduplicated_path_collision_multiple() {
    let temp = TempDir::new().unwrap();
    let original = temp.path().join("test.rs");

    // Create multiple colliding files
    fs::File::create(temp.path().join("test_minified.rs")).unwrap();
    fs::File::create(temp.path().join("test_minified_1.rs"))
        .unwrap();
    fs::File::create(temp.path().join("test_minified_2.rs"))
        .unwrap();

    let result =
        get_deduplicated_path(&original, "_minified").unwrap();
    let result_str = result.to_string_lossy();

    assert!(result_str.contains("test_minified_3.rs"));
}

#[test]
fn test_get_deduplicated_path_different_suffix() {
    let temp = TempDir::new().unwrap();
    let original = temp.path().join("code.rs");

    let result = get_deduplicated_path(&original, "_opt").unwrap();
    let result_str = result.to_string_lossy();

    assert!(result_str.contains("code"));
    assert!(result_str.contains("_opt"));
    assert!(result_str.ends_with(".rs"));
}

#[test]
fn test_get_deduplicated_path_custom_suffix_with_numbers() {
    let temp = TempDir::new().unwrap();
    let original = temp.path().join("script.rs");

    let result = get_deduplicated_path(&original, "_v2").unwrap();
    let result_str = result.to_string_lossy();

    assert!(result_str.contains("script"));
    assert!(result_str.contains("_v2"));
}

#[test]
fn test_get_deduplicated_path_empty_suffix() {
    let temp = TempDir::new().unwrap();
    let original = temp.path().join("test.rs");

    let result = get_deduplicated_path(&original, "").unwrap();
    let result_str = result.to_string_lossy();

    assert!(result_str.contains("test"));
    assert!(result_str.ends_with(".rs"));
    assert!(result_str.contains("_1"));
}

#[test]
fn test_get_deduplicated_path_with_dots_in_name() {
    let temp = TempDir::new().unwrap();
    let original = temp.path().join("test.module.rs");

    let result = get_deduplicated_path(&original, "_min").unwrap();
    let result_str = result.to_string_lossy();

    assert!(result_str.contains("test.module"));
    assert!(result_str.contains("_min"));
}

#[test]
fn test_get_deduplicated_path_gaps_in_numbers() {
    let temp = TempDir::new().unwrap();
    let original = temp.path().join("test.rs");

    // Create files with gaps
    fs::File::create(temp.path().join("test_min_1.rs")).unwrap();
    fs::File::create(temp.path().join("test_min_2.rs")).unwrap();
    // Skip _3
    fs::File::create(temp.path().join("test_min_4.rs")).unwrap();

    let result = get_deduplicated_path(&original, "_min").unwrap();
    let result_str = result.to_string_lossy();

    // Should find the next available number
    assert!(
        result_str.contains("test_min_3.rs")
            || result_str.contains("test_min_5.rs")
    );
}

#[test]
fn test_get_deduplicated_path_no_parent_directory() {
    // This test depends on PathBuf behavior; test with root-like path
    let path = PathBuf::from("test.rs");
    let result = get_deduplicated_path(&path, "_min");

    // Should succeed even without explicit parent (uses current dir)
    assert!(result.is_ok());
}

// ============================================================================
// Process Single File Tests
// ============================================================================

#[tokio::test]
async fn test_process_single_file_creates_output() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("input.rs");
    let code = "fn main() { println!(\"hello\"); }";
    fs::write(&input_file, code).unwrap();

    let stdout = Arc::new(Mutex::new(std::io::stdout()));
    let result = process_single_file(
        input_file.to_str().unwrap(),
        "_minified",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());

    // Verify output file was created
    let output_file = temp.path().join("input_minified_1.rs");
    assert!(output_file.exists());
}

#[tokio::test]
async fn test_process_single_file_minifies_code() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("input.rs");
    let code = r#"
    fn main() {
        // This is a comment
        let x = 5;
        println!("Hello");
    }
    "#;
    fs::write(&input_file, code).unwrap();

    let stdout = Arc::new(Mutex::new(std::io::stdout()));
    let result = process_single_file(
        input_file.to_str().unwrap(),
        "_min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());

    // Verify output is minified
    let output_file = temp.path().join("input_min_1.rs");
    let output_content = fs::read_to_string(output_file).unwrap();
    assert!(!output_content.contains("// This is a comment"));
    assert!(output_content.contains("fn main"));
}

#[tokio::test]
async fn test_process_single_file_preserves_metadata() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("input.rs");
    let code = "fn test() {}";
    fs::write(&input_file, code).unwrap();

    // Capture original metadata
    let original_meta =
        crate::metadata::capture_metadata(&input_file).unwrap();

    let stdout = Arc::new(Mutex::new(std::io::stdout()));
    let result = process_single_file(
        input_file.to_str().unwrap(),
        "_min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());

    // Verify metadata was applied to output
    let output_file = temp.path().join("input_min_1.rs");
    let output_meta =
        crate::metadata::capture_metadata(&output_file).unwrap();

    // Metadata should match original
    assert_eq!(output_meta.modified, original_meta.modified);
}

#[tokio::test]
async fn test_process_single_file_with_concat_output() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("input.rs");
    let concat_file = temp.path().join("combined.rs");

    fs::write(&input_file, "fn main() {}").unwrap();

    let stdout = Arc::new(Mutex::new(std::io::stdout()));
    let result = process_single_file(
        input_file.to_str().unwrap(),
        "_minified",
        Some(concat_file.to_str().unwrap()),
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());

    // Verify concat file exists and has content
    assert!(concat_file.exists());
    let content = fs::read_to_string(&concat_file).unwrap();
    assert!(content.contains("fn main"));
}

#[tokio::test]
async fn test_process_single_file_concat_append() {
    let temp = TempDir::new().unwrap();
    let input_file1 = temp.path().join("input1.rs");
    let input_file2 = temp.path().join("input2.rs");
    let concat_file = temp.path().join("combined.rs");

    fs::write(&input_file1, "fn foo() {}").unwrap();
    fs::write(&input_file2, "fn bar() {}").unwrap();

    let stdout = Arc::new(Mutex::new(std::io::stdout()));

    // Process first file
    let result1 = process_single_file(
        input_file1.to_str().unwrap(),
        "_min",
        Some(concat_file.to_str().unwrap()),
        false,
        stdout.clone(),
    )
        .await;
    assert!(result1.is_ok());

    // Process second file
    let result2 = process_single_file(
        input_file2.to_str().unwrap(),
        "_min",
        Some(concat_file.to_str().unwrap()),
        false,
        stdout,
    )
        .await;
    assert!(result2.is_ok());

    // Verify both contents are in concat file
    let content = fs::read_to_string(&concat_file).unwrap();
    assert!(content.contains("foo"));
    assert!(content.contains("bar"));
}

#[tokio::test]
async fn test_process_single_file_concat_to_stdout() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("input.rs");

    fs::write(&input_file, "fn test() {}").unwrap();

    let stdout = Arc::new(Mutex::new(std::io::stdout()));
    let result = process_single_file(
        input_file.to_str().unwrap(),
        "_minified",
        None,
        true,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_process_single_file_nonexistent_input() {
    let stdout = Arc::new(Mutex::new(std::io::stdout()));
    let result = process_single_file(
        "/nonexistent/path/file.rs",
        "_minified",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_process_single_file_invalid_utf8() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("input.rs");

    // Write invalid UTF-8
    let mut file = fs::File::create(&input_file).unwrap();
    file.write_all(&[0xFF, 0xFE]).unwrap();
    drop(file);

    let stdout = Arc::new(Mutex::new(std::io::stdout()));
    let result = process_single_file(
        input_file.to_str().unwrap(),
        "_minified",
        None,
        false,
        stdout,
    )
        .await;

    // Should fail on UTF-8 conversion
    assert!(result.is_err());
}

#[tokio::test]
async fn test_process_single_file_empty_file() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("empty.rs");

    fs::write(&input_file, "").unwrap();

    let stdout = Arc::new(Mutex::new(std::io::stdout()));
    let result = process_single_file(
        input_file.to_str().unwrap(),
        "_min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());

    let output_file = temp.path().join("empty_min_1.rs");
    assert!(output_file.exists());

    let content = fs::read_to_string(&output_file).unwrap();
    assert!(content.is_empty());
}

#[tokio::test]
async fn test_process_single_file_large_file() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("large.rs");

    // Create a large file with repeated code
    let large_content = (0..1000)
        .map(|i| format!("fn func_{}() {{}}", i))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(&input_file, large_content).unwrap();

    let stdout = Arc::new(Mutex::new(std::io::stdout()));
    let result = process_single_file(
        input_file.to_str().unwrap(),
        "_min",
        None,
        false,
        stdout,
    )
        .await;

    assert!(result.is_ok());
}

// ============================================================================
// Process Stdin Tests
// ============================================================================

#[tokio::test]
async fn test_process_stdin_is_terminal() {
    let args = Args {
        files: vec![],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };

    // When stdin is a terminal, should return early
    let result = process_stdin(&args).await;
    assert!(result.is_ok());
}

// ============================================================================
// Process All Tests
// ============================================================================

#[tokio::test]
async fn test_process_all_single_file() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("test.rs");
    fs::write(&input_file, "fn main() {}").unwrap();

    let args = Args {
        files: vec![input_file.to_string_lossy().to_string()],
        suffix: "_minified".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };

    let result = process_all(&args).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_process_all_multiple_files() {
    let temp = TempDir::new().unwrap();
    let file1 = temp.path().join("test1.rs");
    let file2 = temp.path().join("test2.rs");
    let file3 = temp.path().join("test3.rs");

    // Write test content to files
    fs::write(&file1, "fn foo() { let x = 1; }").unwrap();
    fs::write(&file2, "fn bar() { let y = 2; }").unwrap();
    fs::write(&file3, "fn baz() { let z = 3; }").unwrap();

    let args = Args {
        files: vec![
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
            file3.to_string_lossy().to_string(),
        ],
        suffix: "_min".to_string(),
        concat_output: None,
        concat_to_stdout: false,
    };

    let result = process_all(&args).await;
    assert!(result.is_ok(), "process_all should succeed");

    // Check what files actually exist in temp directory
    let entries: Vec<_> = fs::read_dir(temp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name())
        .collect();

    // Verify we have 6 files: 3 original + 3 minified
    assert_eq!(
        entries.len(),
        6,
        "Should have 3 original files and 3 minified files. Found: {:?}",
        entries
    );

    // Verify output files were created with correct suffix
    let has_min_files = entries
        .iter()
        .filter(|name| name.to_string_lossy().contains("_min"))
        .count();

    assert_eq!(
        has_min_files, 3,
        "Should have exactly 3 minified files"
    );
}

#[test]
fn test_get_deduplicated_path_collision() {
    let temp = TempDir::new().unwrap();
    let original = temp.path().join("test.rs");

    // Create the base output file to force collision
    std::fs::File::create(temp.path().join("test_minified.rs"))
        .unwrap();

    let result =
        get_deduplicated_path(&original, "_minified").unwrap();
    let result_str = result.to_string_lossy();

    assert!(
        result_str.contains("_minified"),
        "Result should contain suffix"
    );
    assert!(
        result_str.contains("1"),
        "Result should contain deduplication number"
    );
}
