use std::{
    ops::RangeInclusive,
    convert::TryFrom,
};
use syn::{
    Ident,
    Result,
    Token,
    LitStr,
    LitChar,
    punctuated::Punctuated,
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

pub struct FontParams {
    pub name: Ident,
    pub path: LitStr,
    pub chrs: Punctuated<FontChars, Token![,]>,
}

impl Parse for FontParams {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;

        input.parse::<Token![,]>()?;

        let path = input.parse::<LitStr>()?;

        input.parse::<Option<Token![,]>>()?;

        let chrs = Punctuated::parse_terminated(input)?;

        Ok(Self {
            name,
            path,
            chrs,
        })
    }
}

#[cfg(test)]
mod test {
    use syn::parse_str;
    use super::*;

    #[test]
    fn ident_only() {
        assert!(parse_str::<FontParams>(r#"monospace"#).is_err());
    }

    #[test]
    fn no_chars() {
        let params: FontParams = parse_str(r#"monospace, "path/to/monospace.bdf""#).unwrap();

        assert_eq!(params.name.to_string(), "monospace");
        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chrs.len(), 0);
    }

    #[test]
    fn single_char() {
        let params: FontParams = parse_str(r#"monospace, "path/to/monospace.bdf", 'a'"#).unwrap();

        assert_eq!(params.name.to_string(), "monospace");
        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chrs.len(), 1);
        assert_eq!(params.chrs[0].range(), 'a'..='a');
    }

    #[test]
    fn single_char_range_inclusive() {
        let params: FontParams = parse_str(r#"monospace, "path/to/monospace.bdf", 'a'..='z'"#).unwrap();

        assert_eq!(params.name.to_string(), "monospace");
        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chrs.len(), 1);
        assert_eq!(params.chrs[0].range(), 'a'..='z');
    }

    #[test]
    fn single_char_range_exclusive() {
        let params: FontParams = parse_str(r#"monospace, "path/to/monospace.bdf", 'a'..'z'"#).unwrap();

        assert_eq!(params.name.to_string(), "monospace");
        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chrs.len(), 1);
        assert_eq!(params.chrs[0].range(), 'a'..='y');
    }

    #[test]
    fn two_char_ranges() {
        let params: FontParams = parse_str(r#"monospace, "path/to/monospace.bdf", 'a'..'z', 'A' ..= 'Z'"#).unwrap();

        assert_eq!(params.name.to_string(), "monospace");
        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chrs.len(), 2);
        assert_eq!(params.chrs[0].range(), 'a'..='y');
        assert_eq!(params.chrs[1].range(), 'A'..='Z');
    }

    #[test]
    fn two_chars_and_two_char_ranges() {
        let params: FontParams = parse_str(r#"monospace, "path/to/monospace.bdf", ',', ';', 'a'..'z', 'A' ..= 'Z'"#).unwrap();

        assert_eq!(params.name.to_string(), "monospace");
        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chrs.len(), 4);
        assert_eq!(params.chrs[0].range(), ','..=',');
        assert_eq!(params.chrs[1].range(), ';'..=';');
        assert_eq!(params.chrs[2].range(), 'a'..='y');
        assert_eq!(params.chrs[3].range(), 'A'..='Z');
    }

    #[test]
    fn two_chars_and_two_char_ranges_unordered() {
        let params: FontParams = parse_str(r#"monospace, "path/to/monospace.bdf", 'a'..'z', ',', ';', 'A' ..= 'Z'"#).unwrap();

        assert_eq!(params.name.to_string(), "monospace");
        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chrs.len(), 4);
        assert_eq!(params.chrs[0].range(), 'a'..='y');
        assert_eq!(params.chrs[1].range(), ','..=',');
        assert_eq!(params.chrs[2].range(), ';'..=';');
        assert_eq!(params.chrs[3].range(), 'A'..='Z');
    }
}
