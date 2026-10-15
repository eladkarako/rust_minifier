use anyhow::Result;
use std::path::Path;

pub mod rust;
pub mod toml;

use rust::RustMinifier;
use toml::TomlMinifier;

/// Generic minifier trait for different file formats
pub trait Minifier {
    fn minify(
        &self,
        source: &str,
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
    let minifier = get_minifier(path)?;
    minifier.minify(source)
}
