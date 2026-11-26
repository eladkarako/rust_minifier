//! Profiles for the line preserving minifier.
//!
//! These formats cannot be joined into one line because newlines or
//! indentation carry meaning. The profile only describes which comments to
//! remove and how carefully to treat continuations, heredocs and block scalars.

/// Syntax description for one line oriented language.
#[derive(Debug, Clone, Copy)]
pub struct LineProfile {
    /// Markers that begin a single line comment.
    pub line_comments: &'static [&'static str],
    /// Only treat a marker as a comment when it starts the line.
    pub line_comment_line_start_only: bool,
    /// Require whitespace before the marker (so `a#b` is not a comment).
    pub line_comment_requires_boundary: bool,
    /// Case insensitive words that start a comment at the beginning of a line.
    pub line_start_words: &'static [&'static str],
    /// Multi line comment delimiters.
    pub block_comments: &'static [(&'static str, &'static str)],
    /// Characters that open a quoted string.
    pub quote_chars: &'static [char],
    /// Whether a backslash escapes the next character inside strings.
    pub backslash_escapes: bool,
    /// Join lines that end with a backslash.
    pub join_backslash_continuations: bool,
    /// Keep the indentation of each line.
    pub preserve_leading_whitespace: bool,
    /// Drop lines that become empty after comment removal.
    pub remove_blank_lines: bool,
    /// Understand shell style `<<EOF` heredocs.
    pub heredoc: bool,
    /// Understand YAML `|` and `>` block scalars.
    pub yaml_block_scalar: bool,
}

const BASE: LineProfile = LineProfile {
    line_comments: &["#"],
    line_comment_line_start_only: false,
    line_comment_requires_boundary: true,
    line_start_words: &[],
    block_comments: &[],
    quote_chars: &['"', '\''],
    backslash_escapes: true,
    join_backslash_continuations: false,
    preserve_leading_whitespace: true,
    remove_blank_lines: true,
    heredoc: false,
    yaml_block_scalar: false,
};

pub static SHELL: LineProfile = LineProfile {
    heredoc: true,
    ..BASE
};

pub static YAML: LineProfile = LineProfile {
    backslash_escapes: false,
    yaml_block_scalar: true,
    ..BASE
};

pub static INI: LineProfile = LineProfile {
    line_comments: &[";", "#", "!"],
    line_comment_line_start_only: true,
    ..BASE
};

pub static PROPERTIES: LineProfile = LineProfile {
    line_comments: &["#", "!"],
    line_comment_line_start_only: true,
    join_backslash_continuations: true,
    ..BASE
};

pub static DOCKERFILE: LineProfile = LineProfile {
    line_comment_line_start_only: true,
    ..BASE
};

pub static MAKEFILE: LineProfile = LineProfile {
    join_backslash_continuations: true,
    ..BASE
};

pub static CMAKE: LineProfile = LineProfile {
    quote_chars: &['"'],
    ..BASE
};

pub static HCL: LineProfile = LineProfile {
    line_comments: &["#", "//"],
    block_comments: &[("/*", "*/")],
    ..BASE
};

pub static NIX: LineProfile = LineProfile {
    block_comments: &[("/*", "*/")],
    ..BASE
};

pub static GITIGNORE: LineProfile = LineProfile {
    line_comment_line_start_only: true,
    ..BASE
};

pub static PROGUARD: LineProfile = LineProfile { ..BASE };

pub static PO: LineProfile = LineProfile {
    line_comment_line_start_only: true,
    ..BASE
};

pub static PRISMA: LineProfile = LineProfile {
    line_comments: &["//"],
    line_comment_requires_boundary: false,
    quote_chars: &['"'],
    ..BASE
};

pub static ASSEMBLY: LineProfile = LineProfile {
    line_comments: &[";"],
    line_comment_requires_boundary: false,
    ..BASE
};

pub static WAT: LineProfile = LineProfile {
    line_comments: &[";;"],
    line_comment_requires_boundary: false,
    block_comments: &[("(;", ";)")],
    ..BASE
};

pub static HASKELL: LineProfile = LineProfile {
    line_comments: &["--"],
    block_comments: &[("{-", "-}")],
    ..BASE
};

pub static FORTRAN: LineProfile = LineProfile {
    line_comments: &["!"],
    line_comment_requires_boundary: false,
    ..BASE
};

pub static VISUAL_BASIC: LineProfile = LineProfile {
    line_comments: &["'"],
    line_comment_requires_boundary: false,
    quote_chars: &['"'],
    ..BASE
};

pub static BATCH: LineProfile = LineProfile {
    line_comments: &["::"],
    line_comment_line_start_only: true,
    line_start_words: &["REM"],
    quote_chars: &['"'],
    ..BASE
};

pub static POWERSHELL: LineProfile = LineProfile {
    block_comments: &[("<#", "#>")],
    ..BASE
};

pub static VIM: LineProfile = LineProfile {
    line_comments: &["\""],
    line_comment_line_start_only: true,
    ..BASE
};

pub static PERL: LineProfile = LineProfile {
    heredoc: true,
    ..BASE
};

pub static RUBY: LineProfile = LineProfile { ..BASE };

pub static R: LineProfile = LineProfile { ..BASE };

pub static JULIA: LineProfile = LineProfile {
    block_comments: &[("#=", "=#")],
    ..BASE
};

pub static TCL: LineProfile = LineProfile {
    line_comment_line_start_only: true,
    ..BASE
};

pub static AWK: LineProfile = LineProfile { ..BASE };

pub static NIM: LineProfile = LineProfile {
    block_comments: &[("#[", "]#")],
    ..BASE
};

/// Return the profile for a line based language id.
pub fn profile_for(
    language_id: &str
) -> Option<&'static LineProfile> {
    match language_id {
        "shell" => Some(&SHELL),
        "yaml" => Some(&YAML),
        "ini" | "browserslist" | "procfile" => Some(&INI),
        "properties" => Some(&PROPERTIES),
        "dockerfile" => Some(&DOCKERFILE),
        "makefile" => Some(&MAKEFILE),
        "cmake" => Some(&CMAKE),
        "hcl" => Some(&HCL),
        "nix" | "just" | "ninja" => Some(&NIX),
        "gitignore" => Some(&GITIGNORE),
        "proguard" | "smali" => Some(&PROGUARD),
        "po" => Some(&PO),
        "prisma" => Some(&PRISMA),
        "assembly" => Some(&ASSEMBLY),
        "wat" => Some(&WAT),
        "haskell" => Some(&HASKELL),
        "fortran" => Some(&FORTRAN),
        "visualbasic" => Some(&VISUAL_BASIC),
        "batch" => Some(&BATCH),
        "powershell" => Some(&POWERSHELL),
        "vim" => Some(&VIM),
        "perl" => Some(&PERL),
        "ruby" => Some(&RUBY),
        "r" => Some(&R),
        "julia" => Some(&JULIA),
        "tcl" => Some(&TCL),
        "awk" => Some(&AWK),
        "nim" => Some(&NIM),
        _ => None,
    }
}
