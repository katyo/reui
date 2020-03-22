use std::{
    ops::RangeInclusive,
    convert::TryFrom,
};

use syn::{
    Result,
    Token,
    LitChar,
    parse::{Parse, ParseStream},
};

pub enum FontChars {
    Char(LitChar),
    Range(LitChar, LitChar, bool),
}

impl FontChars {
    pub fn range(&self) -> RangeInclusive<char> {
        match self {
            FontChars::Char(chr) => {
                let chr = chr.value();
                chr ..= chr
            },
            FontChars::Range(init_chr, last_chr, inclusive) => {
                let init_chr = init_chr.value();
                let last_chr = last_chr.value();
                let last_chr = if *inclusive {
                    last_chr
                } else {
                    TryFrom::try_from(last_chr as u32 - 1).unwrap()
                };
                init_chr ..= last_chr
            },
        }
    }
}

impl Parse for FontChars {
    fn parse(input: ParseStream) -> Result<Self> {
        let chr = input.parse::<LitChar>()?;

        let lookahead = input.lookahead1();

        Ok(if lookahead.peek(Token![..=]) {
            input.parse::<Token![..=]>()?;

            let end_chr = input.parse::<LitChar>()?;

            FontChars::Range(chr, end_chr, true)
        } else if lookahead.peek(Token![..]) {
            input.parse::<Token![..]>()?;

            let end_chr = input.parse::<LitChar>()?;

            FontChars::Range(chr, end_chr, false)
        } else {
            FontChars::Char(chr)
        })
    }
}
