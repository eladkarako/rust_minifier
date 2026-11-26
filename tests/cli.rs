//! End-to-end tests that exercise the compiled command line binary.

use std::fs;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use temp_dir::TempDir;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_rust_minifier")
}

fn run_stdin(arguments: &[&str], input: &str) -> Output {
    let mut child = Command::new(binary())
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start rust_minifier");
    child
        .stdin
        .as_mut()
        .expect("stdin pipe")
        .write_all(input.as_bytes())
        .expect("failed to write stdin");
    child.wait_with_output().expect("failed to wait")
}

#[test]
fn list_types_mentions_supported_and_excluded() {
    let output = Command::new(binary())
        .arg("--list-types")
        .output()
        .expect("failed to run --list-types");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Supported types"));
    assert!(stdout.contains("html"));
    assert!(stdout.contains("Excluded types"));
    assert!(stdout.contains("Python"));
}

#[test]
fn forces_language_for_stdin() {
    let output = run_stdin(
        &["--type", "html"],
        "<!-- comment --><div>  hi  </div>",
    );
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "<div>hi</div>"
    );
}

#[test]
fn short_type_flag_works() {
    let output = run_stdin(&["-t", "json"], "{ \"a\" : 1 }");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"a\":1}"
    );
}

#[test]
fn preserve_line_breaks_keeps_newlines() {
    let output = run_stdin(
        &["--type", "sql", "--preserve-line-breaks"],
        "SELECT 1;\nSELECT 2;",
    );
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "SELECT 1;\nSELECT 2;"
    );
}

#[test]
fn unknown_type_fails() {
    let output =
        run_stdin(&["--type", "not-a-real-language"], "x");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown language type"));
}

#[test]
fn minifies_file_and_writes_suffixed_output() {
    let directory = TempDir::new().unwrap();
    let source = directory.path().join("page.html");
    fs::write(&source, "<p>  hi  </p>").unwrap();

    let output = Command::new(binary())
        .arg("--no-include-file-path")
        .arg(&source)
        .output()
        .expect("failed to run");
    assert!(output.status.success());

    let minified = directory.path().join("page_minified.html");
    assert!(minified.exists(), "expected {:?}", minified);
    assert_eq!(
        fs::read_to_string(minified).unwrap(),
        "<p>hi</p>"
    );
}

#[test]
fn concat_output_uses_each_language_comment_style() {
    let directory = TempDir::new().unwrap();
    let html = directory.path().join("a.html");
    let css = directory.path().join("b.css");
    fs::write(&html, "<p>  hi  </p>").unwrap();
    fs::write(&css, "/* c */ .a { color: red; }").unwrap();
    let destination = directory.path().join("all.txt");

    let output = Command::new(binary())
        .arg(&html)
        .arg(&css)
        .arg("--concat-to")
        .arg(&destination)
        .output()
        .expect("failed to run");
    assert!(output.status.success());

    let text = fs::read_to_string(&destination).unwrap();
    assert!(text.contains("<!--"), "missing HTML comment: {}", text);
    assert!(text.contains("/*"), "missing CSS comment: {}", text);
    assert!(text.contains("<p>hi</p>"));
    assert!(text.contains(".a{color:red}"));
}

#[test]
fn unknown_extension_is_skipped() {
    let directory = TempDir::new().unwrap();
    let source = directory.path().join("image.png");
    fs::write(&source, [0u8, 1, 2, 3]).unwrap();

    let output = Command::new(binary())
        .arg(&source)
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No supported files found"));
}
