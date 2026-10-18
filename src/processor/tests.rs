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
    assert_eq!(
        output_content.contains("This comment should be removed"),
        false
    );
}
