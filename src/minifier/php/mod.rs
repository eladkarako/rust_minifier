//! PHP minifier for mixed HTML/PHP documents.
//!
//! PHP files interleave HTML with `<?php ... ?>` blocks. Each PHP block is
//! minified as code, while the surrounding document is minified as HTML. PHP
//! blocks are replaced with private-use placeholders during HTML minification
//! so whitespace around them is handled correctly, then restored afterwards.

use crate::minifier::c_like::{minify_c_like, profile};
use crate::minifier::markup::minify_markup;
use crate::minifier::options::MinifierOptions;
use crate::minifier::Minifier;
use anyhow::Result;

pub struct PhpMinifier;

impl Minifier for PhpMinifier {
    fn minify(
        &self,
        source: &str,
        options: &MinifierOptions,
    ) -> Result<String> {
        Ok(minify_php(source, options))
    }
}

const PLACEHOLDER_OPEN: char = '\u{e000}';
const PLACEHOLDER_CLOSE: char = '\u{e001}';

fn starts_with_at(
    chars: &[char],
    position: usize,
    needle: &str,
) -> bool {
    let mut offset = 0usize;
    for expected in needle.chars() {
        if chars.get(position + offset) != Some(&expected) {
            return false;
        }
        offset += 1;
    }
    true
}

fn starts_with_ci_at(
    chars: &[char],
    position: usize,
    needle: &str,
) -> bool {
    let mut offset = 0usize;
    for expected in needle.chars() {
        match chars.get(position + offset) {
            Some(actual)
                if actual.to_ascii_lowercase()
                    == expected.to_ascii_lowercase() => {}
            _ => return false,
        }
        offset += 1;
    }
    true
}

fn php_open_tag(
    chars: &[char],
    position: usize,
) -> Option<(usize, &'static str)> {
    if starts_with_ci_at(chars, position, "<?php") {
        return Some((5, "<?php"));
    }
    if starts_with_at(chars, position, "<?=") {
        return Some((3, "<?="));
    }
    if starts_with_at(chars, position, "<?") {
        return Some((2, "<?"));
    }
    None
}

fn find_php_close(chars: &[char], start: usize) -> Option<usize> {
    let mut position = start;
    let mut quote: Option<char> = None;

    while position + 1 < chars.len() {
        let character = chars[position];

        if let Some(active_quote) = quote {
            if character == '\\' {
                position += 2;
                continue;
            }
            if character == active_quote {
                quote = None;
            }
            position += 1;
            continue;
        }

        if character == '\'' || character == '"' {
            quote = Some(character);
            position += 1;
            continue;
        }

        if character == '/' && chars[position + 1] == '*' {
            position += 2;
            while position + 1 < chars.len() {
                if chars[position] == '*'
                    && chars[position + 1] == '/'
                {
                    position += 2;
                    break;
                }
                position += 1;
            }
            continue;
        }

        if character == '/'
            && chars[position + 1] == '/'
        {
            while position < chars.len()
                && chars[position] != '\n'
                && chars[position] != '\r'
            {
                position += 1;
            }
            continue;
        }

        if character == '#'
            && chars.get(position + 1) != Some(&'[')
        {
            while position < chars.len()
                && chars[position] != '\n'
                && chars[position] != '\r'
            {
                position += 1;
            }
            continue;
        }

        if character == '?' && chars[position + 1] == '>' {
            return Some(position);
        }

        position += 1;
    }

    None
}

fn replace_placeholders(
    minified: &str,
    blocks: &[String],
) -> String {
    let chars: Vec<char> = minified.chars().collect();
    let mut output = String::new();
    let mut position = 0usize;

    while position < chars.len() {
        if chars[position] == PLACEHOLDER_OPEN {
            let mut cursor = position + 1;
            let mut digits = String::new();
            while cursor < chars.len() && chars[cursor].is_ascii_digit() {
                digits.push(chars[cursor]);
                cursor += 1;
            }
            if chars.get(cursor) == Some(&PLACEHOLDER_CLOSE) {
                if let Ok(index) = digits.parse::<usize>() {
                    if let Some(block) = blocks.get(index) {
                        output.push_str(block);
                        position = cursor + 1;
                        continue;
                    }
                }
            }
        }
        output.push(chars[position]);
        position += 1;
    }

    output
}

/// Minify a mixed HTML/PHP document.
pub fn minify_php(
    source: &str,
    options: &MinifierOptions,
) -> String {
    let source = source.trim_start_matches('\u{feff}');
    if source.trim().is_empty() {
        return String::new();
    }

    let chars: Vec<char> = source.chars().collect();
    let mut template = String::new();
    let mut blocks: Vec<String> = Vec::new();
    let mut position = 0usize;

    while position < chars.len() {
        if let Some((open_length, open_tag)) =
            php_open_tag(&chars, position)
        {
            let inner_start = position + open_length;
            let close = find_php_close(&chars, inner_start);
            let (inner_end, next_position) = match close {
                Some(index) => (index, index + 2),
                None => (chars.len(), chars.len()),
            };

            let inner: String =
                chars[inner_start..inner_end].iter().collect();
            let code = minify_c_like(&inner, &profile::PHP, options);

            let mut block = String::new();
            block.push_str(open_tag);
            block.push(' ');
            block.push_str(&code);
            if close.is_some() {
                block.push_str("?>");
            }

            template.push(PLACEHOLDER_OPEN);
            template.push_str(&blocks.len().to_string());
            template.push(PLACEHOLDER_CLOSE);
            blocks.push(block);

            position = next_position;
            continue;
        }

        template.push(chars[position]);
        position += 1;
    }

    let minified = minify_markup(&template, "html", options);
    replace_placeholders(&minified, &blocks)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
