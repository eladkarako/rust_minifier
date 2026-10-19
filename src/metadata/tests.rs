use super::*;
use std::fs;
use temp_dir::TempDir;

#[test]
fn test_capture_metadata() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let meta = capture_metadata(&test_file).unwrap();
    assert!(meta.modified > 0);
    assert!(meta.accessed > 0);
}

#[test]
fn test_apply_metadata() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let original_meta = capture_metadata(&test_file).unwrap();

    let modified_seconds_ago = original_meta.modified - 3600;
    let new_meta = FileMetadata {
        modified: modified_seconds_ago,
        accessed: original_meta.accessed,
    };

    let result = apply_metadata(&test_file, &new_meta);
    assert!(result.is_ok());

    let captured = capture_metadata(&test_file).unwrap();
    assert_eq!(captured.modified, modified_seconds_ago);
}

#[test]
fn test_metadata_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let meta1 = capture_metadata(&test_file).unwrap();
    apply_metadata(&test_file, &meta1).unwrap();
    let meta2 = capture_metadata(&test_file).unwrap();

    assert_eq!(meta1.modified, meta2.modified);
    assert_eq!(meta1.accessed, meta2.accessed);
}
