//! Profiles for the configurable C-like minifier.
//!
//! A profile describes comments, string syntax, preprocessor rules and whether
//! newlines need a safe statement separator. Keeping this data separate makes
//! the lexer small and makes adding a language a matter of adding a constant.

/// How newlines should be treated when joining tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatementSeparator {
    /// Newlines are ordinary whitespace. Used by languages that require
    /// semicolons and by whitespace-insensitive languages.
    None,
    /// A newline can end a statement. A safe `;` is inserted when the next
    /// token cannot continue the previous expression.
    Semicolon,
}

/// Syntax description for one C-like language.
#[derive(Debug, Clone, Copy)]
pub struct CLikeProfile {
    pub line_comments: &'static [&'static str],
    pub block_comments: &'static [(&'static str, &'static str)],
    pub nested_block_comments: bool,
    pub quote_chars: &'static [char],
    pub triple_quote_chars: &'static [char],
    pub backslash_escapes: bool,
    pub doubled_quote_escapes: bool,
    pub prefixed_strings: &'static [&'static str],
    pub raw_hash_strings: bool,
    pub backtick_strings: bool,
    pub preprocessor_prefix: Option<char>,
    pub regex_literals: bool,
    pub statement_separator: StatementSeparator,
    pub identifier_extra_chars: &'static str,
    pub long_bracket_strings: bool,
    pub cpp_raw_strings: bool,
}

const C_STYLE_BASE: CLikeProfile = CLikeProfile {
    line_comments: &["//"],
    block_comments: &[("/*", "*/")],
    nested_block_comments: false,
    quote_chars: &['"', '\''],
    triple_quote_chars: &[],
    backslash_escapes: true,
    doubled_quote_escapes: false,
    prefixed_strings: &[],
    raw_hash_strings: false,
    backtick_strings: false,
    preprocessor_prefix: None,
    regex_literals: false,
    statement_separator: StatementSeparator::None,
    identifier_extra_chars: "",
    long_bracket_strings: false,
    cpp_raw_strings: false,
};

pub static JAVASCRIPT: CLikeProfile = CLikeProfile {
    backtick_strings: true,
    regex_literals: true,
    statement_separator: StatementSeparator::Semicolon,
    identifier_extra_chars: "$",
    ..C_STYLE_BASE
};

pub static TYPESCRIPT: CLikeProfile = CLikeProfile {
    backtick_strings: true,
    regex_literals: true,
    statement_separator: StatementSeparator::Semicolon,
    identifier_extra_chars: "$",
    ..C_STYLE_BASE
};

pub static JAVA: CLikeProfile = CLikeProfile {
    triple_quote_chars: &['"'],
    ..C_STYLE_BASE
};

pub static KOTLIN: CLikeProfile = CLikeProfile {
    nested_block_comments: true,
    triple_quote_chars: &['"'],
    backtick_strings: true,
    statement_separator: StatementSeparator::Semicolon,
    identifier_extra_chars: "$",
    ..C_STYLE_BASE
};

pub static GROOVY: CLikeProfile = CLikeProfile {
    triple_quote_chars: &['"', '\''],
    statement_separator: StatementSeparator::Semicolon,
    ..C_STYLE_BASE
};

pub static SCALA: CLikeProfile = CLikeProfile {
    nested_block_comments: true,
    triple_quote_chars: &['"'],
    statement_separator: StatementSeparator::Semicolon,
    ..C_STYLE_BASE
};

pub static C: CLikeProfile = CLikeProfile {
    preprocessor_prefix: Some('#'),
    prefixed_strings: &["u8", "u", "U", "L"],
    ..C_STYLE_BASE
};

pub static CPP: CLikeProfile = CLikeProfile {
    preprocessor_prefix: Some('#'),
    prefixed_strings: &["u8", "u", "U", "L", "R"],
    cpp_raw_strings: true,
    ..C_STYLE_BASE
};

pub static CSHARP: CLikeProfile = CLikeProfile {
    triple_quote_chars: &['"'],
    doubled_quote_escapes: true,
    prefixed_strings: &["$@", "@$", "@", "$"],
    preprocessor_prefix: Some('#'),
    ..C_STYLE_BASE
};

pub static OBJECTIVE_C: CLikeProfile = CLikeProfile {
    preprocessor_prefix: Some('#'),
    prefixed_strings: &["@", "L"],
    ..C_STYLE_BASE
};

pub static GO: CLikeProfile = CLikeProfile {
    backtick_strings: true,
    statement_separator: StatementSeparator::Semicolon,
    ..C_STYLE_BASE
};

pub static SWIFT: CLikeProfile = CLikeProfile {
    nested_block_comments: true,
    triple_quote_chars: &['"'],
    raw_hash_strings: true,
    statement_separator: StatementSeparator::Semicolon,
    ..C_STYLE_BASE
};

pub static DART: CLikeProfile = CLikeProfile {
    nested_block_comments: true,
    triple_quote_chars: &['"', '\''],
    prefixed_strings: &["r"],
    statement_separator: StatementSeparator::Semicolon,
    ..C_STYLE_BASE
};

pub static PHP: CLikeProfile = CLikeProfile {
    line_comments: &["//", "#"],
    ..C_STYLE_BASE
};

pub static LUA: CLikeProfile = CLikeProfile {
    line_comments: &["--"],
    block_comments: &[("--[[", "]]")],
    long_bracket_strings: true,
    ..C_STYLE_BASE
};

pub static PROTO: CLikeProfile = CLikeProfile {
    ..C_STYLE_BASE
};

pub static THRIFT: CLikeProfile = CLikeProfile {
    line_comments: &["//", "#"],
    ..C_STYLE_BASE
};

pub static FLATBUFFERS: CLikeProfile = CLikeProfile {
    quote_chars: &['"'],
    ..C_STYLE_BASE
};

pub static AVRO: CLikeProfile = CLikeProfile {
    quote_chars: &['"'],
    ..C_STYLE_BASE
};

pub static ELIXIR: CLikeProfile = CLikeProfile {
    line_comments: &["#"],
    triple_quote_chars: &['"'],
    ..C_STYLE_BASE
};

pub static ERLANG: CLikeProfile = CLikeProfile {
    line_comments: &["%"],
    ..C_STYLE_BASE
};

pub static CLOJURE: CLikeProfile = CLikeProfile {
    line_comments: &[";"],
    block_comments: &[],
    quote_chars: &['"'],
    ..C_STYLE_BASE
};

pub static OCAML: CLikeProfile = CLikeProfile {
    line_comments: &[],
    block_comments: &[("(*", "*)")],
    nested_block_comments: true,
    quote_chars: &['"'],
    ..C_STYLE_BASE
};

pub static PASCAL: CLikeProfile = CLikeProfile {
    block_comments: &[("{", "}"), ("(*", "*)")],
    quote_chars: &['\''],
    ..C_STYLE_BASE
};

pub static CRYSTAL: CLikeProfile = CLikeProfile {
    line_comments: &["#"],
    statement_separator: StatementSeparator::Semicolon,
    ..C_STYLE_BASE
};

pub static SOLIDITY: CLikeProfile = CLikeProfile {
    ..C_STYLE_BASE
};

pub static CUDA: CLikeProfile = CLikeProfile {
    preprocessor_prefix: Some('#'),
    prefixed_strings: &["u8", "u", "U", "L"],
    ..C_STYLE_BASE
};

pub static VERILOG: CLikeProfile = CLikeProfile {
    quote_chars: &['"'],
    ..C_STYLE_BASE
};

pub static VHDL: CLikeProfile = CLikeProfile {
    line_comments: &["--"],
    block_comments: &[],
    quote_chars: &['"'],
    ..C_STYLE_BASE
};

pub static GLSL: CLikeProfile = CLikeProfile {
    preprocessor_prefix: Some('#'),
    ..C_STYLE_BASE
};

pub static OPENCL: CLikeProfile = CLikeProfile {
    preprocessor_prefix: Some('#'),
    prefixed_strings: &["u8", "u", "U", "L"],
    ..C_STYLE_BASE
};

pub static QML: CLikeProfile = CLikeProfile {
    backtick_strings: true,
    regex_literals: true,
    statement_separator: StatementSeparator::Semicolon,
    identifier_extra_chars: "$",
    ..C_STYLE_BASE
};

pub static STRINGS: CLikeProfile = CLikeProfile {
    quote_chars: &['"'],
    ..C_STYLE_BASE
};

pub static ZIG: CLikeProfile = CLikeProfile {
    quote_chars: &['"'],
    ..C_STYLE_BASE
};

pub static D_LANGUAGE: CLikeProfile = CLikeProfile {
    nested_block_comments: true,
    quote_chars: &['"', '\'', '`'],
    backtick_strings: true,
    ..C_STYLE_BASE
};

/// Return the profile for a language id. This keeps the registry free of
/// engine-specific types while still allowing every language to have a precise
/// lexer configuration.
pub fn profile_for(language_id: &str) -> Option<&'static CLikeProfile> {
    match language_id {
        "javascript" | "jsx" => Some(&JAVASCRIPT),
        "typescript" | "tsx" => Some(&TYPESCRIPT),
        "java" => Some(&JAVA),
        "kotlin" => Some(&KOTLIN),
        "groovy" => Some(&GROOVY),
        "scala" => Some(&SCALA),
        "c" => Some(&C),
        "cpp" => Some(&CPP),
        "csharp" => Some(&CSHARP),
        "objectivec" => Some(&OBJECTIVE_C),
        "go" => Some(&GO),
        "swift" => Some(&SWIFT),
        "dart" => Some(&DART),
        "php" => Some(&PHP),
        "lua" => Some(&LUA),
        "proto" => Some(&PROTO),
        "thrift" => Some(&THRIFT),
        "flatbuffers" => Some(&FLATBUFFERS),
        "avro" => Some(&AVRO),
        "elixir" => Some(&ELIXIR),
        "erlang" => Some(&ERLANG),
        "clojure" => Some(&CLOJURE),
        "ocaml" => Some(&OCAML),
        "pascal" => Some(&PASCAL),
        "crystal" => Some(&CRYSTAL),
        "solidity" => Some(&SOLIDITY),
        "cuda" => Some(&CUDA),
        "verilog" => Some(&VERILOG),
        "vhdl" => Some(&VHDL),
        "glsl" => Some(&GLSL),
        "opencl" => Some(&OPENCL),
        "qml" => Some(&QML),
        "strings" => Some(&STRINGS),
        "zig" => Some(&ZIG),
        "d" => Some(&D_LANGUAGE),
        _ => None,
    }
}
