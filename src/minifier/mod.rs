use anyhow::Result;
use std::path::Path;

pub mod c_like;
pub mod css;
pub mod json;
pub mod language;
pub mod options;
pub mod rust;
pub mod toml;

pub use options::MinifierOptions;
use rust::RustMinifier;
use toml::TomlMinifier;

/// Generic minifier trait for different file formats
pub trait Minifier {
    fn minify(
        &self,
        source: &str,
        options: &MinifierOptions,
    ) -> Result<String>;
}

/// Detect file type and return appropriate minifier
pub fn get_minifier(path: &Path) -> Result<Box<dyn Minifier>> {
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "rs" => Ok(Box::new(RustMinifier)),
        "toml" => Ok(Box::new(TomlMinifier)),
        _ => Err(anyhow::anyhow!(
            "Unsupported file type: .{}",
            extension
        )),
    }
}

/// Public minify function that auto-detects file type
pub fn minify(
    source: &str,
    path: &Path,
) -> Result<String> {
    minify_with_options(source, path, &MinifierOptions::default())
}

/// Public minify function with explicit options.
pub fn minify_with_options(
    source: &str,
    path: &Path,
    options: &MinifierOptions,
) -> Result<String> {
    let minifier = get_minifier(path)?;
    minifier.minify(source, options)
}
