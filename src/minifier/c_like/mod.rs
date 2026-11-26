//! Configurable C-like minifier.
//!
//! The lexer removes comments first, then rebuilds the token stream with the
//! smallest safe amount of whitespace. It understands quoted, triple-quoted,
//! raw, verbatim, backtick and long-bracket strings so comment markers inside
//! literals are never touched.

pub mod profile;

use crate::minifier::options::MinifierOptions;
use crate::minifier::Minifier;
use anyhow::Result;
use profile::{CLikeProfile, StatementSeparator};

/// A C-like minifier bound to one syntax profile.
pub struct CLikeMinifier {
    pub profile: &'static CLikeProfile,
}

impl Minifier for CLikeMinifier {
    fn minify(
        &self,
        source: &str,
        options: &MinifierOptions,
    ) -> Result<String> {
        Ok(minify_c_like(source, self.profile, options))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Word,
    String,
    Punct(char),
    Preprocessor,
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    text: String,
    preceded_by_space: bool,
    preceded_by_newline: bool,
}

struct Lexer<'a> {
    chars: Vec<char>,
    position: usize,
    profile: &'static CLikeProfile,
    at_line_start: bool,
    last_token: Option<Token>,
    _source: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(
        source: &'a str,
        profile: &'static CLikeProfile,
    ) -> Self {
        Self {
            chars: source.chars().collect(),
            position: 0,
            profile,
            at_line_start: true,
            last_token: None,
            _source: source,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    fn peek_at(
        &self,
        offset: usize,
    ) -> Option<char> {
        self.chars
            .get(self.position + offset)
            .copied()
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.peek()?;
        self.position += 1;
        Some(character)
    }

    fn starts_with(
        &self,
        text: &str,
    ) -> bool {
        let mut offset = 0;
        for expected in text.chars() {
            if self.peek_at(offset) != Some(expected) {
                return false;
            }
            offset += 1;
        }
        true
    }

    fn consume_while<F>(
        &mut self,
        mut predicate: F,
    ) -> String
        where
            F: FnMut(char) -> bool,
    {
        let mut result = String::new();
        while let Some(character) = self.peek() {
            if !predicate(character) {
                break;
            }
            result.push(character);
            self.position += 1;
        }
        result
    }

    fn skip_whitespace(&mut self) -> (bool, bool) {
        let mut had_space = false;
        let mut had_newline = false;
        while let Some(character) = self.peek() {
            if !character.is_whitespace() {
                break;
            }
            had_space = true;
            if character == '\n' || character == '\r' {
                had_newline = true;
                self.at_line_start = true;
            }
            self.position += 1;
        }
        (had_space, had_newline)
    }

    fn try_consume_comment(&mut self) -> Option<bool> {
        // Block comments first: Lua's `--[[` would otherwise match `--`.
        for (open, close) in self.profile.block_comments {
            if self.starts_with(open) {
                let had_newline =
                    self.consume_block_comment(open, close);
                return Some(had_newline);
            }
        }
        for marker in self.profile.line_comments {
            if self.starts_with(marker) {
                // PHP 8 attributes start with `#[`; they are code, not a
                // `#` comment.
                if *marker == "#" && self.peek_at(1) == Some('[') {
                    continue;
                }
                self.consume_line_comment();
                return Some(false);
            }
        }
        None
    }

    fn consume_line_comment(&mut self) {
        while let Some(character) = self.peek() {
            if character == '\n' || character == '\r' {
                break;
            }
            self.position += 1;
        }
    }

    fn consume_block_comment(
        &mut self,
        open: &str,
        close: &str,
    ) -> bool {
        for _ in open.chars() {
            self.advance();
        }
        let mut had_newline = false;
        let mut depth = 1usize;
        while let Some(character) = self.peek() {
            if self.profile.nested_block_comments
                && self.starts_with(open)
            {
                for _ in open.chars() {
                    self.advance();
                }
                depth += 1;
                continue;
            }
            if self.starts_with(close) {
                for _ in close.chars() {
                    self.advance();
                }
                depth -= 1;
                if depth == 0 {
                    break;
                }
                continue;
            }
            if character == '\n' || character == '\r' {
                self.at_line_start = true;
                had_newline = true;
            }
            self.advance();
        }
        had_newline
    }

    fn try_consume_preprocessor(&mut self) -> Option<String> {
        let prefix = self.profile.preprocessor_prefix?;
        if !self.at_line_start || self.peek() != Some(prefix) {
            return None;
        }
        let mut text = String::new();
        while let Some(character) = self.peek() {
            if character == '\n' || character == '\r' {
                break;
            }
            text.push(character);
            self.position += 1;
        }
        // Consume the line ending so the next token starts on a new line.
        if self.peek() == Some('\r') {
            self.advance();
        }
        if self.peek() == Some('\n') {
            self.advance();
        }
        self.at_line_start = true;
        Some(text)
    }

    fn try_consume_long_bracket_string(&mut self) -> Option<String> {
        if !self.profile.long_bracket_strings
            || self.peek() != Some('[')
        {
            return None;
        }
        let mut offset = 1usize;
        let mut equals_count = 0usize;
        while self.peek_at(offset) == Some('=') {
            equals_count += 1;
            offset += 1;
        }
        if self.peek_at(offset) != Some('[') {
            return None;
        }

        let mut text = String::new();
        for _ in 0..=offset {
            if let Some(character) = self.advance() {
                text.push(character);
            }
        }

        loop {
            match self.peek() {
                None => break,
                Some(']') => {
                    let mut matches = true;
                    for index in 0..equals_count {
                        if self.peek_at(1 + index) != Some('=') {
                            matches = false;
                            break;
                        }
                    }
                    if matches
                        && self.peek_at(1 + equals_count)
                        == Some(']')
                    {
                        for _ in 0..(equals_count + 2) {
                            if let Some(character) = self.advance() {
                                text.push(character);
                            }
                        }
                        break;
                    }
                    if let Some(character) = self.advance() {
                        text.push(character);
                    }
                }
                Some(character) => {
                    if character == '\n' || character == '\r' {
                        self.at_line_start = true;
                    }
                    text.push(character);
                    self.position += 1;
                }
            }
        }
        Some(text)
    }

    fn try_consume_string(&mut self) -> Option<String> {
        if let Some(text) = self.try_consume_long_bracket_string() {
            return Some(text);
        }

        if self.profile.raw_hash_strings && self.peek() == Some('#')
        {
            let mut hashes = 0usize;
            while self.peek_at(hashes) == Some('#') {
                hashes += 1;
            }
            if self.peek_at(hashes) == Some('"') {
                return Some(
                    self.consume_raw_hash_string(hashes, '"'),
                );
            }
        }

        if let Some(character) = self.peek() {
            if self
                .profile
                .quote_chars
                .contains(&character)
            {
                return Some(self.consume_quote(character));
            }
            if character == '`' && self.profile.backtick_strings {
                return Some(self.consume_quote('`'));
            }
        }

        let mut prefixes: Vec<&'static str> = self
            .profile
            .prefixed_strings
            .iter()
            .copied()
            .collect();
        prefixes
            .sort_by_key(|prefix| std::cmp::Reverse(prefix.len()));
        for prefix in prefixes {
            if !self.starts_with(prefix) {
                continue;
            }
            let after = self.peek_at(prefix.chars().count());
            let after_two = self.peek_at(prefix.chars().count() + 1);
            let after_three =
                self.peek_at(prefix.chars().count() + 2);

            let is_cpp_raw = self.profile.cpp_raw_strings
                && prefix.ends_with('R')
                && after == Some('"');
            if is_cpp_raw {
                return Some(self.consume_cpp_raw_string(prefix));
            }

            let is_triple = after == Some('"')
                && after_two == Some('"')
                && after_three == Some('"');
            if is_triple {
                for _ in 0..prefix.chars().count() {
                    self.advance();
                }
                return Some(self.consume_triple_quote('"'));
            }

            if after == Some('"') || after == Some('\'') {
                let quote = after.unwrap();
                for _ in 0..prefix.chars().count() {
                    self.advance();
                }
                let mut text = String::from(prefix);
                let verbatim = prefix.contains('@');
                let raw = prefix == "r";
                if verbatim {
                    text.push_str(&self.consume_quote_with_options(
                        quote, false, true,
                    ));
                } else if raw {
                    text.push_str(&self.consume_quote_with_options(
                        quote, false, false,
                    ));
                } else {
                    text.push_str(&self.consume_quote(quote));
                }
                return Some(text);
            }

            if after == Some('#') && self.profile.raw_hash_strings {
                let mut hashes = 0usize;
                while self.peek_at(prefix.chars().count() + hashes)
                    == Some('#')
                {
                    hashes += 1;
                }
                let quote_offset = prefix.chars().count() + hashes;
                if self.peek_at(quote_offset) == Some('"') {
                    for _ in 0..prefix.chars().count() {
                        self.advance();
                    }
                    return Some(
                        self.consume_raw_hash_string(hashes, '"'),
                    );
                }
            }
        }
        None
    }

    fn consume_quote(
        &mut self,
        quote: char,
    ) -> String {
        if self
            .profile
            .triple_quote_chars
            .contains(&quote)
            && self.peek_at(1) == Some(quote)
            && self.peek_at(2) == Some(quote)
        {
            return self.consume_triple_quote(quote);
        }
        self.consume_quote_with_options(
            quote,
            self.profile.backslash_escapes,
            self.profile.doubled_quote_escapes,
        )
    }

    fn consume_quote_with_options(
        &mut self,
        quote: char,
        backslash_escapes: bool,
        doubled_quote_escapes: bool,
    ) -> String {
        let mut text = String::new();
        if let Some(character) = self.advance() {
            text.push(character);
        }
        while let Some(character) = self.advance() {
            text.push(character);
            if backslash_escapes && character == '\\' {
                if let Some(escaped) = self.advance() {
                    text.push(escaped);
                }
                continue;
            }
            if character == quote {
                if doubled_quote_escapes
                    && self.peek() == Some(quote)
                {
                    text.push(self.advance().unwrap());
                    continue;
                }
                break;
            }
        }
        text
    }

    fn consume_triple_quote(
        &mut self,
        quote: char,
    ) -> String {
        let mut text = String::new();
        for _ in 0..3 {
            if let Some(character) = self.advance() {
                text.push(character);
            }
        }
        while let Some(character) = self.advance() {
            text.push(character);
            if character == '\\' && self.profile.backslash_escapes {
                if let Some(escaped) = self.advance() {
                    text.push(escaped);
                }
                continue;
            }
            if character == quote
                && self.peek() == Some(quote)
                && self.peek_at(1) == Some(quote)
            {
                text.push(self.advance().unwrap());
                text.push(self.advance().unwrap());
                break;
            }
        }
        text
    }

    fn consume_raw_hash_string(
        &mut self,
        hashes: usize,
        quote: char,
    ) -> String {
        let mut text = String::new();
        for _ in 0..hashes {
            if let Some(character) = self.advance() {
                text.push(character);
            }
        }
        if let Some(character) = self.advance() {
            text.push(character);
        }
        loop {
            match self.peek() {
                None => break,
                Some(character) => {
                    text.push(character);
                    self.position += 1;
                    if character == quote {
                        let mut matches = true;
                        for index in 0..hashes {
                            if self.peek_at(index) != Some('#') {
                                matches = false;
                                break;
                            }
                        }
                        if matches {
                            for _ in 0..hashes {
                                if let Some(hash) = self.advance() {
                                    text.push(hash);
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
        text
    }

    fn consume_cpp_raw_string(
        &mut self,
        prefix: &str,
    ) -> String {
        let mut text = String::from(prefix);
        if let Some(character) = self.advance() {
            text.push(character); // opening quote
        }
        let mut delimiter = String::new();
        while let Some(character) = self.advance() {
            text.push(character);
            if character == '(' {
                break;
            }
            delimiter.push(character);
        }
        let closing = format!("){}\"", delimiter);
        while self.peek().is_some() {
            if self.starts_with(&closing) {
                for _ in 0..closing.chars().count() {
                    if let Some(character) = self.advance() {
                        text.push(character);
                    }
                }
                break;
            }
            if let Some(character) = self.advance() {
                text.push(character);
            }
        }
        text
    }

    fn try_consume_regex(&mut self) -> Option<String> {
        if !self.profile.regex_literals || self.peek() != Some('/') {
            return None;
        }
        if !regex_allowed_after(self.last_token.as_ref()) {
            return None;
        }

        let start_position = self.position;
        let mut text = String::new();
        text.push(self.advance().unwrap());
        let mut in_character_class = false;
        let mut escaped = false;
        let mut closed = false;
        while let Some(character) = self.advance() {
            text.push(character);
            if escaped {
                escaped = false;
                continue;
            }
            if character == '\\' {
                escaped = true;
                continue;
            }
            if character == '[' {
                in_character_class = true;
                continue;
            }
            if character == ']' {
                in_character_class = false;
                continue;
            }
            if character == '/' && !in_character_class {
                closed = true;
                break;
            }
            if character == '\n' || character == '\r' {
                break;
            }
        }
        if !closed {
            self.position = start_position;
            return None;
        }
        while let Some(character) = self.peek() {
            if character.is_alphabetic() {
                text.push(character);
                self.position += 1;
            } else {
                break;
            }
        }
        Some(text)
    }

    fn try_consume_word(&mut self) -> Option<String> {
        let extra_characters = self.profile.identifier_extra_chars;
        let first = self.peek()?;
        if !is_word_start(first, extra_characters) {
            return None;
        }
        Some(self.consume_while(|character| {
            is_word_start(character, extra_characters)
        }))
    }
}

fn is_word_start(
    character: char,
    extra_characters: &str,
) -> bool {
    character.is_alphanumeric()
        || character == '_'
        || extra_characters.contains(character)
}

fn regex_allowed_after(previous: Option<&Token>) -> bool {
    let Some(previous) = previous else {
        return true;
    };
    match &previous.kind {
        TokenKind::Word => matches!(
            previous.text.as_str(),
            "return"
                | "typeof"
                | "instanceof"
                | "in"
                | "of"
                | "new"
                | "delete"
                | "void"
                | "throw"
                | "case"
                | "do"
                | "else"
                | "yield"
                | "await"
        ),
        TokenKind::Punct(character) => matches!(
            character,
            '(' | '['
                | '{'
                | ','
                | ';'
                | ':'
                | '='
                | '!'
                | '&'
                | '|'
                | '?'
                | '+'
                | '-'
                | '*'
                | '%'
                | '^'
                | '~'
                | '<'
                | '>'
        ),
        TokenKind::String | TokenKind::Preprocessor => false,
    }
}

fn tokenize(
    source: &str,
    profile: &'static CLikeProfile,
) -> Vec<Token> {
    let mut lexer = Lexer::new(source, profile);
    let mut tokens = Vec::new();
    let mut had_space = false;
    let mut had_newline = false;

    while let Some(character) = lexer.peek() {
        if character.is_whitespace() {
            let (space, newline) = lexer.skip_whitespace();
            had_space |= space;
            had_newline |= newline;
            continue;
        }

        if let Some(preprocessor) = lexer.try_consume_preprocessor()
        {
            let token = Token {
                kind: TokenKind::Preprocessor,
                text: preprocessor,
                preceded_by_space: had_space,
                preceded_by_newline: true,
            };
            lexer.last_token = Some(token.clone());
            tokens.push(token);
            had_space = false;
            had_newline = false;
            continue;
        }

        if let Some(comment_newline) = lexer.try_consume_comment() {
            had_space = true;
            had_newline |= comment_newline;
            continue;
        }

        if let Some(string) = lexer.try_consume_string() {
            let token = Token {
                kind: TokenKind::String,
                text: string,
                preceded_by_space: had_space,
                preceded_by_newline: had_newline,
            };
            lexer.last_token = Some(token.clone());
            tokens.push(token);
            lexer.at_line_start = false;
            had_space = false;
            had_newline = false;
            continue;
        }

        if let Some(regex) = lexer.try_consume_regex() {
            let token = Token {
                kind: TokenKind::String,
                text: regex,
                preceded_by_space: had_space,
                preceded_by_newline: had_newline,
            };
            lexer.last_token = Some(token.clone());
            tokens.push(token);
            lexer.at_line_start = false;
            had_space = false;
            had_newline = false;
            continue;
        }

        if let Some(word) = lexer.try_consume_word() {
            let token = Token {
                kind: TokenKind::Word,
                text: word,
                preceded_by_space: had_space,
                preceded_by_newline: had_newline,
            };
            lexer.last_token = Some(token.clone());
            tokens.push(token);
            lexer.at_line_start = false;
            had_space = false;
            had_newline = false;
            continue;
        }

        let punctuation = lexer.advance().unwrap();
        let token = Token {
            kind: TokenKind::Punct(punctuation),
            text: punctuation.to_string(),
            preceded_by_space: had_space,
            preceded_by_newline: had_newline,
        };
        lexer.last_token = Some(token.clone());
        tokens.push(token);
        lexer.at_line_start = false;
        had_space = false;
        had_newline = false;
    }

    tokens
}

fn needs_space(
    previous: &Token,
    next: &Token,
    profile: &CLikeProfile,
) -> bool {
    if previous.kind == TokenKind::Preprocessor
        || next.kind == TokenKind::Preprocessor
    {
        return false;
    }
    if !next.preceded_by_space {
        return false;
    }
    match (&previous.kind, &next.kind) {
        (TokenKind::Word, TokenKind::Word) => true,
        (TokenKind::Word, TokenKind::String) => true,
        (TokenKind::String, TokenKind::Word) => true,
        (TokenKind::String, TokenKind::String) => true,
        (TokenKind::Punct(left), TokenKind::Punct(right)) => {
            dangerous_punctuation_pair(*left, *right, profile)
        }
        _ => false,
    }
}

fn dangerous_punctuation_pair(
    left: char,
    right: char,
    profile: &CLikeProfile,
) -> bool {
    let pair = [left, right];
    matches!(
        pair,
        ['+', '+']
            | ['-', '-']
            | ['+', '=']
            | ['-', '=']
            | ['*', '=']
            | ['/', '=']
            | ['%', '=']
            | ['&', '=']
            | ['|', '=']
            | ['^', '=']
            | ['!', '=']
            | ['<', '=']
            | ['>', '=']
            | ['=', '=']
            | ['=', '>']
            | ['-', '>']
            | ['<', '-']
            | [':', ':']
            | ['.', '.']
            | ['?', '.']
            | ['?', '?']
            | ['&', '&']
            | ['|', '|']
            | ['|', '>']
            | ['^', '^']
            | ['<', '<']
            | ['>', '>']
            | ['*', '*']
            | ['/', '/']
            | ['/', '*']
            | ['*', '/']
            | ['#', '#']
            | ['@', '@']
            | ['$', '{']
    ) || (profile.long_bracket_strings
        && (matches!(pair, ['[', '['] | [']', ']'])))
}

fn is_word(token: &Token) -> Option<&str> {
    if token.kind == TokenKind::Word {
        Some(token.text.as_str())
    } else {
        None
    }
}

const PREVIOUS_EXPECTS_MORE: &[&str] = &[
    "new",
    "typeof",
    "void",
    "delete",
    "await",
    "yield",
    "instanceof",
    "in",
    "of",
    "as",
    "is",
    "extends",
    "implements",
    "case",
    "else",
    "if",
    "for",
    "while",
    "switch",
    "catch",
    "try",
    "do",
    "function",
    "class",
    "interface",
    "enum",
    "package",
    "import",
    "export",
    "from",
    "const",
    "let",
    "var",
    "val",
    "fun",
    "def",
    "public",
    "private",
    "protected",
    "static",
    "final",
    "override",
    "open",
    "data",
    "sealed",
    "abstract",
    "synchronized",
    "native",
    "transient",
    "volatile",
    "throws",
    "throw",
    "this",
    "super",
];

const NEXT_CONTINUES: &[&str] = &[
    "else",
    "catch",
    "finally",
    "while",
    "in",
    "instanceof",
    "of",
    "as",
    "is",
    "extends",
    "implements",
    "with",
    "where",
    "when",
    "and",
    "or",
];

fn previous_can_end_statement(token: &Token) -> bool {
    match &token.kind {
        TokenKind::Word => {
            !PREVIOUS_EXPECTS_MORE.contains(&token.text.as_str())
        }
        TokenKind::String => true,
        TokenKind::Punct(character) => {
            matches!(character, ')' | ']' | '}')
        }
        TokenKind::Preprocessor => false,
    }
}

fn next_can_continue_statement(
    next: &Token,
    next_next: Option<&Token>,
) -> bool {
    match &next.kind {
        TokenKind::Word => {
            NEXT_CONTINUES.contains(&next.text.as_str())
        }
        TokenKind::String => false,
        TokenKind::Punct(character) => match character {
            '.' | '?' | ':' | ',' | ')' | ']' | '}' | ';' | '='
            | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^'
            | '<' | '>' | '(' | '[' | '{' => {
                if (*character == '+' || *character == '-')
                    && next_next.is_some_and(|candidate| {
                    candidate.kind
                        == TokenKind::Punct(*character)
                        && !candidate.preceded_by_space
                })
                {
                    // `a` newline `++b` is a new statement.
                    false
                } else {
                    true
                }
            }
            _ => false,
        },
        TokenKind::Preprocessor => true,
    }
}

fn should_insert_semicolon(
    previous: &Token,
    next: &Token,
    next_next: Option<&Token>,
) -> bool {
    if previous.kind == TokenKind::Preprocessor
        || next.kind == TokenKind::Preprocessor
    {
        return false;
    }
    if !previous_can_end_statement(previous) {
        return false;
    }
    !next_can_continue_statement(next, next_next)
}

fn reconstruct(
    tokens: &[Token],
    profile: &CLikeProfile,
    options: &MinifierOptions,
) -> String {
    let mut output = String::new();
    let mut previous: Option<&Token> = None;
    let mut pending_newline = false;
    let mut index = 0usize;

    while index < tokens.len() {
        let token = &tokens[index];
        if token.preceded_by_newline {
            pending_newline = true;
        }

        if token.kind == TokenKind::Preprocessor {
            while output.ends_with(' ') {
                output.pop();
            }
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
            output.push_str(&token.text);
            output.push('\n');
            previous = Some(token);
            pending_newline = false;
            index += 1;
            continue;
        }

        let mut inserted_separator = false;
        if options.preserve_line_breaks && pending_newline {
            while output.ends_with(' ') {
                output.pop();
            }
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
        } else if pending_newline
            && profile.statement_separator
            == StatementSeparator::Semicolon
        {
            if let Some(previous_token) = previous {
                let next_next = tokens.get(index + 1);
                if should_insert_semicolon(
                    previous_token,
                    token,
                    next_next,
                ) && !output.ends_with(';')
                {
                    output.push(';');
                    inserted_separator = true;
                }
            }
        }

        if let Some(previous_token) = previous {
            let newline_was_emitted =
                options.preserve_line_breaks && pending_newline;
            if !newline_was_emitted
                && !inserted_separator
                && needs_space(previous_token, token, profile)
                && !output.ends_with(' ')
                && !output.ends_with('\n')
            {
                output.push(' ');
            }
        }

        output.push_str(&token.text);
        previous = Some(token);
        pending_newline = false;
        index += 1;
    }

    while output.ends_with(' ') {
        output.pop();
    }
    output
}

/// Minify C-like source with the supplied profile.
pub fn minify_c_like(
    source: &str,
    profile: &'static CLikeProfile,
    options: &MinifierOptions,
) -> String {
    let source = source.trim_start_matches('\u{feff}');
    if source.trim().is_empty() {
        return String::new();
    }
    let tokens = tokenize(source, profile);
    reconstruct(&tokens, profile, options)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
