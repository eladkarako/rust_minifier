use anyhow::Result;
use filetime::{set_file_atime, set_file_mtime, FileTime};
use std::fs;
use std::path::Path;

/// Metadata for a file (timestamps).
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub modified: u64,
    pub accessed: u64,
}

/// Capture file metadata before processing.
pub fn capture_metadata(path: &Path) -> Result<FileMetadata> {
    let metadata = fs::metadata(path)?;
    let modified = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let accessed = metadata
        .accessed()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    Ok(FileMetadata { modified, accessed })
}

/// Apply captured metadata to an output file.
pub fn apply_metadata(path: &Path, meta: &FileMetadata) -> Result<()> {
    let mtime = FileTime::from_unix_time(meta.modified as i64, 0);
    let atime = FileTime::from_unix_time(meta.accessed as i64, 0);

    set_file_mtime(path, mtime)?;
    set_file_atime(path, atime)?;

    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
