use std::path::PathBuf;
use proc_macro2::{TokenTree, Literal, token_stream::IntoIter as TokenIter};
use proc_macro_error::{abort};
use enquote;

pub fn parse_char_literal(lit: &Literal) -> Option<char> {
    let s = lit.to_string();
    let mut c = s.chars();
    if let (Some('\''), Some(chr), Some('\''), None) = (c.next(), c.next(), c.next(), c.next()) {
        Some(chr)
    } else {
        None
    }
}

pub fn parse_string_literal(lit: &Literal) -> Option<String> {
    let s = lit.to_string();
    if s.starts_with('"') && s.ends_with('"') {
        enquote::unquote(&s).ok()
    } else {
        None
    }
}

pub fn get_string(tokens: &mut TokenIter) -> String {
    match tokens.next() {
        Some(TokenTree::Literal(lit)) => {
            if let Some(s) = parse_string_literal(&lit) {
                s
            } else {
                abort!(lit, "A string literal was expected.");
            }
        },
        token => abort!(token, "A string literal was expected."),
    }
}

pub fn get_path(tokens: &mut TokenIter) -> PathBuf {
    match tokens.next() {
        Some(token) => {
            let path = token.span().source_file().path();
            if let TokenTree::Literal(lit) = token {
                if let Some(s) = parse_string_literal(&lit) {
                    PathBuf::from(s)
                } else {
                    abort!(lit, "A string literal was expected.");
                }
            } else {
                abort!(token, "A string literal was expected.");
            }
        },
        token => abort!(token, "A string literal was expected."),
    }

    //PathBuf::from(&get_string(tokens))
}

pub fn get_char(tokens: &mut TokenIter) -> char {
    match tokens.next() {
        Some(TokenTree::Literal(lit)) => {
            if let Some(chr) = parse_char_literal(&lit) {
                chr
            } else {
                abort!(lit, "A character was expected.");
            }
        },
        token => abort!(token, "A character was expected."),
    }
}

pub fn get_char_opt(tokens: &mut TokenIter) -> Option<char> {
    match tokens.next() {
        None => None,
        Some(TokenTree::Literal(lit)) => {
            if let Some(chr) = parse_char_literal(&lit) {
                Some(chr)
            } else {
                abort!(lit, "A character was expected.");
            }
        },
        token => abort!(token, "A character was expected."),
    }
}

pub fn skip_punct(tokens: &mut TokenIter, chr: char, required: bool) -> bool {
    match tokens.next() {
        Some(TokenTree::Punct(token)) if token.as_char() == chr => true,
        token @ None => if required {
            abort!(token, "A {} was expected.", chr);
        } else {
            false
        },
        token => abort!(token, "A {} was expected.", chr),
    }
}
