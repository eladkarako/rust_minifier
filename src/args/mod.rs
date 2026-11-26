use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[command(
    after_help = concat!("Repository:    ", env!("CARGO_PKG_REPOSITORY"), "\n", "Homepage:      ", env!("CARGO_PKG_HOMEPAGE"), "\n", "Use --list-types to see every supported file type.")
)]
pub struct Args {
    /// Input files or directories to minify.
    ///
    /// Can be individual supported text files, or directories to recursively search.
    /// If omitted, reads from stdin.
    #[arg(value_name = "FILES")]
    pub files: Vec<String>,

    /// Force a language for every input instead of detecting it.
    ///
    /// Accepts a language id or alias from `--list-types`, for example
    /// `html`, `typescript`, `kotlin` or `yaml`.
    #[arg(short = 't', long = "type", value_name = "LANGUAGE")]
    pub forced_type: Option<String>,

    /// Keep original line breaks where the format allows it.
    ///
    /// By default output is collapsed to a single line. Line based formats
    /// already keep their line structure.
    #[arg(long)]
    pub preserve_line_breaks: bool,

    /// List every supported and excluded file type, then exit.
    #[arg(long)]
    pub list_types: bool,

    /// Suffix to append to minified output filenames.
    ///
    /// For example, with suffix `_minified`, `main.rs` becomes `main_minified.rs`.
    /// Only used when writing individual files (not with `--concat-to`).
    #[arg(short, long, default_value = "_minified")]
    pub suffix: String,

    /// Destination for concatenated output.
    ///
    /// Can be A file path (writes minified content to that file), a directory path (creates a timestamped file in that directory), `STDOUT` (or empty - prints to standard output, default).
    ///
    /// When specified, individual minified files are not written to disk.
    #[arg(
        long,
        value_name = "DESTINATION",
        default_value = "STDOUT"
    )]
    pub concat_to: String,

    /// Include source file paths as comments in output.
    ///
    /// When enabled, each minified file is prefixed with a comment containing its full path.
    /// Useful for tracking which source files contributed to concatenated output.
    #[arg(long, default_value = "true")]
    pub include_file_path: bool,

    /// Do not prefix output with the source file path.
    ///
    /// This is the inverse of `--include-file-path` and wins when both are set.
    #[arg(long)]
    pub no_include_file_path: bool,

    /// Exclude files matching these patterns (non-case-sensitive substring match on filename).
    ///
    /// Files whose names contain any of these patterns are skipped during processing.
    /// Matching is case-insensitive and applied only to the filename, not the full path.
    ///
    /// for example:
    /// rust_minifier src/ --exclude test --exclude mock
    /// will exclude files like: `test_helper.rs`, `mock_data.toml`, `my_test.rs`
    #[arg(long)]
    pub exclude: Vec<String>,

    /// Try to limit maximum number of characters per file in --concat-to result, by generating multiple files each with a '_part_XX_of_YY' suffix.
    #[arg(long, default_value = "unlimited")]
    pub soft_split: String,
}

impl Args {
    pub fn parse() -> Args {
        <Args as clap::Parser>::parse()
    }

    /// Whether source paths should be embedded in the output.
    pub fn include_source_path(&self) -> bool {
        self.include_file_path && !self.no_include_file_path
    }

    /// Build the shared minifier options from the CLI flags.
    pub fn minifier_options(
        &self
    ) -> crate::minifier::MinifierOptions {
        crate::minifier::MinifierOptions {
            preserve_line_breaks: self.preserve_line_breaks,
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
