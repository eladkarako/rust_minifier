use crate::args::Args;
use crate::metadata::{apply_metadata, capture_metadata};
use crate::minifier;
use crate::minifier::language::{self, Language};
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
    language: &'static Language,
    minified_content: String,
    // include_file_path is already applied to minified_content, no need to store it
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

fn parse_soft_split(soft_split: &str) -> Result<Option<usize>> {
    let normalized = soft_split.trim().to_lowercase();
    if normalized == "unlimited" || normalized.is_empty() {
        Ok(None)
    } else {
        normalized
            .parse::<usize>()
            .map(Some)
            .map_err(|_| anyhow::anyhow!("Invalid soft_split value: {}. Use 'unlimited' or a positive number.", soft_split))
    }
}

fn soft_split_content(
    content: &str,
    max_chars: usize,
) -> Vec<String> {
    if content.len() <= max_chars {
        return vec![content.to_string()];
    }

    // Split by the minified file separator (3 newlines)
    let chunks: Vec<&str> = content.split(SEPARATOR).collect();

    let mut parts = Vec::new();
    let mut current_part = String::new();

    for (idx, chunk) in chunks.iter().enumerate() {
        let chunk_with_sep = if idx == 0 {
            chunk.to_string()
        } else {
            format!("{}{}", SEPARATOR, chunk)
        };

        if current_part.is_empty() {
            current_part.push_str(&chunk_with_sep);
        } else if (current_part.len() + chunk_with_sep.len())
            <= max_chars
        {
            current_part.push_str(&chunk_with_sep);
        } else {
            // Would exceed limit, save current and start new
            parts.push(current_part);
            current_part = chunk_with_sep;
        }
    }

    if !current_part.is_empty() {
        parts.push(current_part);
    }

    parts
}

pub async fn process_all(args: &Args) -> Result<()> {
    // Print arguments
    eprintln!("━━━ Arguments ━━━");
    eprintln!("  Files:              {:?}", args.files);
    eprintln!("  Suffix:             {}", args.suffix);
    eprintln!("  Concat to:          {}", args.concat_to);
    eprintln!(
        "  Include file path:  {}",
        args.include_source_path()
    );
    eprintln!("  Soft split:         {}", args.soft_split);
    eprintln!("  Preserve newlines:  {}", args.preserve_line_breaks);
    if let Some(ref forced) = args.forced_type {
        eprintln!("  Forced type:        {}", forced);
    }
    eprintln!();

    let forced_language = match args.forced_type.as_deref() {
        Some(name) => Some(
            language::find_language(name).ok_or_else(|| {
                anyhow::anyhow!(
                    "Unknown language type: {}. Use --list-types to see supported values.",
                    name
                )
            })?,
        ),
        None => None,
    };
    let minifier_options = args.minifier_options();

    let soft_split_limit = parse_soft_split(&args.soft_split)?;

    let max_threads = num_cpus::get().saturating_mul(2).max(1);
    let semaphore =
        Arc::new(tokio::sync::Semaphore::new(max_threads));
    let stdout = Arc::new(Mutex::new(io::stdout()));
    let mut handles = vec![];
    let start_time = std::time::Instant::now();

    let mut expanded_files = Vec::new();
    for file_arg in &args.files {
        expand_path(
            file_arg,
            &mut expanded_files,
            &args.exclude,
            forced_language.is_some(),
        )?;
    }

    if expanded_files.is_empty() {
        eprintln!("No supported files found.");
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
    let include_file_path = args.include_source_path();
    let num_files = expanded_files.len();

    for (idx, file) in expanded_files.iter().enumerate() {
        let sem = semaphore.clone();
        let file = file.clone();
        let suffix = suffix.clone();
        let concat_to_dest = concat_to_destination.clone();
        let stdout = stdout.clone();
        let minifier_options = minifier_options.clone();

        let handle = tokio::spawn({
            async move {
                let _permit = sem.acquire().await.ok();
                let file_start = std::time::Instant::now();
                let result = process_single_file(
                    &file,
                    &suffix,
                    concat_to_dest.as_deref(),
                    include_file_path,
                    soft_split_limit,
                    stdout,
                    forced_language,
                    minifier_options,
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
        sorted.sort_by(|a, b| {
            match a
                .language
                .category
                .cmp(&b.language.category)
            {
                std::cmp::Ordering::Equal => {
                    alphanumeric_sort::compare_str(
                        &a.file_path,
                        &b.file_path,
                    )
                }
                other => other,
            }
        });

        for (i, pf) in sorted.iter().enumerate() {
            eprintln!(
                "  [{}] ({}) {}",
                i + 1,
                pf.language.display_name,
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

    // Sort: first by language category, then by file path using
    // natural/alphanumeric ordering (UTF-8 aware).
    sorted.sort_by(|a, b| {
        match a
            .language
            .category
            .cmp(&b.language.category)
        {
            std::cmp::Ordering::Equal => {
                compare_str(&a.file_path, &b.file_path)
            }
            other => other,
        }
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
    accept_any_file: bool,
) -> Result<()> {
    let p = Path::new(path);
    if p.is_dir() {
        walk_directory(p, expanded, exclude_patterns)?;
    } else if p.is_file() {
        let supported = accept_any_file
            || language::detect_language(p).is_some();
        if supported && !should_exclude_file(path, exclude_patterns)
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

/// Recursively walk a directory for supported text files.
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
            let path_str = path.to_string_lossy().to_string();
            if language::detect_language(&path).is_some()
                && !should_exclude_file(&path_str, exclude_patterns)
            {
                expanded.push(path_str);
            }
        }
    }
    Ok(())
}

/// Determine output file extension based on the languages in the file list.
fn determine_output_extension(files: &[String]) -> String {
    let mut languages: Vec<&'static Language> = Vec::new();
    for file in files {
        if let Some(language) =
            language::detect_language(Path::new(file))
        {
            if !languages
                .iter()
                .any(|known| known.id == language.id)
            {
                languages.push(language);
            }
        }
    }

    match languages.as_slice() {
        [] => ".txt".to_string(),
        [only] => format!(
            ".{}",
            only.extensions
                .first()
                .copied()
                .unwrap_or("txt")
        ),
        [first, second] => format!(
            "._{}_and_{}.txt",
            first
                .extensions
                .first()
                .copied()
                .unwrap_or(first.id),
            second
                .extensions
                .first()
                .copied()
                .unwrap_or(second.id)
        ),
        _ => "._mixed.txt".to_string(),
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
        let timestamp = Local::now()
            .format("%Y%m%d%H%M%S")
            .to_string();
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
        let timestamp = Local::now()
            .format("%Y%m%d%H%M%S")
            .to_string();
        let filename = format!("minified_{}{}", timestamp, ext);
        let full_path = dest_path.join(filename);
        return Ok(Some(full_path.to_string_lossy().to_string()));
    }

    // Otherwise, treat as a file path in the current directory
    Ok(Some(dest_path.to_string_lossy().to_string()))
}

#[cfg(target_os = "windows")]
const EOL: &str = "\r\n";

#[cfg(target_os = "windows")]
const SEPARATOR: &str = "\r\n\r\n\r\n";

#[cfg(not(target_os = "windows"))]
const EOL: &str = "\n";

#[cfg(not(target_os = "windows"))]
const SEPARATOR: &str = "\n\n\n";

fn format_path_for_output(path: &str) -> String {
    if path.starts_with("\\\\?\\") {
        path.strip_prefix("\\\\?\\")
            .unwrap_or(path)
            .to_string()
    } else {
        path.to_string()
    }
}

async fn process_single_file<W: io::Write + Send + 'static>(
    file_path: &str,
    suffix: &str,
    concat_to_path: Option<&str>,
    include_file_path: bool,
    soft_split_limit: Option<usize>,
    stdout: Arc<Mutex<W>>,
    forced_language: Option<&'static Language>,
    options: minifier::MinifierOptions,
) -> Result<Option<ProcessedFile>> {
    let path = Path::new(file_path);
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let source = String::from_utf8(buffer)?;
    let meta = capture_metadata(path)?;

    // Ambiguous extensions (for example MATLAB vs Objective-C `.m`) are
    // resolved from the contents. When the contents do not match any
    // supported language the file is skipped.
    let language = match forced_language {
        Some(language) => language,
        None => match minifier::detect_language(path, &source) {
            Some(language) => language,
            None => return Ok(None),
        },
    };

    let minified =
        minifier::minify_with_language(&source, language, &options)?;

    let output = if include_file_path {
        let full_path = std::fs::canonicalize(path)?
            .to_string_lossy()
            .to_string();
        let formatted_path = format_path_for_output(&full_path);
        match language.comment.render(&formatted_path) {
            Some(comment) => format!(
                "{}{}{}{}",
                comment, EOL, minified, SEPARATOR
            ),
            None => format!("{}{}", minified, SEPARATOR),
        }
    } else {
        minified
    };

    // If concatenating, return the result to be collected and sorted
    if let Some(_concat_path) = concat_to_path {
        return Ok(Some(ProcessedFile {
            file_path: file_path.to_string(),
            language,
            minified_content: output,
        }));
    }

    let parts = if let Some(limit) = soft_split_limit {
        soft_split_content(&output, limit)
    } else {
        vec![output]
    };

    if parts.len() == 1 {
        let output_path = get_deduplicated_path(path, suffix)?;
        let file = fs::File::create(&output_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(parts[0].as_bytes())?;
        writer.flush()?;
        apply_metadata(&output_path, &meta)?;

        let mut stdout_lock = stdout.lock().await;
        stdout_lock.write_all(parts[0].as_bytes())?;
        stdout_lock.flush()?;
    } else {
        let total_parts = parts.len();
        for (idx, part) in parts.iter().enumerate() {
            let part_num = idx + 1;
            let part_suffix = format!(
                "{}_part_{:02}_{:02}",
                suffix, part_num, total_parts
            );
            let output_path =
                get_deduplicated_path(path, &part_suffix)?;
            let file = fs::File::create(&output_path)?;
            let mut writer = BufWriter::new(file);
            writer.write_all(part.as_bytes())?;
            writer.flush()?;
            apply_metadata(&output_path, &meta)?;

            // Write to stdout if no concat_to
            let mut stdout_lock = stdout.lock().await;
            stdout_lock.write_all(part.as_bytes())?;
            stdout_lock.flush()?;
        }
    }

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

pub async fn process_stdin(args: &Args) -> Result<()> {
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

    let language = match args.forced_type.as_deref() {
        Some(name) => language::find_language(name).ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown language type: {}. Use --list-types to see supported values.",
                name
            )
        })?,
        None => language::find_language("rust").expect("rust is registered"),
    };
    let minified = minifier::minify_with_language(
        &source,
        language,
        &args.minifier_options(),
    )?;

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writer.write_all(minified.as_bytes())?;
    writer.flush()?;

    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
