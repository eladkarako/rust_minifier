use anyhow::Result;
use proc_macro2::TokenStream;
use std::str::FromStr;

/// Minify Rust source code.
///
/// Aggressively removes all comments, rustdocs, and unnecessary whitespace.
/// Returns the minified code as a string.
pub fn minify(source: &str) -> Result<String> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }

    // Remove comments and rustdocs while preserving string literals
    let no_comments = remove_comments_and_rustdocs(trimmed)?;

    // Parse into token stream to validate Rust syntax
    let tokens: TokenStream = no_comments
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse tokens: {}", e))?;

    // Reconstruct with minimal whitespace
    let minified = reconstruct_minimal(tokens);

    Ok(minified)
}

/// Remove all comments, rustdocs (///, //!), and block comments (/** */, /*! */).
/// Preserves string and raw string literals.
fn remove_comments_and_rustdocs(source: &str) -> Result<String> {
    let mut result = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut in_string = false;
    let mut string_char = ' ';
    let mut escape_next = false;

    while let Some(ch) = chars.next() {
        // Handle escape sequences in strings
        if in_string && escape_next {
            result.push(ch);
            escape_next = false;
            continue;
        }

        // Handle string boundaries
        if in_string {
            result.push(ch);
            if ch == '\\' {
                escape_next = true;
            } else if ch == string_char {
                in_string = false;
            }
            continue;
        }

        // Detect string start
        if (ch == '"' || ch == '\'') && !is_char_literal(&result) {
            in_string = true;
            string_char = ch;
            result.push(ch);
            continue;
        }

        // Detect raw strings (r#"..." or r"...")
        if ch == 'r' && chars.peek() == Some(&'#') || chars.peek() == Some(&'"') {
            result.push(ch);
            result.push(*chars.peek().unwrap());
            chars.next();
            if ch == 'r' && *chars.peek().unwrap_or(&' ') == '#' {
                // Skip hashes and find opening quote
                while chars.peek() == Some(&'#') {
                    result.push(chars.next().unwrap());
                }
            }
            // Now inside raw string
            if let Some(&'"') = chars.peek() {
                result.push(chars.next().unwrap());
                in_string = true;
                string_char = '"';
            }
            continue;
        }

        // Detect comments
        if ch == '/' {
            if chars.peek() == Some(&'/') {
                // Line comment (// or /// or //!)
                chars.next(); // consume second /
                while chars.peek().is_some() && chars.peek() != Some(&'\n') {
                    chars.next(); // skip entire line
                }
                // Preserve newline for structure
                if let Some('\n') = chars.peek() {
                    result.push('\n');
                    chars.next();
                }
                continue;
            } else if chars.peek() == Some(&'*') {
                // Block comment (/* or /** or /*!)
                chars.next(); // consume *
                while let Some(curr) = chars.next() {
                    if curr == '*' && chars.peek() == Some(&'/') {
                        chars.next(); // consume /
                        break;
                    }
                }
                continue;
            }
        }

        result.push(ch);
    }

    Ok(result)
}

/// Reconstruct minified code from token stream with minimal spacing.
fn reconstruct_minimal(tokens: TokenStream) -> String {
    let mut result = String::new();
    let mut prev_token = String::new();

    for token in tokens {
        let token_str = token.to_string();

        // Add space between tokens only if necessary
        if !prev_token.is_empty() && needs_space_between(&prev_token, &token_str) {
            result.push(' ');
        }

        result.push_str(&token_str);
        prev_token = token_str;
    }

    result
}

/// Determine if whitespace is required between two consecutive tokens.
fn needs_space_between(prev: &str, next: &str) -> bool {
    let prev_last = prev.chars().last().unwrap_or(' ');
    let next_first = next.chars().next().unwrap_or(' ');

    // No space needed if either token is a bracket or punctuation (with exceptions)
    let prev_is_ident = prev_last.is_alphanumeric() || prev_last == '_' || prev_last == ')' || prev_last == ']' || prev_last == '}';
    let next_is_ident = next_first.is_alphanumeric() || next_first == '_' || next_first == '(' || next_first == '[' || next_first == '{';

    // Space needed if both look like identifiers or delimiters
    prev_is_ident && next_is_ident
}

/// Check if the context suggests this is a character literal (e.g., 'a') not a string.
fn is_char_literal(context: &str) -> bool {
    let trimmed = context.trim_end();
    if trimmed.is_empty() {
        return false;
    }
    let last_char = trimmed.chars().last().unwrap_or(' ');
    matches!(last_char, '(' | ',' | '=' | ':' | '{' | '[' | ' ')
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
