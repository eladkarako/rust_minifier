//! SQL minifier.
//!
//! Handles `--` line comments, `/* */` block comments, single quoted strings
//! with `''` escapes, double quoted and backtick identifiers, SQL Server
//! `[...]` identifiers and PostgreSQL dollar quoted strings. Whitespace is
//! collapsed and removed around punctuation while word boundaries are kept.

use crate::minifier::options::MinifierOptions;
use crate::minifier::Minifier;
use anyhow::Result;

pub struct SqlMinifier;

impl Minifier for SqlMinifier {
    fn minify(
        &self,
        source: &str,
        options: &MinifierOptions,
    ) -> Result<String> {
        Ok(minify_sql(source, options))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Word,
    String,
    Punct(char),
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    text: String,
    preceded_by_space: bool,
    preceded_by_newline: bool,
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

fn consume_quoted(
    chars: &[char],
    start: usize,
    quote: char,
    close: char,
    allow_backslash: bool,
    doubled_escape: bool,
) -> (String, usize) {
    let mut text = String::new();
    text.push(chars[start]);
    let mut position = start + 1;
    while position < chars.len() {
        let character = chars[position];
        text.push(character);
        position += 1;
        if allow_backslash && character == '\\' {
            if let Some(next) = chars.get(position).copied() {
                text.push(next);
                position += 1;
            }
            continue;
        }
        if character == close {
            if doubled_escape && chars.get(position) == Some(&close)
            {
                text.push(close);
                position += 1;
                continue;
            }
            break;
        }
        let _ = quote;
    }
    (text, position)
}

fn consume_dollar_quote(
    chars: &[char],
    start: usize,
) -> Option<(String, usize)> {
    let mut cursor = start + 1;
    while cursor < chars.len()
        && (chars[cursor].is_alphanumeric() || chars[cursor] == '_')
    {
        cursor += 1;
    }
    if chars.get(cursor) != Some(&'$') {
        return None;
    }
    let tag: String = chars[start..=cursor].iter().collect();
    let mut text = tag.clone();
    let mut position = cursor + 1;
    while position < chars.len() {
        if starts_with_at(chars, position, &tag) {
            text.push_str(&tag);
            position += tag.chars().count();
            return Some((text, position));
        }
        text.push(chars[position]);
        position += 1;
    }
    Some((text, position))
}

fn is_word_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn tokenize(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut position = 0usize;
    let mut had_space = false;
    let mut had_newline = false;

    while position < chars.len() {
        let character = chars[position];

        if character.is_whitespace() {
            had_space = true;
            if character == '\n' || character == '\r' {
                had_newline = true;
            }
            position += 1;
            continue;
        }

        if character == '-' && chars.get(position + 1) == Some(&'-')
        {
            while position < chars.len()
                && chars[position] != '\n'
                && chars[position] != '\r'
            {
                position += 1;
            }
            had_space = true;
            continue;
        }

        if character == '#' {
            while position < chars.len()
                && chars[position] != '\n'
                && chars[position] != '\r'
            {
                position += 1;
            }
            had_space = true;
            continue;
        }

        if character == '/' && chars.get(position + 1) == Some(&'*')
        {
            position += 2;
            while position < chars.len() {
                if chars[position] == '*'
                    && chars.get(position + 1) == Some(&'/')
                {
                    position += 2;
                    break;
                }
                if chars[position] == '\n' || chars[position] == '\r'
                {
                    had_newline = true;
                }
                position += 1;
            }
            had_space = true;
            continue;
        }

        if character == '\'' {
            let (text, next) = consume_quoted(
                &chars, position, '\'', '\'', true, true,
            );
            tokens.push(Token {
                kind: TokenKind::String,
                text,
                preceded_by_space: had_space,
                preceded_by_newline: had_newline,
            });
            position = next;
            had_space = false;
            had_newline = false;
            continue;
        }

        if character == '"' {
            let (text, next) = consume_quoted(
                &chars, position, '"', '"', true, true,
            );
            tokens.push(Token {
                kind: TokenKind::String,
                text,
                preceded_by_space: had_space,
                preceded_by_newline: had_newline,
            });
            position = next;
            had_space = false;
            had_newline = false;
            continue;
        }

        if character == '`' {
            let (text, next) = consume_quoted(
                &chars, position, '`', '`', false, true,
            );
            tokens.push(Token {
                kind: TokenKind::String,
                text,
                preceded_by_space: had_space,
                preceded_by_newline: had_newline,
            });
            position = next;
            had_space = false;
            had_newline = false;
            continue;
        }

        if character == '[' {
            let (text, next) = consume_quoted(
                &chars, position, '[', ']', false, true,
            );
            tokens.push(Token {
                kind: TokenKind::String,
                text,
                preceded_by_space: had_space,
                preceded_by_newline: had_newline,
            });
            position = next;
            had_space = false;
            had_newline = false;
            continue;
        }

        if character == '$' {
            if let Some((text, next)) =
                consume_dollar_quote(&chars, position)
            {
                tokens.push(Token {
                    kind: TokenKind::String,
                    text,
                    preceded_by_space: had_space,
                    preceded_by_newline: had_newline,
                });
                position = next;
                had_space = false;
                had_newline = false;
                continue;
            }
        }

        if is_word_character(character) {
            let mut text = String::new();
            while position < chars.len()
                && is_word_character(chars[position])
            {
                text.push(chars[position]);
                position += 1;
            }
            tokens.push(Token {
                kind: TokenKind::Word,
                text,
                preceded_by_space: had_space,
                preceded_by_newline: had_newline,
            });
            had_space = false;
            had_newline = false;
            continue;
        }

        tokens.push(Token {
            kind: TokenKind::Punct(character),
            text: character.to_string(),
            preceded_by_space: had_space,
            preceded_by_newline: had_newline,
        });
        position += 1;
        had_space = false;
        had_newline = false;
    }

    tokens
}

fn needs_space(
    previous: &Token,
    next: &Token,
) -> bool {
    if !next.preceded_by_space {
        return false;
    }
    if let TokenKind::Punct(character) = next.kind {
        if matches!(character, ',' | ';' | ')' | ']' | '.') {
            return false;
        }
        if character == '(' {
            return false;
        }
    }
    if let TokenKind::Punct(character) = previous.kind {
        if matches!(character, '(' | '[' | '.') {
            return false;
        }
    }
    true
}

fn reconstruct(
    tokens: &[Token],
    options: &MinifierOptions,
) -> String {
    let mut output = String::new();
    let mut previous: Option<&Token> = None;

    for token in tokens {
        if options.preserve_line_breaks && token.preceded_by_newline
        {
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
        } else if let Some(previous_token) = previous {
            if needs_space(previous_token, token)
                && !output.ends_with(' ')
                && !output.ends_with('\n')
            {
                output.push(' ');
            }
        }
        output.push_str(&token.text);
        previous = Some(token);
    }

    output.trim().to_string()
}

/// Minify a SQL document.
pub fn minify_sql(
    source: &str,
    options: &MinifierOptions,
) -> String {
    let source = source.trim_start_matches('\u{feff}');
    if source.trim().is_empty() {
        return String::new();
    }
    let tokens = tokenize(source);
    reconstruct(&tokens, options)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
