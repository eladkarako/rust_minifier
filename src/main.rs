#[cfg(all(
    target_os = "linux",
    any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "aarch64"
    )
))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
