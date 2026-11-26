//! JSON, JSONC and JSON5 minifier.
//!
//! JSON is whitespace insensitive, so the default output is a single line with
//! no spaces at all. Comments are accepted (many real `.json` files are JSONC)
//! and trailing commas are removed. Strings are copied verbatim.

use crate::minifier::options::MinifierOptions;
use crate::minifier::Minifier;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonProfile {
    pub allow_comments: bool,
    pub allow_single_quotes: bool,
    pub remove_trailing_commas: bool,
}

/// Strict JSON. Comments and single quoted strings are still tolerated because
/// many real `.json` files are really JSONC or JSON5, and accepting them is
/// safer than emitting a corrupted file.
pub static JSON: JsonProfile = JsonProfile {
    allow_comments: true,
    allow_single_quotes: true,
    remove_trailing_commas: true,
};

/// JSON with comments.
pub static JSONC: JsonProfile = JsonProfile {
    allow_comments: true,
    allow_single_quotes: true,
    remove_trailing_commas: true,
};

/// JSON5, which additionally allows single quoted strings.
pub static JSON5: JsonProfile = JsonProfile {
    allow_comments: true,
    allow_single_quotes: true,
    remove_trailing_commas: true,
};

pub struct JsonMinifier {
    pub profile: &'static JsonProfile,
}

impl Minifier for JsonMinifier {
    fn minify(
        &self,
        source: &str,
        options: &MinifierOptions,
    ) -> Result<String> {
        Ok(minify_json(source, self.profile, options))
    }
}

fn skip_line_comment(
    chars: &[char],
    position: &mut usize,
) {
    while *position < chars.len()
        && chars[*position] != '\n'
        && chars[*position] != '\r'
    {
        *position += 1;
    }
}

fn skip_block_comment(
    chars: &[char],
    position: &mut usize,
) {
    *position += 2;
    while *position < chars.len() {
        if chars[*position] == '*'
            && chars.get(*position + 1) == Some(&'/')
        {
            *position += 2;
            return;
        }
        *position += 1;
    }
}

fn next_significant_character(
    chars: &[char],
    mut position: usize,
    profile: &JsonProfile,
) -> Option<char> {
    while position < chars.len() {
        let character = chars[position];
        if character.is_whitespace() {
            position += 1;
            continue;
        }
        if profile.allow_comments
            && character == '/'
            && chars.get(position + 1) == Some(&'/')
        {
            skip_line_comment(chars, &mut position);
            continue;
        }
        if profile.allow_comments
            && character == '/'
            && chars.get(position + 1) == Some(&'*')
        {
            skip_block_comment(chars, &mut position);
            continue;
        }
        return Some(character);
    }
    None
}

fn copy_string(
    chars: &[char],
    position: &mut usize,
    quote: char,
    output: &mut String,
) {
    output.push(quote);
    *position += 1;
    while *position < chars.len() {
        let character = chars[*position];
        output.push(character);
        *position += 1;
        if character == '\\' {
            if let Some(escaped) = chars.get(*position).copied() {
                output.push(escaped);
                *position += 1;
            }
            continue;
        }
        if character == quote {
            return;
        }
    }
}

/// Minify a JSON-like document.
pub fn minify_json(
    source: &str,
    profile: &JsonProfile,
    options: &MinifierOptions,
) -> String {
    let source = source.trim_start_matches('\u{feff}');
    let chars: Vec<char> = source.chars().collect();
    let mut output = String::new();
    let mut position = 0usize;

    while position < chars.len() {
        let character = chars[position];

        if character.is_whitespace() {
            if options.preserve_line_breaks {
                if (character == '\n' || character == '\r')
                    && !output.ends_with('\n')
                {
                    output.push('\n');
                }
            }
            position += 1;
            continue;
        }

        if profile.allow_comments
            && character == '/'
            && chars.get(position + 1) == Some(&'/')
        {
            skip_line_comment(&chars, &mut position);
            continue;
        }

        if profile.allow_comments
            && character == '/'
            && chars.get(position + 1) == Some(&'*')
        {
            skip_block_comment(&chars, &mut position);
            continue;
        }

        if character == '"'
            || (character == '\'' && profile.allow_single_quotes)
        {
            copy_string(
                &chars,
                &mut position,
                character,
                &mut output,
            );
            continue;
        }

        if character == ','
            && profile.remove_trailing_commas
            && matches!(
                next_significant_character(
                    &chars,
                    position + 1,
                    profile
                ),
                Some('}') | Some(']')
            )
        {
            position += 1;
            continue;
        }

        output.push(character);
        position += 1;
    }

    while output.ends_with('\n') {
        output.pop();
    }
    output
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
