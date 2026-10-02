use super::*;
use temp_dir::TempDir;

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
}

#[test]
fn test_get_deduplicated_path_collision() {
    let temp = TempDir::new().unwrap();
    let original = temp.path().join("test.rs");

    // Create first output file
    std::fs::File::create(temp.path().join("test_minified_1.rs"))
        .unwrap();

    let result =
        get_deduplicated_path(&original, "_minified").unwrap();
    let result_str = result.to_string_lossy();

    assert!(result_str.contains("_minified"));
    assert!(result_str.contains("1"));
}

#[tokio::test]
async fn test_process_single_file_creates_output() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("input.rs");
    let code = "fn main() { println!(\"hello\"); }";
    std::fs::write(&input_file, code).unwrap();

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
}
