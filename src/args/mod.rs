use clap::Parser;

/// Rust code minifier with aggressive whitespace and comment removal.
#[derive(Parser, Debug, Clone)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[command(after_help = concat!(
"Repository:    ", env!("CARGO_PKG_REPOSITORY"), "\n",
"Homepage:      ", env!("CARGO_PKG_HOMEPAGE")
))]
pub struct Args {
    /// Files to minify (leave empty for stdin)
    #[arg(value_name = "FILES")]
    pub files: Vec<String>,

    /// Output suffix for minified files (default: _minified)
    #[arg(short, long, default_value = "_minified")]
    pub suffix: String,

    /// Concatenate all output to this file
    #[arg(long, value_name = "PATH")]
    pub concat_output: Option<String>,

    /// Write to stdout instead of files as each thread finishes
    #[arg(long)]
    pub concat_to_stdout: bool,
}

impl Args {
    /// Parse command-line arguments.
    pub fn parse() -> Args {
        <Args as clap::Parser>::parse()
    }
}


#[cfg(test)]
#[path = "tests.rs"]
mod tests;
