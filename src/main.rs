#[cfg(all(
    any(target_os = "linux", target_os = "windows"),
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

    if args.list_types {
        print!("{}", minifier::language::type_listing());
        return Ok(());
    }

    if args.files.is_empty() {
        processor::process_stdin(&args).await?;
    } else {
        processor::process_all(&args).await?;
    }

    Ok(())
}
