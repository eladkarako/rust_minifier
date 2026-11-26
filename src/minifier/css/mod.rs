//! CSS, SCSS and LESS minifier.
//!
//! Selector descendant spaces are meaningful, so the reconstruction step is
//! context aware: it keeps spaces between words and around selector colons,
//! while removing them around braces, commas, semicolons and declaration
//! colons. `url(...)` contents are protected because an unquoted URL can
//! contain `//`.

use crate::minifier::options::MinifierOptions;
use crate::minifier::Minifier;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssDialect {
    Css,
    Scss,
    Less,
}

impl CssDialect {
    fn has_line_comments(self) -> bool {
        matches!(self, CssDialect::Scss | CssDialect::Less)
    }
}

pub struct CssMinifier {
    pub dialect: CssDialect,
}

impl Minifier for CssMinifier {
    fn minify(
        &self,
        source: &str,
        options: &MinifierOptions,
    ) -> Result<String> {
        Ok(minify_css(source, self.dialect, options))
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

fn starts_with_ignore_case(
    source: &[char],
    position: usize,
    needle: &str,
) -> bool {
    let mut offset = 0usize;
    for expected in needle.chars() {
        let Some(actual) = source.get(position + offset) else {
            return false;
        };
        if actual.to_ascii_lowercase()
            != expected.to_ascii_lowercase()
        {
            return false;
        }
        offset += 1;
    }
    true
}

fn consume_string(
    chars: &[char],
    position: &mut usize,
    quote: char,
) -> String {
    let mut text = String::new();
    if let Some(character) = chars.get(*position).copied() {
        text.push(character);
        *position += 1;
    }
    while let Some(character) = chars.get(*position).copied() {
        text.push(character);
        *position += 1;
        if character == '\\' {
            if let Some(escaped) = chars.get(*position).copied() {
                text.push(escaped);
                *position += 1;
            }
            continue;
        }
        if character == quote {
            break;
        }
    }
    text
}

fn consume_url(
    chars: &[char],
    position: &mut usize,
) -> String {
    let mut text = String::new();
    let mut depth = 0usize;
    while let Some(character) = chars.get(*position).copied() {
        text.push(character);
        *position += 1;
        if character == '(' {
            depth += 1;
        } else if character == ')' {
            depth -= 1;
            if depth == 0 {
                break;
            }
        } else if character == '"' || character == '\'' {
            // A quoted URL still needs the quote and its closing pair copied.
            let quote = character;
            while let Some(inner) = chars.get(*position).copied() {
                text.push(inner);
                *position += 1;
                if inner == '\\' {
                    if let Some(escaped) =
                        chars.get(*position).copied()
                    {
                        text.push(escaped);
                        *position += 1;
                    }
                    continue;
                }
                if inner == quote {
                    break;
                }
            }
        }
    }
    text
}

fn is_word_character(character: char) -> bool {
    character.is_alphanumeric()
        || character == '_'
        || character == '-'
}

fn starts_word(
    chars: &[char],
    position: usize,
) -> bool {
    let character = chars[position];
    if character == '-' {
        return chars
            .get(position + 1)
            .is_some_and(|next| {
                next.is_alphanumeric()
                    || *next == '_'
                    || *next == '-'
            });
    }
    character.is_alphanumeric() || character == '_'
}

fn tokenize(
    source: &str,
    dialect: CssDialect,
) -> Vec<Token> {
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

        if dialect.has_line_comments()
            && character == '/'
            && chars.get(position + 1) == Some(&'/')
        {
            position += 2;
            while position < chars.len()
                && chars[position] != '\n'
                && chars[position] != '\r'
            {
                position += 1;
            }
            had_space = true;
            continue;
        }

        if character == '"' || character == '\'' {
            let text =
                consume_string(&chars, &mut position, character);
            tokens.push(Token {
                kind: TokenKind::String,
                text,
                preceded_by_space: had_space,
                preceded_by_newline: had_newline,
            });
            had_space = false;
            had_newline = false;
            continue;
        }

        if starts_with_ignore_case(&chars, position, "url(") {
            let text = consume_url(&chars, &mut position);
            tokens.push(Token {
                kind: TokenKind::String,
                text,
                preceded_by_space: had_space,
                preceded_by_newline: had_newline,
            });
            had_space = false;
            had_newline = false;
            continue;
        }

        if starts_word(&chars, position) {
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

fn is_function_name(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "calc"
            | "var"
            | "url"
            | "rgb"
            | "rgba"
            | "hsl"
            | "hsla"
            | "hwb"
            | "lab"
            | "lch"
            | "oklab"
            | "oklch"
            | "color"
            | "color-mix"
            | "min"
            | "max"
            | "clamp"
            | "attr"
            | "counter"
            | "counters"
            | "env"
            | "cubic-bezier"
            | "linear-gradient"
            | "radial-gradient"
            | "conic-gradient"
            | "repeating-linear-gradient"
            | "repeating-radial-gradient"
            | "repeating-conic-gradient"
            | "image-set"
            | "cross-fade"
            | "element"
            | "paint"
            | "path"
            | "polygon"
            | "circle"
            | "ellipse"
            | "inset"
            | "matrix"
            | "matrix3d"
            | "translate"
            | "translate3d"
            | "translatex"
            | "translatey"
            | "translatez"
            | "scale"
            | "scale3d"
            | "scalex"
            | "scaley"
            | "scalez"
            | "rotate"
            | "rotate3d"
            | "rotatex"
            | "rotatey"
            | "rotatez"
            | "skew"
            | "skewx"
            | "skewy"
            | "perspective"
            | "steps"
            | "blur"
            | "brightness"
            | "contrast"
            | "drop-shadow"
            | "grayscale"
            | "hue-rotate"
            | "invert"
            | "opacity"
            | "saturate"
            | "sepia"
            | "format"
            | "local"
            | "supports"
            | "selector"
            | "not"
            | "is"
            | "where"
            | "has"
            | "nth-child"
            | "nth-last-child"
            | "nth-of-type"
            | "nth-last-of-type"
            | "lang"
            | "dir"
            | "host"
            | "host-context"
            | "slotted"
            | "part"
            | "theme"
    )
}

fn dangerous_pair(
    left: char,
    right: char,
) -> bool {
    matches!(
        [left, right],
        ['/', '*']
            | ['/', '/']
            | ['-', '-']
            | [':', ':']
            | ['=', '=']
            | ['>', '=']
            | ['<', '=']
            | ['!', '=']
            | ['~', '=']
            | ['|', '=']
            | ['^', '=']
            | ['$', '=']
            | ['*', '=']
            | ['+', '=']
            | ['.', '.']
            | ['@', '@']
            | ['#', '#']
    )
}

fn colon_is_declaration(
    tokens: &[Token],
    index: usize,
) -> bool {
    let mut parentheses = 0isize;
    let mut braces = 0isize;
    for token in tokens.iter().skip(index + 1) {
        match &token.kind {
            TokenKind::Punct('(') => parentheses += 1,
            TokenKind::Punct(')') => {
                parentheses -= 1;
                if parentheses < 0 {
                    return true;
                }
            }
            TokenKind::Punct('{') => {
                if parentheses == 0 {
                    if braces == 0 {
                        return false;
                    }
                    braces += 1;
                }
            }
            TokenKind::Punct('}') => {
                if parentheses == 0 {
                    if braces == 0 {
                        return true;
                    }
                    braces -= 1;
                }
            }
            TokenKind::Punct(';')
            if parentheses == 0 && braces == 0 =>
                {
                    return true;
                }
            _ => {}
        }
    }
    true
}

fn needs_space(
    previous: &Token,
    next: &Token,
    tokens: &[Token],
    next_index: usize,
) -> bool {
    if !next.preceded_by_space {
        return false;
    }
    match (&previous.kind, &next.kind) {
        (TokenKind::Word, TokenKind::Word) => true,
        (TokenKind::Word, TokenKind::String) => true,
        (TokenKind::String, TokenKind::Word) => true,
        (TokenKind::String, TokenKind::String) => true,
        (TokenKind::Word, TokenKind::Punct('(')) => {
            !is_function_name(&previous.text)
        }
        (TokenKind::Word, TokenKind::Punct(':')) => {
            !colon_is_declaration(tokens, next_index)
        }
        (TokenKind::Word, TokenKind::Punct(character)) => matches!(
            character,
            '+' | '-' | '*' | '/' | '.' | '#' | '[' | '~' | '>'
        ),
        (TokenKind::Punct(':'), TokenKind::Word) => {
            !colon_is_declaration(tokens, next_index - 1)
        }
        (TokenKind::Punct(character), TokenKind::Word) => matches!(
            character,
            '+' | '-' | '*' | '/' | ')' | ']' | '~' | '>'
        ),
        (TokenKind::Punct(')'), TokenKind::Punct('.'))
        | (TokenKind::Punct(')'), TokenKind::Punct('#'))
        | (TokenKind::Punct(')'), TokenKind::Punct('['))
        | (TokenKind::Punct(')'), TokenKind::Punct(':'))
        | (TokenKind::Punct(']'), TokenKind::Punct('.'))
        | (TokenKind::Punct(']'), TokenKind::Punct('#'))
        | (TokenKind::Punct(']'), TokenKind::Punct('['))
        | (TokenKind::Punct(']'), TokenKind::Punct(':')) => true,
        (TokenKind::Punct(left), TokenKind::Punct(right)) => {
            dangerous_pair(*left, *right)
                || matches!(left, '+' | '-' | '*' | '/')
                || matches!(right, '+' | '-' | '*' | '/')
        }
        _ => false,
    }
}

fn reconstruct(
    tokens: &[Token],
    options: &MinifierOptions,
) -> String {
    let mut output = String::new();
    let mut previous: Option<&Token> = None;

    for (index, token) in tokens.iter().enumerate() {
        if token.preceded_by_newline && options.preserve_line_breaks
        {
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
        } else if let Some(previous_token) = previous {
            if needs_space(previous_token, token, tokens, index)
                && !output.ends_with(' ')
                && !output.ends_with('\n')
            {
                output.push(' ');
            }
        }

        if token.kind == TokenKind::Punct('}') {
            while output.ends_with(';') {
                output.pop();
            }
        }

        output.push_str(&token.text);
        previous = Some(token);
    }

    while output.ends_with(' ') {
        output.pop();
    }
    output
}

/// Minify a CSS-like document.
pub fn minify_css(
    source: &str,
    dialect: CssDialect,
    options: &MinifierOptions,
) -> String {
    let source = source.trim_start_matches('\u{feff}');
    if source.trim().is_empty() {
        return String::new();
    }
    let tokens = tokenize(source, dialect);
    reconstruct(&tokens, options)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
