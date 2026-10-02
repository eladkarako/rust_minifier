use anyhow::Result;
use is_terminal::IsTerminal;
use std::fs;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::args::Args;
use crate::metadata::{apply_metadata, capture_metadata};
use crate::minifier::minify;

/// Process all files with Tokio parallelism.
pub async fn process_all(args: &Args) -> Result<()> {
    let max_threads = num_cpus::get().saturating_mul(2).max(1);
    let semaphore =
        Arc::new(tokio::sync::Semaphore::new(max_threads));

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let mut handles = vec![];

    let start_time = std::time::Instant::now();

    for (idx, file) in args.files.iter().enumerate() {
        let sem = semaphore.clone();
        let file = file.clone();
        let _suffix = args.suffix.clone();
        let _concat_output = args.concat_output.clone();
        let _concat_to_stdout = args.concat_to_stdout;
        let stdout = stdout.clone();
        let handle = tokio::spawn({
            let args = args.clone();
            async move {
                let _permit = sem.acquire().await.ok();
                let file_start = std::time::Instant::now();
                let result = process_single_file(
                    &file,
                    &args.suffix,
                    args.concat_output.as_deref(),
                    args.concat_to_stdout,
                    stdout,
                )
                    .await;
                let elapsed = file_start.elapsed().as_millis();
                eprintln!(
                    "[{}/{}] {} ... {}ms",
                    idx + 1,
                    args.files.len(),
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

/// Process a single file.
async fn process_single_file(
    file_path: &str,
    suffix: &str,
    concat_output: Option<&str>,
    concat_to_stdout: bool,
    stdout: Arc<Mutex<io::Stdout>>,
) -> Result<()> {
    let path = Path::new(file_path);

    // Read file with buffered reader
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let source = String::from_utf8(buffer)?;

    // Capture metadata before modification
    let meta = capture_metadata(path)?;

    // Minify
    let minified = minify(&source)?;

    if concat_to_stdout {
        // Write to stdout as thread finishes
        let mut stdout_lock = stdout.lock().await;
        stdout_lock.write_all(minified.as_bytes())?;
        stdout_lock.flush()?;
    } else if let Some(concat_path) = concat_output {
        // Append to concatenation file (handle locking separately)
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(concat_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(minified.as_bytes())?;
        writer.flush()?;
    } else {
        // Write to individual output file
        let output_path = get_deduplicated_path(path, suffix)?;
        let file = fs::File::create(&output_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(minified.as_bytes())?;
        writer.flush()?;

        // Restore metadata
        apply_metadata(&output_path, &meta)?;
    }

    Ok(())
}

/// Get deduplicated output file path.
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

    // First, try the path without a counter
    let mut output_path = parent.join(format!(
        "{}{}.rs",
        stem.to_string_lossy(),
        suffix
    ));

    // If it exists, add counters: _1, _2, _3, etc.
    if output_path.exists() {
        let mut counter = 1;
        loop {
            output_path = parent.join(format!(
                "{}{}_{}.rs",
                stem.to_string_lossy(),
                suffix,
                counter
            ));
            if !output_path.exists() {
                break;
            }
            counter += 1;
        }
    }

    Ok(output_path)
}

/// Process stdin.
pub async fn process_stdin(_args: &Args) -> Result<()> {
    // Check if stdin is a terminal
    if io::stdin().is_terminal() {
        eprintln!(
            "No files provided and stdin is a terminal. Use --help for usage."
        );
        return Ok(());
    }

    // Read from stdin with buffered reader
    let mut reader = BufReader::new(io::stdin().lock());
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let source = String::from_utf8(buffer)?;

    // Minify
    let minified = minify(&source)?;

    // Write to stdout
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writer.write_all(minified.as_bytes())?;
    writer.flush()?;

    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
