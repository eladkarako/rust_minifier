use crate::args::Args;
use crate::metadata::{apply_metadata, capture_metadata};
use crate::minifier;
use anyhow::Result;
use chrono::Local;
use is_terminal::IsTerminal;
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn process_all(args: &Args) -> Result<()> {
    let max_threads = num_cpus::get().saturating_mul(2).max(1);
    let semaphore =
        Arc::new(tokio::sync::Semaphore::new(max_threads));
    let stdout = Arc::new(Mutex::new(io::stdout()));
    let mut handles = vec![];
    let start_time = std::time::Instant::now();

    // Expand directories into file list
    let mut expanded_files = Vec::new();
    for file_arg in &args.files {
        expand_path(file_arg, &mut expanded_files)?;
    }

    if expanded_files.is_empty() {
        eprintln!("No .rs or .toml files found.");
        return Ok(());
    }

    // Determine output extension based on file types
    let output_extension =
        determine_output_extension(&expanded_files);

    // Prepare concat_to path
    let concat_to_destination = prepare_concat_destination(
        &args.concat_to,
        &output_extension,
    )?;

    let suffix = args.suffix.clone();
    let include_file_path = args.include_file_path;
    let num_files = expanded_files.len();

    for (idx, file) in expanded_files.iter().enumerate() {
        let sem = semaphore.clone();
        let file = file.clone();
        let suffix = suffix.clone();
        let concat_to_dest = concat_to_destination.clone();
        let stdout = stdout.clone();

        let handle = tokio::spawn({
            async move {
                let _permit = sem.acquire().await.ok();
                let file_start = std::time::Instant::now();
                let result = process_single_file(
                    &file,
                    &suffix,
                    concat_to_dest.as_deref(),
                    include_file_path,
                    stdout,
                )
                    .await;
                let elapsed = file_start.elapsed().as_millis();
                eprintln!(
                    "[{}/{}] {} ... {}ms",
                    idx + 1,
                    num_files,
                    file,
                    elapsed
                );
                result
            }
        });
        handles.push(handle);
    }

    let mut errors = vec![];
    for handle in handles {
        if let Err(e) = handle.await {
            errors.push(format!("Task error: {}", e));
        }
    }

    if !errors.is_empty() {
        eprintln!("\nErrors encountered:");
        for err in errors {
            eprintln!("  - {}", err);
        }
    }

    let elapsed = start_time.elapsed().as_millis();
    eprintln!("Total time: {}ms", elapsed);
    Ok(())
}

/// Recursively expand directories into .rs and .toml files
fn expand_path(
    path: &str,
    expanded: &mut Vec<String>,
) -> Result<()> {
    let p = Path::new(path);

    if p.is_dir() {
        walk_directory(p, expanded)?;
    } else if p.is_file() {
        let ext = p
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        if ext == "rs" || ext == "toml" {
            expanded.push(path.to_string());
        }
    } else {
        return Err(anyhow::anyhow!(
            "Path does not exist: {}",
            path
        ));
    }

    Ok(())
}

/// Recursively walk directory for .rs and .toml files
fn walk_directory(
    dir: &Path,
    expanded: &mut Vec<String>,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            walk_directory(&path, expanded)?;
        } else if path.is_file() {
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();
            if ext == "rs" || ext == "toml" {
                expanded.push(path.to_string_lossy().to_string());
            }
        }
    }
    Ok(())
}

/// Determine output file extension based on file types
fn determine_output_extension(files: &[String]) -> String {
    let mut has_rs = false;
    let mut has_toml = false;

    for file in files {
        let p = Path::new(file);
        let ext = p
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        if ext == "rs" {
            has_rs = true;
        } else if ext == "toml" {
            has_toml = true;
        }
    }

    if has_rs && has_toml {
        "._rs_and_toml.txt".to_string()
    } else if has_rs {
        ".rs".to_string()
    } else if has_toml {
        ".toml".to_string()
    } else {
        ".txt".to_string()
    }
}

/// Prepare the concat_to destination
/// Returns None for stdout, Some(path) for file or directory
fn prepare_concat_destination(
    concat_to: &str,
    ext: &str,
) -> Result<Option<String>> {
    let normalized = concat_to.trim().to_uppercase();

    if normalized == "STDOUT" {
        return Ok(None);
    }

    let dest_path = Path::new(concat_to);

    // If it's an existing directory or looks like a directory path
    if dest_path.exists() && dest_path.is_dir() {
        let timestamp =
            Local::now().format("%Y%m%d%H%M%S").to_string();
        let filename = format!("minified_{}{}", timestamp, ext);
        let full_path = dest_path.join(filename);
        return Ok(Some(full_path.to_string_lossy().to_string()));
    }

    // If it doesn't exist but parent does (treat as file path)
    if let Some(parent) = dest_path.parent() {
        if parent.exists() && parent.is_dir() {
            return Ok(Some(
                dest_path.to_string_lossy().to_string(),
            ));
        }
    }

    // If the path ends with a separator, treat as directory and create it
    if concat_to.ends_with('/') || concat_to.ends_with('\\') {
        fs::create_dir_all(dest_path)?;
        let timestamp =
            Local::now().format("%Y%m%d%H%M%S").to_string();
        let filename = format!("minified_{}{}", timestamp, ext);
        let full_path = dest_path.join(filename);
        return Ok(Some(full_path.to_string_lossy().to_string()));
    }

    // Otherwise, treat as a file path in the current directory
    Ok(Some(dest_path.to_string_lossy().to_string()))
}

#[cfg(target_os = "windows")]
const EOL: &str = "\r\n";
#[cfg(not(target_os = "windows"))]
const EOL: &str = "\n";

fn format_path_for_output(path: &str) -> String {
    if path.starts_with("\\\\?\\") {
        path.strip_prefix("\\\\?\\").unwrap_or(path).to_string()
    } else {
        path.to_string()
    }
}

async fn process_single_file<W: io::Write + Send + 'static>(
    file_path: &str,
    suffix: &str,
    concat_to_path: Option<&str>,
    include_file_path: bool,
    stdout: Arc<Mutex<W>>,
) -> Result<()> {
    let path = Path::new(file_path);
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let source = String::from_utf8(buffer)?;
    let meta = capture_metadata(path)?;
    let minified = minifier::minify(&source, path)?;

    let output = if include_file_path {
        let full_path = std::fs::canonicalize(path)?
            .to_string_lossy()
            .to_string();
        let formatted_path = format_path_for_output(&full_path);
        let comment_char =
            if file_path.ends_with(".toml") { "#" } else { "//" };
        format!(
            "{} {}{}{}{}{}",
            comment_char, formatted_path, EOL, minified, EOL, EOL
        )
    } else {
        minified
    };

    if let Some(concat_path) = concat_to_path {
        // Append to file
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(concat_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(output.as_bytes())?;
        writer.flush()?;
    } else {
        // Write to stdout
        let mut stdout_lock = stdout.lock().await;
        stdout_lock.write_all(output.as_bytes())?;
        stdout_lock.flush()?;
    }

    // Only apply metadata if writing individual files (not concatenating)
    if concat_to_path.is_none() {
        let output_path = get_deduplicated_path(path, suffix)?;
        let file = fs::File::create(&output_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(output.as_bytes())?;
        writer.flush()?;
        apply_metadata(&output_path, &meta)?;
    }

    Ok(())
}

fn get_deduplicated_path(
    original: &Path,
    suffix: &str,
) -> Result<PathBuf> {
    let parent = original
        .parent()
        .ok_or_else(|| anyhow::anyhow!("No parent directory"))?;
    let stem = original
        .file_stem()
        .ok_or_else(|| anyhow::anyhow!("No file stem"))?;
    let ext = original
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("rs");
    let mut output_path = parent.join(format!(
        "{}{}. {}",
        stem.to_string_lossy(),
        suffix,
        ext
    ));

    if output_path.exists() {
        let mut counter = 1;
        loop {
            output_path = parent.join(format!(
                "{}{}_{}.{}",
                stem.to_string_lossy(),
                suffix,
                counter,
                ext
            ));
            if !output_path.exists() {
                break;
            }
            counter += 1;
        }
    }
    Ok(output_path)
}

pub async fn process_stdin(_args: &Args) -> Result<()> {
    if io::stdin().is_terminal() {
        eprintln!(
            "No files provided and stdin is a terminal. Use --help for usage."
        );
        return Ok(());
    }
    let mut reader = BufReader::new(io::stdin().lock());
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let source = String::from_utf8(buffer)?;
    let dummy_path = Path::new("dummy.rs");
    let minified = minifier::minify(&source, dummy_path)?;
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writer.write_all(minified.as_bytes())?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
