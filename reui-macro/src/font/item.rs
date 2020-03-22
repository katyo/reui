use syn::{
    Attribute,
    Visibility,
    token::{Static, Mut, Colon, Eq, Semi, Comma, Paren},
    Ident,
    Result,
    parenthesized,
    custom_keyword,
    punctuated::Punctuated,
    parse::{Parse, ParseStream},
};

use super::FontChars;

pub struct FontItem {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub static_token: Static,
    pub mutability: Option<Mut>,
    pub ident: Ident,
    pub colon_token: Colon,
    pub ty: Font,
    pub eq_token: Eq,
    pub paren_token: Paren,
    pub chars: Punctuated<FontChars, Comma>,
    pub semi_token: Semi,
}

impl Parse for FontItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let chars_content;
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            static_token: input.parse()?,
            mutability: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            eq_token: input.parse()?,
            paren_token: parenthesized!(chars_content in input),
            chars: Punctuated::parse_terminated(&chars_content)?,
            semi_token: input.parse()?,
        })
    }
}

custom_keyword!(Font);

#[cfg(test)]
mod test {
    use syn::parse_str;
    use super::*;

    #[test]
    fn no_value() {
        assert!(parse_str::<FontItem>(r#"static MONOSPACE: Font"#).is_err());
    }

    #[test]
    fn no_chars() {
        let params: FontItem = parse_str(r#"static MONOSPACE: Font = ();"#).unwrap();

        assert_eq!(params.ident.to_string(), "MONOSPACE");
        assert_eq!(params.chars.len(), 0);
    }

    #[test]
    fn single_char() {
        let params: FontItem = parse_str(r#"pub static MONOSPACE: Font = ('a');"#).unwrap();

        assert_eq!(params.ident.to_string(), "MONOSPACE");
        assert_eq!(params.chars.len(), 1);
        assert_eq!(params.chars[0].range(), 'a'..='a');
    }

    #[test]
    fn single_char_range_inclusive() {
        let params: FontItem = parse_str(r#"static MONOSPACE: Font = ('a'..='z');"#).unwrap();

        assert_eq!(params.ident.to_string(), "MONOSPACE");
        assert_eq!(params.chars.len(), 1);
        assert_eq!(params.chars[0].range(), 'a'..='z');
    }

    #[test]
    fn single_char_range_exclusive() {
        let params: FontItem = parse_str(r#"static MONOSPACE: Font = ('a'..'z');"#).unwrap();

        assert_eq!(params.ident.to_string(), "MONOSPACE");
        assert_eq!(params.chars.len(), 1);
        assert_eq!(params.chars[0].range(), 'a'..='y');
    }

    #[test]
    fn two_char_ranges() {
        let params: FontItem = parse_str(r#"pub static MONOSPACE: Font = ('a'..'z', 'A' ..= 'Z');"#).unwrap();

        assert_eq!(params.ident.to_string(), "MONOSPACE");
        assert_eq!(params.chars.len(), 2);
        assert_eq!(params.chars[0].range(), 'a'..='y');
        assert_eq!(params.chars[1].range(), 'A'..='Z');
    }

    #[test]
    fn two_chars_and_two_char_ranges() {
        let params: FontItem = parse_str(r#"static MONOSPACE: Font = (',', ';', 'a'..'z', 'A' ..= 'Z');"#).unwrap();

        assert_eq!(params.ident.to_string(), "MONOSPACE");
        assert_eq!(params.chars.len(), 4);
        assert_eq!(params.chars[0].range(), ','..=',');
        assert_eq!(params.chars[1].range(), ';'..=';');
        assert_eq!(params.chars[2].range(), 'a'..='y');
        assert_eq!(params.chars[3].range(), 'A'..='Z');
    }

    #[test]
    fn two_chars_and_two_char_ranges_unordered() {
        let params: FontItem = parse_str(r#"pub static MONOSPACE: Font = ('a'..'z', ',', ';', 'A' ..= 'Z');"#).unwrap();

        assert_eq!(params.ident.to_string(), "MONOSPACE");
        assert_eq!(params.chars.len(), 4);
        assert_eq!(params.chars[0].range(), 'a'..='y');
        assert_eq!(params.chars[1].range(), ','..=',');
        assert_eq!(params.chars[2].range(), ';'..=';');
        assert_eq!(params.chars[3].range(), 'A'..='Z');
    }
}
