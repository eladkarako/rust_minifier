use anyhow::Result;
use proc_macro2::TokenStream;

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
fn remove_comments_and_rustdocs(source:&str)->Result<String>{
    let mut result=String::with_capacity(source.len());
    let mut chars=source.chars().peekable();
    let mut in_string=false;
    let mut string_char=' ';
    let mut escape_next=false;
    let mut raw_string_hashes=0;

    while let Some(ch)=chars.next(){
        if in_string&&escape_next{
            result.push(ch);
            escape_next=false;
            continue;
        }
        if in_string{
            result.push(ch);
            if ch=='\\'{
                escape_next=true;
            }else if ch==string_char{
                if raw_string_hashes>0{
                    let mut hash_count=0;
                    while chars.peek()==Some(&'#')&&hash_count<raw_string_hashes{
                        result.push(chars.next().unwrap());
                        hash_count+=1;
                    }
                    if hash_count==raw_string_hashes{
                        in_string=false;
                        raw_string_hashes=0;
                    }
                }else{
                    in_string=false;
                }
            }
            continue;
        }
        if ch=='r'&&matches!(chars.peek(),Some(&'"')|Some(&'#')){
            result.push(ch);
            // Check what comes next
            if chars.peek()==Some(&'"'){
                // r"..."
                result.push(chars.next().unwrap());
                in_string=true;
                string_char='"';
                raw_string_hashes=0;
            }else if chars.peek()==Some(&'#'){
                // r#"..."# or r##"..."## etc.
                let mut hash_count=0;
                while chars.peek()==Some(&'#'){
                    result.push(chars.next().unwrap());
                    hash_count+=1;
                }
                if chars.peek()==Some(&'"'){
                    result.push(chars.next().unwrap());
                    in_string=true;
                    string_char='"';
                    raw_string_hashes=hash_count;
                }
            }
            continue;
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

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
