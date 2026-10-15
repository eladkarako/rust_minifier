use anyhow::Result;
use is_terminal::IsTerminal;
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::args::Args;
use crate::metadata::{apply_metadata, capture_metadata};
use crate::minifier;

pub async fn process_all(args: &Args) -> Result<()> {
    let max_threads = num_cpus::get().saturating_mul(2).max(1);
    let semaphore =
        Arc::new(tokio::sync::Semaphore::new(max_threads));
    let stdout = Arc::new(Mutex::new(io::stdout()));
    let mut handles = vec![];
    let start_time = std::time::Instant::now();

    // Clone args fields once, outside the loop
    let suffix = args.suffix.clone();
    let concat_output = args.concat_output.clone();
    let concat_to_stdout = args.concat_to_stdout;
    let include_file_path = args.include_file_path;
    let num_files = args.files.len();

    for (idx, file) in args.files.iter().enumerate() {
        let sem = semaphore.clone();
        let file = file.clone();
        let suffix = suffix.clone();
        let concat_output = concat_output.clone();
        let stdout = stdout.clone();

        let handle = tokio::spawn({
            async move {
                let _permit = sem.acquire().await.ok();
                let file_start = std::time::Instant::now();
                let result = process_single_file(
                    &file,
                    &suffix,
                    concat_output.as_deref(),
                    concat_to_stdout,
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
    concat_output: Option<&str>,
    concat_to_stdout: bool,
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

    // Auto-detect file type and minify
    let minified = minifier::minify(&source, path)?;

    let output = if include_file_path {
        let full_path = std::fs::canonicalize(path)?
            .to_string_lossy()
            .to_string();
        let formatted_path = format_path_for_output(&full_path);

        // Use appropriate comment syntax based on file type
        let comment_char =
            if file_path.ends_with(".toml") { "#" } else { "//" };
        format!(
            "{} {}{}{}",
            comment_char, formatted_path, EOL, minified
        )
    } else {
        minified
    };

    if concat_to_stdout {
        let mut stdout_lock = stdout.lock().await;
        stdout_lock.write_all(output.as_bytes())?;
        stdout_lock.flush()?;
    } else if let Some(concat_path) = concat_output {
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(concat_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(output.as_bytes())?;
        writer.flush()?;
    } else {
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

    // Detect original extension to preserve it
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

    // For stdin, default to Rust unless we can detect otherwise
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
