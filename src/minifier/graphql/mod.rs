//! GraphQL minifier.
//!
//! Removes `#` comments and insignificant whitespace while keeping string and
//! block string contents verbatim. Word boundaries are preserved so `query
//! Name` and `on Type` stay separated.

use crate::minifier::options::MinifierOptions;
use crate::minifier::Minifier;
use anyhow::Result;

pub struct GraphQlMinifier;

impl Minifier for GraphQlMinifier {
    fn minify(
        &self,
        source: &str,
        options: &MinifierOptions,
    ) -> Result<String> {
        Ok(minify_graphql(source, options))
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

fn consume_string(chars: &[char], start: usize) -> (String, usize) {
    // Block string: triple double quote.
    if chars.get(start + 1) == Some(&'"')
        && chars.get(start + 2) == Some(&'"')
    {
        let mut text = String::new();
        for _ in 0..3 {
            if let Some(character) = chars.get(start + text.chars().count()) {
                text.push(*character);
            }
        }
        let mut position = start + 3;
        while position < chars.len() {
            if chars[position] == '"'
                && chars.get(position + 1) == Some(&'"')
                && chars.get(position + 2) == Some(&'"')
            {
                text.push('"');
                text.push('"');
                text.push('"');
                position += 3;
                return (text, position);
            }
            if chars[position] == '\\'
                && chars.get(position + 1) == Some(&'"')
            {
                text.push('\\');
                text.push('"');
                position += 2;
                continue;
            }
            text.push(chars[position]);
            position += 1;
        }
        return (text, position);
    }

    let mut text = String::new();
    text.push('"');
    let mut position = start + 1;
    while position < chars.len() {
        let character = chars[position];
        text.push(character);
        position += 1;
        if character == '\\' {
            if let Some(next) = chars.get(position).copied() {
                text.push(next);
                position += 1;
            }
            continue;
        }
        if character == '"' {
            break;
        }
    }
    (text, position)
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

        if character == '"' {
            let (text, next) = consume_string(&chars, position);
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

fn is_word_like(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Word | TokenKind::String)
}

fn needs_space(previous: &Token, next: &Token) -> bool {
    next.preceded_by_space && is_word_like(previous) && is_word_like(next)
}

fn reconstruct(tokens: &[Token], options: &MinifierOptions) -> String {
    let mut output = String::new();
    let mut previous: Option<&Token> = None;

    for token in tokens {
        if options.preserve_line_breaks && token.preceded_by_newline {
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

/// Minify a GraphQL document.
pub fn minify_graphql(
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
