//! Unit tests for the line preserving minifier.

use super::*;

fn minify(source: &str, profile: &LineProfile) -> String {
    minify_line_based(source, profile, &MinifierOptions::default())
}

#[test]
fn shell_removes_comments_and_keeps_shebang() {
    let source = "#!/bin/bash\n# comment\necho hello   # trailing\n";
    assert_eq!(
        minify(source, &profile::SHELL),
        "#!/bin/bash\necho hello"
    );
}

#[test]
fn shell_preserves_heredoc() {
    let source = "cat <<EOF\n# not a comment\nEOF\necho done\n";
    assert_eq!(
        minify(source, &profile::SHELL),
        "cat <<EOF\n# not a comment\nEOF\necho done"
    );
}

#[test]
fn yaml_removes_comments() {
    let source = "# c\nkey: value # trailing\nlist:\n  - a\n  - b\n";
    assert_eq!(
        minify(source, &profile::YAML),
        "key: value\nlist:\n  - a\n  - b"
    );
}

#[test]
fn yaml_preserves_block_scalar() {
    let source = "key: |\n  line one # not comment\n  # also not comment\nnext: 1\n";
    assert_eq!(
        minify(source, &profile::YAML),
        "key: |\n  line one # not comment\n  # also not comment\nnext: 1"
    );
}

#[test]
fn yaml_removes_blank_lines() {
    assert_eq!(
        minify("a: 1\n\n\nb: 2\n", &profile::YAML),
        "a: 1\nb: 2"
    );
}

#[test]
fn ini_only_removes_line_start_comments() {
    let source = "; comment\nkey = value ; inline\n# comment\n";
    assert_eq!(
        minify(source, &profile::INI),
        "key = value ; inline"
    );
}

#[test]
fn properties_joins_continuations() {
    let source = "key = a \\\n  b\n# c\n";
    assert_eq!(minify(source, &profile::PROPERTIES), "key = a b");
}

#[test]
fn dockerfile_only_removes_full_line_comments() {
    let source = "# comment\nFROM alpine # not a comment\nRUN echo hi\n";
    assert_eq!(
        minify(source, &profile::DOCKERFILE),
        "FROM alpine # not a comment\nRUN echo hi"
    );
}

#[test]
fn makefile_preserves_tabs() {
    let source = "# c\nall:\n\t@echo hi # tabbed\n";
    assert_eq!(
        minify(source, &profile::MAKEFILE),
        "all:\n\t@echo hi"
    );
}

#[test]
fn prisma_removes_double_slash_comments() {
    let source = "// c\nmodel User {\n  id Int // trailing\n}\n";
    assert_eq!(
        minify(source, &profile::PRISMA),
        "model User {\n  id Int\n}"
    );
}

#[test]
fn assembly_removes_semicolon_comments() {
    let source = "; c\nmov eax, 1 ; trailing\n";
    assert_eq!(minify(source, &profile::ASSEMBLY), "mov eax, 1");
}

#[test]
fn powershell_removes_block_comments() {
    let source = "# c\n<# block\nstill #>Write-Host hi\n";
    assert_eq!(minify(source, &profile::POWERSHELL), "Write-Host hi");
}

#[test]
fn batch_removes_rem_and_colon_comments() {
    let source = "REM comment\n:: comment\necho hi\n";
    assert_eq!(minify(source, &profile::BATCH), "echo hi");
}

#[test]
fn vim_only_removes_line_start_quote() {
    let source = "\" comment\nlet x = \"abc\"\n";
    assert_eq!(
        minify(source, &profile::VIM),
        "let x = \"abc\""
    );
}

#[test]
fn hcl_removes_hash_slash_and_block_comments() {
    let source =
        "# c\n// c2\nkey = \"a#b\" // trailing\n/* block\nend */\nvalue = 1\n";
    assert_eq!(
        minify(source, &profile::HCL),
        "key = \"a#b\"\nvalue = 1"
    );
}

#[test]
fn wat_removes_line_comments() {
    assert_eq!(
        minify(";; c\n(module)\n", &profile::WAT),
        "(module)"
    );
}

#[test]
fn wat_removes_inline_block_comments() {
    let result =
        minify("(module (; x ;) (func))\n", &profile::WAT);
    assert!(!result.contains('x'));
    assert!(result.starts_with("(module"));
    assert!(result.contains("(func))"));
}

#[test]
fn haskell_removes_dash_comments() {
    let source = "-- c\nmain = putStrLn \"a--b\"\n";
    assert_eq!(
        minify(source, &profile::HASKELL),
        "main = putStrLn \"a--b\""
    );
}

#[test]
fn fortran_removes_bang_comments() {
    let source = "! c\nx = 1 ! trailing\n";
    assert_eq!(minify(source, &profile::FORTRAN), "x = 1");
}

#[test]
fn visual_basic_removes_apostrophe_comments() {
    let source = "' c\nDim x = \"a'b\"\n";
    assert_eq!(
        minify(source, &profile::VISUAL_BASIC),
        "Dim x = \"a'b\""
    );
}

#[test]
fn perl_preserves_heredoc() {
    let source = "my $x = <<'END';\n# not comment\nEND\nprint $x;\n";
    assert_eq!(
        minify(source, &profile::PERL),
        "my $x = <<'END';\n# not comment\nEND\nprint $x;"
    );
}

#[test]
fn julia_removes_block_comments() {
    let source = "#= c =#\nx = 1\n";
    assert_eq!(minify(source, &profile::JULIA), "x = 1");
}

#[test]
fn nix_removes_hash_comments() {
    let source = "# c\n{ pkgs }: pkgs.hello\n";
    assert_eq!(
        minify(source, &profile::NIX),
        "{ pkgs }: pkgs.hello"
    );
}

#[test]
fn tcl_only_removes_line_start_comments() {
    let source = "# c\nset x \"#value\"\n";
    assert_eq!(
        minify(source, &profile::TCL),
        "set x \"#value\""
    );
}

#[test]
fn trims_trailing_whitespace() {
    assert_eq!(
        minify("a = 1   \n", &profile::INI),
        "a = 1"
    );
}

#[test]
fn preserves_indentation() {
    let source = "root:\n  child:\n    value: 1\n";
    assert_eq!(
        minify(source, &profile::YAML),
        "root:\n  child:\n    value: 1"
    );
}

#[test]
fn empty_input_returns_empty() {
    assert_eq!(minify("   \n\t\n", &profile::YAML), "");
}

#[test]
fn removes_bom() {
    assert_eq!(
        minify("\u{feff}key: 1\n", &profile::YAML),
        "key: 1"
    );
}

#[test]
fn dockerfile_syntax_directive_preserved() {
    let source = "# syntax=docker/dockerfile:1\nFROM alpine\n";
    assert_eq!(
        minify(source, &profile::DOCKERFILE),
        "# syntax=docker/dockerfile:1\nFROM alpine"
    );
}
