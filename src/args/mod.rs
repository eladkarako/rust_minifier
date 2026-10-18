use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(about=env!("CARGO_PKG_DESCRIPTION"),long_about=None)]
#[command(
    after_help = concat!("Repository:    ", env!("CARGO_PKG_REPOSITORY"), "\n", "Homepage:      ", env!("CARGO_PKG_HOMEPAGE"))
)]
pub struct Args {
    #[arg(value_name = "FILES")]
    pub files: Vec<String>,

    #[arg(short, long, default_value = "_minified")]
    pub suffix: String,

    #[arg(
        long,
        value_name = "DESTINATION",
        default_value = "STDOUT"
    )]
    pub concat_to: String,

    #[arg(long, default_value = "true")]
    pub include_file_path: bool,
}

impl Args {
    pub fn parse() -> Args {
        <Args as clap::Parser>::parse()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
