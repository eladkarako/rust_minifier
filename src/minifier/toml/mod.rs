use super::Minifier;
use anyhow::Result;

pub struct TomlMinifier;

impl Minifier for TomlMinifier {
    fn minify(
        &self,
        source: &str,
    ) -> Result<String> {
        let mut result = String::new();

        for line in source.lines() {
            // Remove leading whitespace
            let line = line.trim_start();

            // Remove inline comments (preserve # inside strings)
            let line = remove_inline_comments(line);
            let trimmed = line.trim();

            // Skip empty lines and comment-only lines
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Process the line
            let processed = normalize_spacing(trimmed);
            result.push_str(&processed);
            result.push('\n');
        }

        // Clean up trailing whitespace
        Ok(result.trim_end().to_string())
    }
}

/// Remove inline comments while preserving # inside strings
fn remove_inline_comments(line: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut escape_next = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '"' && !escape_next {
            in_string = !in_string;
            result.push(ch);
            escape_next = false;
            continue;
        }

        if in_string {
            result.push(ch);
            escape_next = ch == '\\' && !escape_next;
            continue;
        }

        // Not in string: if we hit #, skip rest of line
        if ch == '#' {
            break;
        }

        result.push(ch);
        escape_next = false;
    }

    result
}

/// Normalize spacing: remove spaces/tabs around =, ,, {, and }
fn normalize_spacing(line: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut escape_next = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        // Handle string boundaries
        if ch == '"' && !escape_next {
            in_string = !in_string;
            result.push(ch);
            escape_next = false;
            continue;
        }

        if in_string {
            result.push(ch);
            escape_next = ch == '\\' && !escape_next;
            continue;
        }

        // Handle equals sign: remove spaces/tabs around it
        if ch == '=' {
            // Remove trailing spaces/tabs from result
            while result.ends_with(' ') || result.ends_with('\t') {
                result.pop();
            }
            result.push('=');

            // Skip spaces/tabs after =
            while let Some(&next_ch) = chars.peek() {
                if next_ch == ' ' || next_ch == '\t' {
                    chars.next();
                } else {
                    break;
                }
            }
            continue;
        }

        // Handle comma: remove spaces/tabs around it
        if ch == ',' {
            // Remove trailing spaces/tabs before comma
            while result.ends_with(' ') || result.ends_with('\t') {
                result.pop();
            }
            result.push(',');

            // Skip spaces/tabs after comma
            while let Some(&next_ch) = chars.peek() {
                if next_ch == ' ' || next_ch == '\t' {
                    chars.next();
                } else {
                    break;
                }
            }
            continue;
        }

        // Handle opening brace: remove spaces/tabs after it
        if ch == '{' {
            result.push('{');

            // Skip spaces/tabs after {
            while let Some(&next_ch) = chars.peek() {
                if next_ch == ' ' || next_ch == '\t' {
                    chars.next();
                } else {
                    break;
                }
            }
            continue;
        }

        // Handle closing brace: remove spaces/tabs before it
        if ch == '}' {
            // Remove trailing spaces/tabs before }
            while result.ends_with(' ') || result.ends_with('\t') {
                result.pop();
            }
            result.push('}');
            continue;
        }

        // Handle normal characters
        result.push(ch);
        escape_next = false;
    }

    result
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
