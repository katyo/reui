use std::{
    ops::RangeInclusive,
    convert::TryFrom,
    path::PathBuf,
};
use proc_macro2::{TokenStream, TokenTree};
use proc_macro_error::{abort};
use super::{skip_punct, get_char, get_char_opt, get_path};

pub struct FontParams {
    pub name: String,
    pub path: PathBuf,
    pub chrs: Vec<RangeInclusive<char>>,
}

impl FontParams {
    pub fn parse(input: TokenStream) -> Self {
        let mut tokens = input.clone().into_iter();

        let name = match tokens.next() {
            Some(TokenTree::Ident(id)) => id.to_string(),
            token => abort!(token, "A font name was expected."),
        };

        skip_punct(&mut tokens, ',', true);

        let path = get_path(&mut tokens);

        let mut chrs = Vec::new();

        skip_punct(&mut tokens, ',', false);

        loop {
            let init_chr = if let Some(chr) = get_char_opt(&mut tokens) {
                chr
            } else {
                break;
            };
            match tokens.next() {
                None => break,
                Some(TokenTree::Punct(token)) if token.as_char() == ',' => {
                    chrs.push(init_chr ..= init_chr);
                    continue;
                },
                Some(TokenTree::Punct(token)) if token.as_char() == '.' => (),
                token => abort!(token, "A ',' or '..' was expected."),
            }
            skip_punct(&mut tokens, '.', true);
            let inclusive = skip_punct(&mut tokens, '=', false);
            let mut last_chr = get_char(&mut tokens);
            if !inclusive {
                last_chr = char::try_from(last_chr as u32 - 1).unwrap();
            }
            chrs.push(init_chr ..= last_chr);
        };

        Self {
            name,
            path,
            chrs,
        }
    }
}
