use super::Minifier;
use anyhow::Result;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

pub struct RustMinifier;

impl Minifier for RustMinifier {
    fn minify(
        &self,
        source: &str,
    ) -> Result<String> {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            return Ok(String::new());
        }

        let no_comments = remove_comments_and_rustdocs(trimmed)?;
        let tokens: TokenStream =
            no_comments.parse().map_err(|e| {
                anyhow::anyhow!("Failed to parse tokens: {}", e)
            })?;
        let minified = reconstruct_minimal(tokens);
        Ok(minified)
    }
}

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

        if ch == 'r'
            && matches!(chars.peek(), Some(&'"') | Some(&'#'))
        {
            result.push(ch);
            if chars.peek() == Some(&'"') {
                result.push(chars.next().unwrap());
                in_string = true;
                string_char = '"';
                raw_string_hashes = 0;
            } else if chars.peek() == Some(&'#') {
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

        if (ch == '"' || ch == '\'') && !in_string {
            in_string = true;
            string_char = ch;
            result.push(ch);
            continue;
        }

        if ch == '/' {
            if chars.peek() == Some(&'/') {
                chars.next();
                let is_rustdoc = chars.peek() == Some(&'/')
                    || chars.peek() == Some(&'!');
                if is_rustdoc {
                    chars.next();
                }
                while let Some(&c) = chars.peek() {
                    if c == '\n' {
                        break;
                    }
                    chars.next();
                }
                if chars.peek() == Some(&'\n') {
                    result.push('\n');
                    chars.next();
                }
                continue;
            } else if chars.peek() == Some(&'*') {
                chars.next();
                let is_rustdoc = chars.peek() == Some(&'*')
                    || chars.peek() == Some(&'!');
                if is_rustdoc {
                    chars.next();
                }
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

fn reconstruct_minimal(tokens: TokenStream) -> String {
    let mut result = String::new();
    let mut prev_token: Option<TokenTree> = None;

    for token in tokens.into_iter() {
        if let Some(ref prev) = prev_token {
            if needs_space_between(prev, &token) {
                result.push(' ');
            }
        }

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
                result.push_str(&reconstruct_minimal(g.stream()));
                if delim_close != '\0' {
                    result.push(delim_close);
                }
            }
            _ => {
                result.push_str(&token.to_string());
            }
        }

        prev_token = Some(token);
    }

    result
}

fn needs_space_between(
    prev_token: &TokenTree,
    next_token: &TokenTree,
) -> bool {
    use proc_macro2::TokenTree::*;
    match (prev_token, next_token) {
        (Ident(..), Ident(..)) => true,
        (Ident(..), Literal(..)) => true,
        (Ident(..), Group(..)) => false,
        (Literal(..), Ident(..)) => true,
        (Literal(..), Literal(..)) => true,
        (Literal(..), Group(..)) => false,
        (Group(..), _) => false,
        (Punct(..), _) => false,
        _ => false,
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
