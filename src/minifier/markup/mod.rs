//! HTML, XML, SVG and single file component minifier.
//!
//! The engine removes comments, collapses whitespace in text nodes while
//! respecting inline elements, normalizes tags and attributes, preserves
//! `<pre>`/`<textarea>` contents, and minifies embedded `<script>`/`<style>`
//! blocks with the matching engine. Template regions such as `{{ }}`, `{% %}`,
//! `<% %>`, `@{ }` and `<? ?>` are treated as opaque so their contents are not
//! rewritten.

use crate::minifier::c_like::{minify_c_like, profile};
use crate::minifier::css::{minify_css, CssDialect};
use crate::minifier::json::{minify_json, JSON};
use crate::minifier::options::MinifierOptions;
use crate::minifier::Minifier;
use anyhow::Result;

/// Markup minifier bound to a language id.
pub struct MarkupMinifier {
    pub language: &'static str,
}

impl Minifier for MarkupMinifier {
    fn minify(
        &self,
        source: &str,
        options: &MinifierOptions,
    ) -> Result<String> {
        Ok(minify_markup(source, self.language, options))
    }
}

#[derive(Debug, Clone)]
enum Token {
    Text(String),
    Tag {
        text: String,
        name: String,
        closing: bool,
        self_closing: bool,
    },
    Opaque(String),
}

enum TemplateAction {
    Comment,
    Template(String),
}

struct ParsedTag {
    text: String,
    name: String,
    closing: bool,
    self_closing: bool,
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

fn collapse_whitespace(input: &str) -> String {
    let mut output = String::new();
    let mut pending_space = false;
    for character in input.chars() {
        if character.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !output.is_empty() {
            output.push(' ');
        }
        pending_space = false;
        output.push(character);
    }
    output
}

fn is_inline_element(name: &str) -> bool {
    matches!(
        name,
        "a" | "abbr"
            | "b"
            | "bdi"
            | "bdo"
            | "br"
            | "button"
            | "cite"
            | "code"
            | "data"
            | "del"
            | "dfn"
            | "em"
            | "i"
            | "img"
            | "input"
            | "ins"
            | "kbd"
            | "label"
            | "mark"
            | "meter"
            | "output"
            | "picture"
            | "progress"
            | "q"
            | "ruby"
            | "s"
            | "samp"
            | "select"
            | "small"
            | "span"
            | "strong"
            | "sub"
            | "sup"
            | "time"
            | "u"
            | "var"
            | "wbr"
            | "audio"
            | "video"
            | "canvas"
            | "embed"
            | "iframe"
            | "object"
            | "svg"
            | "math"
    )
}

fn is_raw_element(name: &str) -> bool {
    matches!(
        name,
        "script"
            | "style"
            | "pre"
            | "textarea"
            | "xmp"
            | "plaintext"
            | "title"
    )
}

fn looks_like_tag_start(chars: &[char], position: usize) -> bool {
    if chars.get(position) != Some(&'<') {
        return false;
    }
    match chars.get(position + 1) {
        Some('/') => chars
            .get(position + 2)
            .is_some_and(|character| character.is_alphabetic()),
        Some('!') | Some('?') => true,
        Some(character) => character.is_alphabetic(),
        None => false,
    }
}

fn normalize_tag(raw: &str) -> String {
    let chars: Vec<char> = raw.chars().collect();
    let mut output = String::new();
    let mut quote: Option<char> = None;
    let mut pending_space = false;
    let mut position = 0usize;

    while position < chars.len() {
        let character = chars[position];

        if let Some(active_quote) = quote {
            output.push(character);
            if character == '\\' {
                if let Some(next) = chars.get(position + 1) {
                    output.push(*next);
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

        if character == '"' || character == '\'' {
            if pending_space && needs_space_before(&output) {
                output.push(' ');
            }
            pending_space = false;
            quote = Some(character);
            output.push(character);
            position += 1;
            continue;
        }

        if character.is_whitespace() {
            pending_space = true;
            position += 1;
            continue;
        }

        if character == '=' {
            while output.ends_with(' ') {
                output.pop();
            }
            output.push('=');
            pending_space = false;
            position += 1;
            while position < chars.len() && chars[position].is_whitespace() {
                position += 1;
            }
            continue;
        }

        if character == '/' && chars.get(position + 1) == Some(&'>') {
            while output.ends_with(' ') {
                output.pop();
            }
            output.push('/');
            output.push('>');
            pending_space = false;
            position += 2;
            continue;
        }

        if character == '>' {
            while output.ends_with(' ') {
                output.pop();
            }
            output.push('>');
            pending_space = false;
            position += 1;
            continue;
        }

        if pending_space && needs_space_before(&output) {
            output.push(' ');
        }
        pending_space = false;
        output.push(character);
        position += 1;
    }

    output
}

fn needs_space_before(output: &str) -> bool {
    !output.is_empty()
        && !output.ends_with('<')
        && !output.ends_with('/')
        && !output.ends_with('!')
}

fn parse_tag(
    chars: &[char],
    position: usize,
) -> Option<(ParsedTag, usize)> {
    if chars.get(position) != Some(&'<') {
        return None;
    }

    let mut cursor = position + 1;
    let closing = chars.get(cursor) == Some(&'/');
    if closing {
        cursor += 1;
    }

    let mut quote: Option<char> = None;
    let mut end = None;
    while cursor < chars.len() {
        let character = chars[cursor];
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            }
        } else if character == '"' || character == '\'' {
            quote = Some(character);
        } else if character == '>' {
            end = Some(cursor);
            break;
        }
        cursor += 1;
    }
    let end = end?;

    let raw: String = chars[position..=end].iter().collect();

    let mut name = String::new();
    let mut name_position = position + 1;
    if closing {
        name_position += 1;
    }
    while name_position < chars.len() {
        let character = chars[name_position];
        if character.is_alphanumeric()
            || character == '-'
            || character == '_'
            || character == ':'
        {
            name.push(character.to_ascii_lowercase());
            name_position += 1;
        } else {
            break;
        }
    }

    let self_closing = raw.trim_end().ends_with("/>");
    Some((
        ParsedTag {
            text: normalize_tag(&raw),
            name,
            closing,
            self_closing,
        },
        end + 1,
    ))
}

fn find_element_close(
    chars: &[char],
    start: usize,
    name: &str,
) -> Option<(usize, usize)> {
    let mut position = start;
    while position + 1 < chars.len() {
        if chars[position] == '<' && chars[position + 1] == '/' {
            let mut cursor = position + 2;
            let mut matched = true;
            for expected in name.chars() {
                match chars.get(cursor) {
                    Some(actual)
                        if actual.to_ascii_lowercase() == expected => {}
                    _ => {
                        matched = false;
                        break;
                    }
                }
                cursor += 1;
            }
            if matched
                && chars
                    .get(cursor)
                    .is_some_and(|character| {
                        character.is_whitespace() || *character == '>'
                    })
            {
                let mut end = cursor;
                while end < chars.len() && chars[end] != '>' {
                    end += 1;
                }
                if end < chars.len() {
                    return Some((position, end + 1));
                }
            }
        }
        position += 1;
    }
    None
}

fn match_template_or_comment(
    chars: &[char],
    position: usize,
) -> Option<(TemplateAction, usize)> {
    if starts_with_at(chars, position, "<!--") {
        // Keep IE conditional comments; they contain markup meant for a
        // specific browser rather than human commentary.
        let is_conditional = starts_with_at(chars, position, "<!--[if")
            || starts_with_at(chars, position, "<![endif");
        if let Some(end) = find_from(chars, position + 4, "-->") {
            let new_position = end + 3;
            if is_conditional {
                let text: String =
                    chars[position..new_position].iter().collect();
                return Some((TemplateAction::Template(text), new_position));
            }
            return Some((TemplateAction::Comment, new_position));
        }
        return None;
    }

    let comment_markers: &[(&str, &str)] = &[
        ("{{!--", "--}}"),
        ("{{!", "}}"),
        ("{#", "#}"),
        ("<%--", "--%>"),
        ("<%#", "%>"),
        ("@*", "*@"),
    ];
    for (open, close) in comment_markers {
        if starts_with_at(chars, position, open) {
            if let Some(end) = find_from(chars, position + open.len(), close) {
                return Some((
                    TemplateAction::Comment,
                    end + close.len(),
                ));
            }
        }
    }

    let template_markers: &[(&str, &str)] = &[
        ("<![CDATA[", "]]>"),
        ("{{", "}}"),
        ("{%", "%}"),
        ("<%", "%>"),
        ("@{", "}"),
        ("@(", ")"),
        ("<?", "?>"),
    ];
    for (open, close) in template_markers {
        if starts_with_at(chars, position, open) {
            if let Some(end) = find_from(chars, position + open.len(), close) {
                let new_position = end + close.len();
                let text: String =
                    chars[position..new_position].iter().collect();
                return Some((TemplateAction::Template(text), new_position));
            }
        }
    }

    None
}

fn strip_comments_only(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut output = String::new();
    let mut position = 0usize;
    while position < chars.len() {
        if let Some((action, next_position)) =
            match_template_or_comment(&chars, position)
        {
            if let TemplateAction::Template(text) = action {
                output.push_str(&text);
            }
            position = next_position;
            continue;
        }
        output.push(chars[position]);
        position += 1;
    }
    output
}

fn process_raw_content(
    name: &str,
    opening_tag: &str,
    content: &str,
    language: &str,
    options: &MinifierOptions,
) -> String {
    match name {
        "script" => {
            let lower = opening_tag.to_ascii_lowercase();
            if lower.contains("application/ld+json")
                || lower.contains("application/json")
                || lower.contains("importmap")
                || lower.contains("speculationrules")
            {
                minify_json(content, &JSON, options)
            } else if lower.contains("text/html")
                || lower.contains("text/template")
            {
                minify_markup(content, language, options)
            } else if lower.contains("lang=\"ts\"")
                || lower.contains("lang='ts'")
                || lower.contains("text/typescript")
            {
                minify_c_like(content, &profile::TYPESCRIPT, options)
            } else {
                minify_c_like(content, &profile::JAVASCRIPT, options)
            }
        }
        "style" => {
            let lower = opening_tag.to_ascii_lowercase();
            if lower.contains("lang=\"scss\"")
                || lower.contains("lang='scss'")
                || lower.contains("text/scss")
            {
                minify_css(content, CssDialect::Scss, options)
            } else if lower.contains("lang=\"less\"")
                || lower.contains("lang='less'")
                || lower.contains("text/less")
            {
                minify_css(content, CssDialect::Less, options)
            } else {
                minify_css(content, CssDialect::Css, options)
            }
        }
        "title" => collapse_whitespace(&strip_comments_only(content)),
        "pre" | "textarea" | "xmp" | "plaintext" => {
            strip_comments_only(content)
        }
        _ => content.to_string(),
    }
}

fn find_frontmatter_end(chars: &[char]) -> Option<usize> {
    if !starts_with_at(chars, 0, "---") {
        return None;
    }
    let mut position = 3usize;
    while position < chars.len() && chars[position] != '\n' {
        position += 1;
    }
    if position >= chars.len() {
        return None;
    }
    position += 1;
    while position < chars.len() {
        if starts_with_at(chars, position, "---") {
            let mut line_end = position;
            while line_end < chars.len() && chars[line_end] != '\n' {
                line_end += 1;
            }
            return Some(if line_end < chars.len() {
                line_end + 1
            } else {
                line_end
            });
        }
        while position < chars.len() && chars[position] != '\n' {
            position += 1;
        }
        if position < chars.len() {
            position += 1;
        }
    }
    None
}

fn is_html_family(language: &str) -> bool {
    matches!(
        language,
        "html"
            | "svg"
            | "vue"
            | "svelte"
            | "astro"
            | "razor"
            | "jsp"
            | "twig"
            | "handlebars"
            | "ejs"
    )
}

fn token_is_inline(token: &Token, inline_aware: bool) -> bool {
    if !inline_aware {
        return false;
    }
    match token {
        Token::Tag { name, .. } => is_inline_element(name),
        Token::Opaque(_) | Token::Text(_) => true,
    }
}

fn normalize_text(
    text: &str,
    previous_inline: bool,
    next_inline: bool,
) -> String {
    let collapsed = collapse_whitespace(text);
    if collapsed.is_empty() {
        if previous_inline || next_inline {
            return " ".to_string();
        }
        return String::new();
    }

    let mut output = String::new();
    if text
        .chars()
        .next()
        .is_some_and(|character| character.is_whitespace())
        && previous_inline
    {
        output.push(' ');
    }
    output.push_str(&collapsed);
    if text
        .chars()
        .last()
        .is_some_and(|character| character.is_whitespace())
        && next_inline
    {
        output.push(' ');
    }
    output
}

fn reconstruct(tokens: &[Token], inline_aware: bool) -> String {
    let mut output = String::new();
    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::Tag { text, .. } => output.push_str(text),
            Token::Opaque(text) => output.push_str(text),
            Token::Text(text) => {
                let previous_inline = index
                    .checked_sub(1)
                    .is_some_and(|previous| {
                        token_is_inline(&tokens[previous], inline_aware)
                    });
                let next_inline = tokens
                    .get(index + 1)
                    .is_some_and(|next| {
                        token_is_inline(next, inline_aware)
                    });
                let piece =
                    normalize_text(text, previous_inline, next_inline);
                if piece.is_empty() {
                    continue;
                }
                if piece == " " {
                    if !output.is_empty() && !output.ends_with(' ') {
                        output.push(' ');
                    }
                } else if output.is_empty() || output.ends_with(' ') {
                    output.push_str(piece.trim_start());
                } else {
                    output.push_str(&piece);
                }
            }
        }
    }
    output.trim().to_string()
}

/// Minify a markup document.
pub fn minify_markup(
    source: &str,
    language: &str,
    options: &MinifierOptions,
) -> String {
    let source = source.trim_start_matches('\u{feff}');
    if source.trim().is_empty() {
        return String::new();
    }

    let chars: Vec<char> = source.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut text = String::new();
    let mut position = 0usize;

    if language == "astro" {
        if let Some(end) = find_frontmatter_end(&chars) {
            let frontmatter: String = chars[..end].iter().collect();
            tokens.push(Token::Opaque(frontmatter));
            position = end;
        }
    }

    while position < chars.len() {
        if let Some((action, next_position)) =
            match_template_or_comment(&chars, position)
        {
            if !text.is_empty() {
                tokens.push(Token::Text(std::mem::take(&mut text)));
            }
            if let TemplateAction::Template(region) = action {
                tokens.push(Token::Opaque(region));
            }
            position = next_position;
            continue;
        }

        if chars[position] == '<'
            && looks_like_tag_start(&chars, position)
        {
            if let Some((tag, next_position)) =
                parse_tag(&chars, position)
            {
                if !text.is_empty() {
                    tokens.push(Token::Text(std::mem::take(&mut text)));
                }

                let is_raw = !tag.closing
                    && !tag.self_closing
                    && is_raw_element(&tag.name);

                if is_raw {
                    if let Some((content_end, close_end)) =
                        find_element_close(&chars, next_position, &tag.name)
                    {
                        let content: String =
                            chars[next_position..content_end]
                                .iter()
                                .collect();
                        let processed = process_raw_content(
                            &tag.name,
                            &tag.text,
                            &content,
                            language,
                            options,
                        );
                        let name = tag.name.clone();
                        tokens.push(Token::Tag {
                            text: tag.text,
                            name: tag.name,
                            closing: tag.closing,
                            self_closing: tag.self_closing,
                        });
                        if !processed.is_empty() {
                            tokens.push(Token::Opaque(processed));
                        }
                        let close_raw: String =
                            chars[content_end..close_end].iter().collect();
                        tokens.push(Token::Tag {
                            text: normalize_tag(&close_raw),
                            name,
                            closing: true,
                            self_closing: false,
                        });
                        position = close_end;
                        continue;
                    }

                    // No closing tag: treat the remainder as raw content.
                    let content: String =
                        chars[next_position..].iter().collect();
                    let processed = process_raw_content(
                        &tag.name,
                        &tag.text,
                        &content,
                        language,
                        options,
                    );
                    tokens.push(Token::Tag {
                        text: tag.text,
                        name: tag.name,
                        closing: tag.closing,
                        self_closing: tag.self_closing,
                    });
                    if !processed.is_empty() {
                        tokens.push(Token::Opaque(processed));
                    }
                    position = chars.len();
                    continue;
                }

                tokens.push(Token::Tag {
                    text: tag.text,
                    name: tag.name,
                    closing: tag.closing,
                    self_closing: tag.self_closing,
                });
                position = next_position;
                continue;
            }
        }

        text.push(chars[position]);
        position += 1;
    }

    if !text.is_empty() {
        tokens.push(Token::Text(text));
    }

    reconstruct(&tokens, is_html_family(language))
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
