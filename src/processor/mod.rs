use crate::args::Args;
use crate::metadata::{apply_metadata, capture_metadata};
use crate::minifier;
use alphanumeric_sort::compare_str;
use anyhow::Result;
use chrono::Local;
use is_terminal::IsTerminal;
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct ProcessedFile {
    file_path: String,
    file_type: FileType,
    minified_content: String,
    // include_file_path is already applied to minified_content, no need to store it
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum FileType {
    Toml,
    Rust,
}

impl FileType {
    fn from_path(path: &str) -> Self {
        if path.ends_with(".toml") {
            FileType::Toml
        } else {
            FileType::Rust
        }
    }
}

fn should_exclude_file(
    file_path: &str,
    exclude_patterns: &[String],
) -> bool {
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    for pattern in exclude_patterns {
        if file_name.contains(&pattern.to_lowercase()) {
            return true;
        }
    }
    false
}

pub async fn process_all(args: &Args) -> Result<()> {
    // Print arguments
    eprintln!("━━━ Arguments ━━━");
    eprintln!("  Files:              {:?}", args.files);
    eprintln!("  Suffix:             {}", args.suffix);
    eprintln!("  Concat to:          {}", args.concat_to);
    eprintln!("  Include file path:  {}", args.include_file_path);
    eprintln!();

    let max_threads = num_cpus::get().saturating_mul(2).max(1);
    let semaphore =
        Arc::new(tokio::sync::Semaphore::new(max_threads));
    let stdout = Arc::new(Mutex::new(io::stdout()));
    let mut handles = vec![];
    let start_time = std::time::Instant::now();

    let mut expanded_files = Vec::new();
    for file_arg in &args.files {
        expand_path(file_arg, &mut expanded_files, &args.exclude)?;
    }

    if expanded_files.is_empty() {
        eprintln!("No .rs or .toml files found.");
        return Ok(());
    }

    // Print expanded files
    eprintln!("━━━ Found {} Files ━━━", expanded_files.len());
    for (i, file) in expanded_files.iter().enumerate() {
        eprintln!("  [{}] {}", i + 1, file);
    }
    eprintln!();

    let output_extension =
        determine_output_extension(&expanded_files);
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

    let mut processed_files: Vec<ProcessedFile> = vec![];
    let mut errors = vec![];

    for handle in handles {
        match handle.await {
            Ok(Ok(Some(pf))) => processed_files.push(pf),
            Ok(Ok(None)) => {}
            Ok(Err(e)) => errors.push(e.to_string()),
            Err(e) => errors.push(format!("Task error: {}", e)),
        }
    }

    // If concat_to was specified, show sorted order before writing
    if let Some(ref dest) = concat_to_destination {
        eprintln!("━━━ Concatenated Output Order ━━━");
        eprintln!("  Destination: {}", dest);
        eprintln!();

        // Create sorted copy to show order
        let mut sorted = processed_files.clone();
        sorted.sort_by(|a, b| match a.file_type.cmp(&b.file_type) {
            std::cmp::Ordering::Equal => {
                alphanumeric_sort::compare_str(
                    &a.file_path,
                    &b.file_path,
                )
            }
            other => other,
        });

        for (i, pf) in sorted.iter().enumerate() {
            let file_type_str = match pf.file_type {
                FileType::Toml => "TOML",
                FileType::Rust => "Rust",
            };
            eprintln!(
                "  [{}] ({}) {}",
                i + 1,
                file_type_str,
                pf.file_path
            );
        }
        eprintln!();

        sort_and_write_concatenated(&processed_files, dest)?;
    }

    if !errors.is_empty() {
        eprintln!("\nErrors encountered:");
        for err in errors {
            eprintln!("  - {}", err);
        }
    }

    let elapsed = start_time.elapsed().as_millis();
    eprintln!("━━━ Summary ━━━");
    eprintln!("  Total time: {}ms", elapsed);
    Ok(())
}

fn sort_and_write_concatenated(
    files: &[ProcessedFile],
    dest: &str,
) -> Result<()> {
    let mut sorted = files.to_vec();

    // Sort: first by file type (Toml before Rust),
    // then by file path using natural/alphanumeric ordering (UTF-8 aware)
    sorted.sort_by(|a, b| match a.file_type.cmp(&b.file_type) {
        std::cmp::Ordering::Equal => {
            compare_str(&a.file_path, &b.file_path)
        }
        other => other,
    });

    let file = fs::File::create(dest)?;
    let mut writer = BufWriter::new(file);

    for pf in sorted {
        writer.write_all(pf.minified_content.as_bytes())?;
    }

    writer.flush()?;
    Ok(())
}

fn expand_path(
    path: &str,
    expanded: &mut Vec<String>,
    exclude_patterns: &[String],
) -> Result<()> {
    let p = Path::new(path);
    if p.is_dir() {
        walk_directory(p, expanded, exclude_patterns)?;
    } else if p.is_file() {
        let ext = p
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        if (ext == "rs" || ext == "toml")
            && !should_exclude_file(path, exclude_patterns)
        {
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
    exclude_patterns: &[String],
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk_directory(&path, expanded, exclude_patterns)?;
        } else if path.is_file() {
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();
            let path_str = path.to_string_lossy().to_string();
            if (ext == "rs" || ext == "toml")
                && !should_exclude_file(&path_str, exclude_patterns)
            {
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
) -> Result<Option<ProcessedFile>> {
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

    // If concatenating, return the result to be collected and sorted
    if let Some(_concat_path) = concat_to_path {
        return Ok(Some(ProcessedFile {
            file_path: file_path.to_string(),
            file_type: FileType::from_path(file_path),
            minified_content: output,
            // Remove: include_file_path,
        }));
    }

    // Otherwise, write individual minified file
    let output_path = get_deduplicated_path(path, suffix)?;
    let file = fs::File::create(&output_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(output.as_bytes())?;
    writer.flush()?;
    apply_metadata(&output_path, &meta)?;

    // Write to stdout if no concat_to
    let mut stdout_lock = stdout.lock().await;
    stdout_lock.write_all(output.as_bytes())?;
    stdout_lock.flush()?;

    Ok(None)
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
        "{}{}.{}",
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
