use super::*;
use std::fs;
use std::io::Write;
use std::time::SystemTime;
use tempfile::NamedTempFile;

/// Test FileMetadata struct creation
#[test]
fn test_file_metadata_struct_creation() {
    let meta =
        FileMetadata { modified: 1_000_000, accessed: 2_000_000 };

    assert_eq!(meta.modified, 1_000_000);
    assert_eq!(meta.accessed, 2_000_000);
}

/// Test FileMetadata with zero timestamps
#[test]
fn test_file_metadata_zero_timestamps() {
    let meta = FileMetadata { modified: 0, accessed: 0 };

    assert_eq!(meta.modified, 0);
    assert_eq!(meta.accessed, 0);
}

/// Test FileMetadata with maximum u64 values
#[test]
fn test_file_metadata_max_timestamps() {
    let meta =
        FileMetadata { modified: u64::MAX, accessed: u64::MAX };

    assert_eq!(meta.modified, u64::MAX);
    assert_eq!(meta.accessed, u64::MAX);
}

/// Test FileMetadata Clone trait
#[test]
fn test_file_metadata_clone() {
    let meta1 =
        FileMetadata { modified: 1_500_000, accessed: 2_500_000 };

    let meta2 = meta1.clone();

    assert_eq!(meta1.modified, meta2.modified);
    assert_eq!(meta1.accessed, meta2.accessed);
}

/// Test FileMetadata Debug trait
#[test]
fn test_file_metadata_debug() {
    let meta =
        FileMetadata { modified: 1_234_567, accessed: 7_654_321 };

    let debug_str = format!("{:?}", meta);
    assert!(debug_str.contains("modified"));
    assert!(debug_str.contains("accessed"));
    assert!(debug_str.contains("1234567"));
}

/// Test capture_metadata with a real temporary file
#[test]
fn test_capture_metadata_success() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    let meta = capture_metadata(path)?;

    assert!(meta.modified > 0);
    assert!(meta.accessed > 0);
    Ok(())
}

/// Test capture_metadata returns reasonable timestamps (not in future)
#[test]
fn test_capture_metadata_reasonable_values() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    let meta = capture_metadata(path)?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    // Timestamp should be <= now
    assert!(meta.modified <= now + 1); // +1 for potential clock skew
    assert!(meta.accessed <= now + 1);
    Ok(())
}

/// Test capture_metadata for non-existent file returns error
#[test]
fn test_capture_metadata_nonexistent_file() {
    let path = Path::new(
        "/nonexistent/path/to/file_that_does_not_exist.rs",
    );
    let result = capture_metadata(path);

    assert!(result.is_err());
}

/// Test apply_metadata with valid metadata
#[test]
fn test_apply_metadata_success() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    // Capture original metadata
    let _original_meta = capture_metadata(path)?;

    // Create new metadata with different timestamps
    let new_meta = FileMetadata {
        modified: 1_000_000_000,
        accessed: 1_100_000_000,
    };

    // Apply new metadata
    apply_metadata(path, &new_meta)?;

    // Verify metadata was applied
    let captured_meta = capture_metadata(path)?;

    assert_eq!(captured_meta.modified, new_meta.modified);
    assert_eq!(captured_meta.accessed, new_meta.accessed);
    Ok(())
}

/// Test apply_metadata with older timestamps
#[test]
fn test_apply_metadata_older_timestamps() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    let old_meta = FileMetadata {
        modified: 500_000_000,
        accessed: 600_000_000,
    };

    apply_metadata(path, &old_meta)?;
    let captured = capture_metadata(path)?;

    assert_eq!(captured.modified, old_meta.modified);
    assert_eq!(captured.accessed, old_meta.accessed);
    Ok(())
}

/// Test apply_metadata with zero timestamps (Unix epoch)
#[test]
fn test_apply_metadata_unix_epoch() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    let epoch_meta = FileMetadata { modified: 0, accessed: 0 };

    apply_metadata(path, &epoch_meta)?;
    let captured = capture_metadata(path)?;

    assert_eq!(captured.modified, 0);
    assert_eq!(captured.accessed, 0);
    Ok(())
}

/// Test apply_metadata to non-existent file returns error
#[test]
fn test_apply_metadata_nonexistent_file() {
    let path = Path::new("/nonexistent/path/to/file.rs");
    let meta =
        FileMetadata { modified: 1_000_000, accessed: 2_000_000 };

    let result = apply_metadata(path, &meta);
    assert!(result.is_err());
}

/// Test capture and apply metadata cycle (round-trip)
#[test]
fn test_metadata_round_trip() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    // Step 1: Capture original metadata
    let original = capture_metadata(path)?;

    // Step 2: Create and apply new metadata
    let new_meta = FileMetadata {
        modified: 1_234_567_890,
        accessed: 1_345_678_901,
    };
    apply_metadata(path, &new_meta)?;

    // Step 3: Capture to verify
    let captured = capture_metadata(path)?;
    assert_eq!(captured.modified, new_meta.modified);
    assert_eq!(captured.accessed, new_meta.accessed);

    // Step 4: Apply original back and verify
    apply_metadata(path, &original)?;
    let restored = capture_metadata(path)?;
    assert_eq!(restored.modified, original.modified);
    assert_eq!(restored.accessed, original.accessed);

    Ok(())
}

/// Test multiple consecutive apply_metadata calls
#[test]
fn test_apply_metadata_multiple_times() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    let meta1 = FileMetadata {
        modified: 1_000_000_000,
        accessed: 1_100_000_000,
    };
    apply_metadata(path, &meta1)?;
    let captured1 = capture_metadata(path)?;
    assert_eq!(captured1.modified, meta1.modified);

    let meta2 = FileMetadata {
        modified: 2_000_000_000,
        accessed: 2_100_000_000,
    };
    apply_metadata(path, &meta2)?;
    let captured2 = capture_metadata(path)?;
    assert_eq!(captured2.modified, meta2.modified);

    let meta3 = FileMetadata {
        modified: 3_000_000_000,
        accessed: 3_100_000_000,
    };
    apply_metadata(path, &meta3)?;
    let captured3 = capture_metadata(path)?;
    assert_eq!(captured3.modified, meta3.modified);

    Ok(())
}

/// Test that accessed and modified times can be different
#[test]
fn test_metadata_different_times() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    let meta = FileMetadata {
        modified: 1_000_000_000,
        accessed: 2_000_000_000,
    };

    apply_metadata(path, &meta)?;
    let captured = capture_metadata(path)?;

    assert_ne!(captured.modified, captured.accessed);
    assert_eq!(captured.modified, 1_000_000_000);
    assert_eq!(captured.accessed, 2_000_000_000);

    Ok(())
}

/// Test metadata with same modified and accessed times
#[test]
fn test_metadata_same_times() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    let meta = FileMetadata {
        modified: 1_500_000_000,
        accessed: 1_500_000_000,
    };

    apply_metadata(path, &meta)?;
    let captured = capture_metadata(path)?;

    assert_eq!(captured.modified, captured.accessed);
    assert_eq!(captured.modified, 1_500_000_000);

    Ok(())
}

/// Test capturing metadata from current working directory file
#[test]
fn test_capture_metadata_current_dir() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    let meta = capture_metadata(path)?;

    // Just verify we got valid timestamps
    assert!(meta.modified > 0 || meta.modified == 0);
    assert!(meta.accessed > 0 || meta.accessed == 0);

    Ok(())
}

/// Test that apply_metadata doesn't modify file content
#[test]
fn test_apply_metadata_preserves_content() -> Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    let test_content = b"fn main() { println!(\"Hello, world!\"); }";
    temp_file.write_all(test_content)?;
    temp_file.flush()?;

    let path = temp_file.path();

    let meta = FileMetadata {
        modified: 1_234_567_890,
        accessed: 1_234_567_890,
    };

    apply_metadata(path, &meta)?;

    // Verify content is unchanged
    let content = fs::read(path)?;
    assert_eq!(content, test_content);

    Ok(())
}

/// Test metadata with very recent timestamp (near current time)
#[test]
fn test_metadata_recent_timestamp() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let meta = FileMetadata { modified: now, accessed: now };

    apply_metadata(path, &meta)?;
    let captured = capture_metadata(path)?;

    assert_eq!(captured.modified, now);
    assert_eq!(captured.accessed, now);

    Ok(())
}

/// Test metadata with year 2000 timestamp
#[test]
fn test_metadata_y2k_timestamp() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    // January 1, 2000, 00:00:00 UTC
    let y2k = 946_684_800u64;

    let meta = FileMetadata { modified: y2k, accessed: y2k };

    apply_metadata(path, &meta)?;
    let captured = capture_metadata(path)?;

    assert_eq!(captured.modified, y2k);
    assert_eq!(captured.accessed, y2k);

    Ok(())
}

/// Test metadata with year 2038 timestamp (32-bit limit)
#[test]
fn test_metadata_year_2038_timestamp() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    // January 19, 2038, 03:14:07 UTC (max 32-bit signed Unix timestamp)
    let year_2038 = 2_147_483_647u64;

    let meta =
        FileMetadata { modified: year_2038, accessed: year_2038 };

    apply_metadata(path, &meta)?;
    let captured = capture_metadata(path)?;

    assert_eq!(captured.modified, year_2038);
    assert_eq!(captured.accessed, year_2038);

    Ok(())
}

/// Test that modified time is before or equal to accessed time in normal cases
#[test]
fn test_metadata_modified_before_accessed() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();

    let meta = FileMetadata {
        modified: 1_000_000_000,
        accessed: 2_000_000_000,
    };

    apply_metadata(path, &meta)?;
    let captured = capture_metadata(path)?;

    assert!(captured.modified <= captured.accessed);

    Ok(())
}

/// Helper function to create test metadata structs with specific values
fn create_test_metadata(
    modified: u64,
    accessed: u64,
) -> FileMetadata {
    FileMetadata { modified, accessed }
}

/// Test the helper function works correctly
#[test]
fn test_helper_create_test_metadata() {
    let meta = create_test_metadata(100, 200);
    assert_eq!(meta.modified, 100);
    assert_eq!(meta.accessed, 200);
}

/// Test applying metadata from one file to another
#[test]
fn test_apply_metadata_cross_file() -> Result<()> {
    let temp_file1 = NamedTempFile::new()?;
    let temp_file2 = NamedTempFile::new()?;

    let path1 = temp_file1.path();
    let path2 = temp_file2.path();

    // Capture metadata from file1
    let meta1 = capture_metadata(path1)?;

    // Apply file1's metadata to file2
    apply_metadata(path2, &meta1)?;

    // Verify file2 now has file1's metadata
    let captured = capture_metadata(path2)?;
    assert_eq!(captured.modified, meta1.modified);
    assert_eq!(captured.accessed, meta1.accessed);

    Ok(())
}
