use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

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
    let tokens: TokenStream = no_comments.parse().map_err(|e| {
        anyhow::anyhow!("Failed to parse tokens: {}", e)
    })?;

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
    let mut raw_string_hashes = 0;

    while let Some(ch) = chars.next() {
        if in_string && escape_next {
            result.push(ch);
            escape_next = false;
            continue;
        }

        if in_string {
            result.push(ch);
            if ch == '\\' {
                escape_next = true;
            } else if ch == string_char {
                if raw_string_hashes > 0 {
                    let mut hash_count = 0;
                    while chars.peek() == Some(&'#')
                        && hash_count < raw_string_hashes
                    {
                        result.push(chars.next().unwrap());
                        hash_count += 1;
                    }
                    if hash_count == raw_string_hashes {
                        in_string = false;
                        raw_string_hashes = 0;
                    }
                } else {
                    in_string = false;
                }
            }
            continue;
        }

        // Handle raw strings
        if ch == 'r'
            && matches!(chars.peek(), Some(&'"') | Some(&'#'))
        {
            result.push(ch);
            // Check what comes next
            if chars.peek() == Some(&'"') {
                // r"..."
                result.push(chars.next().unwrap());
                in_string = true;
                string_char = '"';
                raw_string_hashes = 0;
            } else if chars.peek() == Some(&'#') {
                // r#"..."# or r##"..."## etc.
                let mut hash_count = 0;
                while chars.peek() == Some(&'#') {
                    result.push(chars.next().unwrap());
                    hash_count += 1;
                }
                if chars.peek() == Some(&'"') {
                    result.push(chars.next().unwrap());
                    in_string = true;
                    string_char = '"';
                    raw_string_hashes = hash_count;
                }
            }
            continue;
        }

        // Handle regular strings
        if (ch == '"' || ch == '\'') && !in_string {
            in_string = true;
            string_char = ch;
            result.push(ch);
            continue;
        }

        // Handle comments
        if ch == '/' {
            if chars.peek() == Some(&'/') {
                // Single-line comment or rustdoc (// or /// or //!)
                chars.next(); // consume second '/'
                // Check if it's a rustdoc
                let is_rustdoc = chars.peek() == Some(&'/')
                    || chars.peek() == Some(&'!');
                if is_rustdoc {
                    chars.next(); // consume the third character
                }
                // Skip until end of line
                while let Some(&c) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }
                // Preserve newline for structure
                if chars.peek() == Some(&'\n') {
                    result.push('\n');
                    chars.next();
                }
                continue;
            } else if chars.peek() == Some(&'*') {
                // Multi-line comment or rustdoc (/* or /** or /*!)
                chars.next(); // consume '*'
                let is_rustdoc = chars.peek() == Some(&'*')
                    || chars.peek() == Some(&'!');
                if is_rustdoc {
                    chars.next(); // consume the extra character
                }
                // Skip until */
                let mut prev_ch = ' ';
                while let Some(c) = chars.next() {
                    if prev_ch == '*' && c == '/' {
                        break;
                    }
                    prev_ch = c;
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
    let mut prev_token: Option<TokenTree> = None;

    for token in tokens.into_iter() {
        // Add space between tokens if needed
        if let Some(ref prev) = prev_token {
            if needs_space_between(prev, &token) {
                result.push(' ');
            }
        }

        // Handle each token type separately to avoid proc_macro2's internal spacing
        match &token {
            TokenTree::Punct(p) => {
                result.push(p.as_char());
            }
            TokenTree::Group(g) => {
                let delim_open = match g.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => '(',
                    proc_macro2::Delimiter::Brace => '{',
                    proc_macro2::Delimiter::Bracket => '[',
                    proc_macro2::Delimiter::None => '\0',
                };
                let delim_close = match g.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => ')',
                    proc_macro2::Delimiter::Brace => '}',
                    proc_macro2::Delimiter::Bracket => ']',
                    proc_macro2::Delimiter::None => '\0',
                };

                if delim_open != '\0' {
                    result.push(delim_open);
                }
                // Recursively reconstruct the inner stream
                result.push_str(&reconstruct_minimal(g.stream()));
                if delim_close != '\0' {
                    result.push(delim_close);
                }
            }
            _ => {
                // For Ident and Literal, to_string() is safe
                result.push_str(&token.to_string());
            }
        }

        prev_token = Some(token);
    }

    result
}

/// Determine if a space is required between two tokens.
fn needs_space_between(
    prev_token: &TokenTree,
    next_token: &TokenTree,
) -> bool {
    use proc_macro2::TokenTree::*;

    match (prev_token, next_token) {
        // Ident to Ident = need space
        (Ident(..), Ident(..)) => true,
        // Ident to Literal = need space
        (Ident(..), Literal(..)) => true,
        // Ident to Group = no space (ident(...))
        (Ident(..), Group(..)) => false,

        // Literal to Ident = need space
        (Literal(..), Ident(..)) => true,
        // Literal to Literal = need space
        (Literal(..), Literal(..)) => true,
        // Literal to Group = no space
        (Literal(..), Group(..)) => false,

        // Group to anything = NO space
        (Group(..), _) => false,

        // Never add space after punctuation — Rust's lexer handles it
        (Punct(..), _) => false,

        // Default: no space
        _ => false,
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
