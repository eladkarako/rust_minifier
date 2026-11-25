//! Line preserving minifier.
//!
//! Used for formats where newlines and indentation are part of the syntax:
//! YAML, shell, INI/properties, Makefiles, Dockerfiles, SQL-like config files
//! and many scripting languages. Comments are removed first, trailing
//! whitespace is trimmed, blank lines are dropped, and the line structure is
//! otherwise kept intact. Heredocs and YAML block scalars are preserved
//! verbatim so their contents are never altered.

pub mod profile;

use crate::minifier::options::MinifierOptions;
use crate::minifier::Minifier;
use anyhow::Result;
use profile::LineProfile;

/// Line based minifier bound to one syntax profile.
pub struct LineBasedMinifier {
    pub language: &'static str,
}

impl Minifier for LineBasedMinifier {
    fn minify(
        &self,
        source: &str,
        options: &MinifierOptions,
    ) -> Result<String> {
        let profile = profile::profile_for(self.language).ok_or_else(|| {
            anyhow::anyhow!(
                "No line based profile for language: {}",
                self.language
            )
        })?;
        Ok(minify_line_based(source, profile, options))
    }
}

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

fn find_from(
    chars: &[char],
    start: usize,
    needle: &str,
) -> Option<usize> {
    let needle: Vec<char> = needle.chars().collect();
    if needle.is_empty() {
        return Some(start);
    }
    let mut position = start;
    while position + needle.len() <= chars.len() {
        if chars[position..position + needle.len()] == needle[..] {
            return Some(position);
        }
        position += 1;
    }
    None
}

fn strip_line_comments(
    line: &str,
    profile: &LineProfile,
    in_block: &mut Option<(&'static str, &'static str)>,
) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut output = String::new();
    let mut position = 0usize;
    let mut quote: Option<char> = None;

    while position < chars.len() {
        if let Some((_open, close)) = *in_block {
            if starts_with_at(&chars, position, close) {
                position += close.chars().count();
                *in_block = None;
                continue;
            }
            position += 1;
            continue;
        }

        let character = chars[position];

        if let Some(active_quote) = quote {
            output.push(character);
            if profile.backslash_escapes && character == '\\' {
                if let Some(next) = chars.get(position + 1).copied() {
                    output.push(next);
                    position += 2;
                    continue;
                }
            }
            if character == active_quote {
                quote = None;
            }
            position += 1;
            continue;
        }

        let mut opened_block = false;
        for (open, close) in profile.block_comments {
            if starts_with_at(&chars, position, open) {
                let after_open = position + open.chars().count();
                if let Some(end) = find_from(&chars, after_open, close) {
                    position = end + close.chars().count();
                } else {
                    *in_block = Some((*open, *close));
                    return output;
                }
                opened_block = true;
                break;
            }
        }
        if opened_block {
            continue;
        }

        let mut is_line_comment = false;
        for marker in profile.line_comments {
            if starts_with_at(&chars, position, marker) {
                let at_line_start = output.trim().is_empty();
                let previous_is_boundary = position == 0
                    || chars[position - 1].is_whitespace();
                let accepted = if profile.line_comment_line_start_only {
                    at_line_start
                } else if profile.line_comment_requires_boundary {
                    previous_is_boundary
                } else {
                    true
                };
                if accepted {
                    is_line_comment = true;
                    break;
                }
            }
        }
        if is_line_comment {
            break;
        }

        if position == 0 || output.trim().is_empty() {
            for word in profile.line_start_words {
                if starts_with_ci_at(&chars, position, word) {
                    let after = position + word.chars().count();
                    if chars
                        .get(after)
                        .is_none_or(|next| next.is_whitespace())
                    {
                        is_line_comment = true;
                        break;
                    }
                }
            }
            if is_line_comment {
                break;
            }
        }

        if profile.quote_chars.contains(&character) {
            quote = Some(character);
            output.push(character);
            position += 1;
            continue;
        }

        output.push(character);
        position += 1;
    }

    output
}

fn leading_spaces(line: &str) -> usize {
    line.chars().take_while(|character| *character == ' ').count()
}

fn detect_heredoc(line: &str) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();
    let mut position = 0usize;
    let mut quote: Option<char> = None;

    while position + 1 < chars.len() {
        let character = chars[position];
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            }
            position += 1;
            continue;
        }
        if character == '"' || character == '\'' {
            quote = Some(character);
            position += 1;
            continue;
        }
        if character == '<' && chars[position + 1] == '<' {
            let mut cursor = position + 2;
            if chars.get(cursor) == Some(&'-') {
                cursor += 1;
            }
            while chars
                .get(cursor)
                .is_some_and(|next| next.is_whitespace())
            {
                cursor += 1;
            }
            let mut delimiter = String::new();
            if let Some(&quote_character) = chars.get(cursor) {
                if quote_character == '"' || quote_character == '\'' {
                    cursor += 1;
                    while cursor < chars.len()
                        && chars[cursor] != quote_character
                    {
                        delimiter.push(chars[cursor]);
                        cursor += 1;
                    }
                } else {
                    while cursor < chars.len()
                        && !chars[cursor].is_whitespace()
                        && chars[cursor] != ';'
                        && chars[cursor] != '|'
                        && chars[cursor] != '&'
                    {
                        delimiter.push(chars[cursor]);
                        cursor += 1;
                    }
                }
            }
            if !delimiter.is_empty() {
                return Some(delimiter);
            }
            position = cursor;
            continue;
        }
        position += 1;
    }

    None
}

fn yaml_block_scalar_start(line: &str) -> bool {
    let trimmed = line.trim_end();
    if trimmed.ends_with('|') || trimmed.ends_with('>') {
        return true;
    }
    let chars: Vec<char> = trimmed.chars().collect();
    if chars.len() >= 2 {
        let last = chars[chars.len() - 1];
        let previous = chars[chars.len() - 2];
        if (last == '-' || last == '+' || last.is_ascii_digit())
            && (previous == '|' || previous == '>')
        {
            return true;
        }
    }
    false
}

fn is_first_line_directive(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("#!")
        || trimmed.starts_with("# syntax=")
        || trimmed.starts_with("# escape=")
}

/// Minify a line oriented document.
pub fn minify_line_based(
    source: &str,
    profile: &LineProfile,
    _options: &MinifierOptions,
) -> String {
    let source = source.trim_start_matches('\u{feff}');
    if source.trim().is_empty() {
        return String::new();
    }

    let mut output: Vec<String> = Vec::new();
    let mut in_block: Option<(&'static str, &'static str)> = None;
    let mut heredoc: Option<String> = None;
    let mut block_scalar_indent: Option<usize> = None;
    let mut pending: Option<String> = None;
    let mut first_output = true;

    for raw_line in source.lines() {
        let raw_line = raw_line.trim_end_matches('\r');

        if let Some(delimiter) = heredoc.clone() {
            if raw_line.trim() == delimiter {
                heredoc = None;
            }
            output.push(raw_line.to_string());
            first_output = false;
            continue;
        }

        if let Some(base_indent) = block_scalar_indent {
            if raw_line.trim().is_empty() {
                output.push(raw_line.to_string());
                continue;
            }
            if leading_spaces(raw_line) > base_indent {
                output.push(raw_line.to_string());
                continue;
            }
            block_scalar_indent = None;
        }

        if first_output && output.is_empty() && is_first_line_directive(raw_line)
        {
            output.push(raw_line.trim_end().to_string());
            first_output = false;
            continue;
        }

        let mut line =
            strip_line_comments(raw_line, profile, &mut in_block);
        if in_block.is_some() && line.trim().is_empty() {
            continue;
        }

        line = line.trim_end().to_string();

        if let Some(previous) = pending.take() {
            let mut combined = previous;
            combined.push_str(line.trim_start());
            line = combined;
        }

        if profile.join_backslash_continuations && line.ends_with('\\') {
            line.pop();
            pending = Some(line);
            continue;
        }

        if line.trim().is_empty() {
            if !profile.remove_blank_lines {
                output.push(String::new());
            }
            continue;
        }

        if !profile.preserve_leading_whitespace {
            line = line.trim_start().to_string();
        }

        if profile.heredoc {
            if let Some(delimiter) = detect_heredoc(&line) {
                heredoc = Some(delimiter);
            }
        }

        if profile.yaml_block_scalar && yaml_block_scalar_start(&line) {
            block_scalar_indent = Some(leading_spaces(&line));
        }

        output.push(line);
        first_output = false;
    }

    if let Some(previous) = pending {
        output.push(previous);
    }

    output.join("\n")
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
