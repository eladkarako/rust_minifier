use anyhow::Result;
use std::path::Path;

pub mod c_like;
pub mod css;
pub mod graphql;
pub mod json;
pub mod language;
pub mod line_based;
pub mod markup;
pub mod options;
pub mod php;
pub mod rust;
pub mod sql;
pub mod toml;

pub use options::MinifierOptions;

use c_like::{profile, CLikeMinifier};
use css::{CssDialect, CssMinifier};
use graphql::GraphQlMinifier;
use json::{JsonMinifier, JSON};
use language::{EngineKind, Language};
use line_based::LineBasedMinifier;
use markup::MarkupMinifier;
use php::PhpMinifier;
use rust::RustMinifier;
use sql::SqlMinifier;
use toml::TomlMinifier;

/// Generic minifier trait for different file formats.
pub trait Minifier {
    fn minify(
        &self,
        source: &str,
        options: &MinifierOptions,
    ) -> Result<String>;
}

/// Build a minifier for a detected language.
pub fn minifier_for_language(
    language: &'static Language,
) -> Result<Box<dyn Minifier>> {
    let minifier: Box<dyn Minifier> = match language.engine {
        EngineKind::Rust => Box::new(RustMinifier),
        EngineKind::Toml => Box::new(TomlMinifier),
        EngineKind::Markup => Box::new(MarkupMinifier {
            language: language.id,
        }),
        EngineKind::Css => {
            let dialect = match language.id {
                "scss" => CssDialect::Scss,
                "less" => CssDialect::Less,
                _ => CssDialect::Css,
            };
            Box::new(CssMinifier { dialect })
        }
        EngineKind::Json => Box::new(JsonMinifier { profile: &JSON }),
        EngineKind::CStyle => {
            let profile = profile::profile_for(language.id).ok_or_else(
                || {
                    anyhow::anyhow!(
                        "No C-like profile for language: {}",
                        language.id
                    )
                },
            )?;
            Box::new(CLikeMinifier { profile })
        }
        EngineKind::Sql => Box::new(SqlMinifier),
        EngineKind::GraphQl => Box::new(GraphQlMinifier),
        EngineKind::LineBased => Box::new(LineBasedMinifier {
            language: language.id,
        }),
        EngineKind::Php => Box::new(PhpMinifier),
    };
    Ok(minifier)
}

/// Detect a language from a path and return its minifier.
pub fn get_minifier(path: &Path) -> Result<Box<dyn Minifier>> {
    let language =
        language::detect_language(path).ok_or_else(|| {
            anyhow::anyhow!(
                "Unsupported file type: {}",
                path.display()
            )
        })?;
    minifier_for_language(language)
}

/// Detect the language for a path and source.
pub fn detect_language(
    path: &Path,
    source: &str,
) -> Option<&'static Language> {
    language::detect_language_from_source(path, source)
}

/// Minify with an already detected language.
pub fn minify_with_language(
    source: &str,
    language: &'static Language,
    options: &MinifierOptions,
) -> Result<String> {
    let minifier = minifier_for_language(language)?;
    minifier.minify(source, options)
}

/// Public minify function that auto-detects file type.
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
    let language =
        detect_language(path, source).ok_or_else(|| {
            anyhow::anyhow!(
                "Unsupported file type: {}",
                path.display()
            )
        })?;
    minify_with_language(source, language, options)
}

#[cfg(test)]
#[path = "dispatch_tests.rs"]
mod dispatch_tests;
