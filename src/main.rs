mod args;
mod metadata;
mod minifier;
mod processor;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();

    if args.files.is_empty() {
        processor::process_stdin(&args).await?;
    } else {
        processor::process_all(&args).await?;
    }

    Ok(())
}
