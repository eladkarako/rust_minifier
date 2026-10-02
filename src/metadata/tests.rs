use super::*;
use std::fs;
use std::path::Path;
use temp_dir::TempDir;

#[test]
fn test_capture_metadata() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let meta = capture_metadata(&test_file).unwrap();
    assert!(meta.modified > 0);
    assert!(meta.accessed > 0);
}

#[test]
fn test_apply_metadata() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let original_meta = capture_metadata(&test_file).unwrap();

    // Modify file
    fs::write(&test_file, "modified content").unwrap();

    // Apply original metadata
    apply_metadata(&test_file, &original_meta).unwrap();

    let restored_meta = capture_metadata(&test_file).unwrap();
    assert_eq!(restored_meta.modified, original_meta.modified);
    assert_eq!(restored_meta.accessed, original_meta.accessed);
}

