use super::*;
use std::fs;
use std::thread;
use std::time::Duration;
use temp_dir::TempDir;

// ============================================================================
// Capture Metadata Tests
// ============================================================================

#[test]
fn test_capture_metadata_valid_file() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let meta = capture_metadata(&test_file).unwrap();
    assert!(meta.modified > 0);
    assert!(meta.accessed > 0);
}

#[test]
fn test_capture_metadata_nonexistent_file() {
    let temp = TempDir::new().unwrap();
    let nonexistent = temp.path().join("nonexistent.txt");

    let result = capture_metadata(&nonexistent);
    assert!(result.is_err());
}

#[test]
fn test_capture_metadata_modified_greater_than_accessed() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "initial content").unwrap();

    let meta = capture_metadata(&test_file).unwrap();
    // Modified and accessed should be close in time for a freshly created file
    assert!(
        (meta.modified as i64 - meta.accessed as i64).abs() <= 60
    );
}

#[test]
fn test_capture_metadata_empty_file() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("empty.txt");
    fs::write(&test_file, "").unwrap();

    let meta = capture_metadata(&test_file).unwrap();
    assert!(meta.modified > 0);
    assert!(meta.accessed > 0);
}

#[test]
fn test_capture_metadata_large_file() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("large.txt");
    let large_content = "x".repeat(1024 * 1024); // 1MB
    fs::write(&test_file, large_content).unwrap();

    let meta = capture_metadata(&test_file).unwrap();
    assert!(meta.modified > 0);
    assert!(meta.accessed > 0);
}

// ============================================================================
// Apply Metadata Tests
// ============================================================================

#[test]
fn test_apply_metadata_valid() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let original_meta = capture_metadata(&test_file).unwrap();

    // Modify the file
    fs::write(&test_file, "modified content").unwrap();

    // Apply original metadata
    apply_metadata(&test_file, &original_meta).unwrap();

    let restored_meta = capture_metadata(&test_file).unwrap();
    assert_eq!(restored_meta.modified, original_meta.modified);
    assert_eq!(restored_meta.accessed, original_meta.accessed);
}

#[test]
fn test_apply_metadata_nonexistent_file() {
    let temp = TempDir::new().unwrap();
    let nonexistent = temp.path().join("nonexistent.txt");

    let meta =
        FileMetadata { modified: 1609459200, accessed: 1609459200 };

    let result = apply_metadata(&nonexistent, &meta);
    assert!(result.is_err());
}

#[test]
fn test_apply_metadata_specific_timestamps() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    // Use a specific timestamp (Jan 1, 2021)
    let meta =
        FileMetadata { modified: 1609459200, accessed: 1609459200 };

    apply_metadata(&test_file, &meta).unwrap();

    let restored_meta = capture_metadata(&test_file).unwrap();
    // Allow a small tolerance due to system precision
    assert!(
        (restored_meta.modified as i64 - meta.modified as i64).abs()
            <= 1
    );
    assert!(
        (restored_meta.accessed as i64 - meta.accessed as i64).abs()
            <= 1
    );
}

#[test]
fn test_apply_metadata_different_modified_and_accessed() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let meta = FileMetadata {
        modified: 1609459200, // Jan 1, 2021
        accessed: 1640995200, // Jan 1, 2022
    };

    apply_metadata(&test_file, &meta).unwrap();

    let restored_meta = capture_metadata(&test_file).unwrap();
    assert!(
        (restored_meta.modified as i64 - meta.modified as i64).abs()
            <= 1
    );
    assert!(
        (restored_meta.accessed as i64 - meta.accessed as i64).abs()
            <= 1
    );
}

// ============================================================================
// Round-Trip Tests
// ============================================================================

#[test]
fn test_metadata_round_trip_capture_apply() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");
    fs::write(&test_file, "original content").unwrap();

    // Capture original metadata
    let original = capture_metadata(&test_file).unwrap();

    // Modify file multiple times
    for i in 0..3 {
        thread::sleep(Duration::from_millis(10));
        fs::write(&test_file, format!("content {}", i)).unwrap();
    }

    // Apply original metadata back
    apply_metadata(&test_file, &original).unwrap();

    // Verify
    let restored = capture_metadata(&test_file).unwrap();
    assert_eq!(restored.modified, original.modified);
    assert_eq!(restored.accessed, original.accessed);
}

#[test]
fn test_metadata_structure_clone() {
    let meta =
        FileMetadata { modified: 1609459200, accessed: 1640995200 };

    let cloned = meta.clone();
    assert_eq!(meta.modified, cloned.modified);
    assert_eq!(meta.accessed, cloned.accessed);
}

#[test]
fn test_metadata_debug_output() {
    let meta =
        FileMetadata { modified: 1609459200, accessed: 1640995200 };

    let debug_str = format!("{:?}", meta);
    assert!(debug_str.contains("modified"));
    assert!(debug_str.contains("accessed"));
}
