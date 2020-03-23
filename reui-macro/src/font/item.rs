use syn::{
    token::{Comma, Paren},
    Result,
    Ident,
    parenthesized,
    punctuated::Punctuated,
    parse::{ParseStream},
};

use crate::parse::{StaticItem, StaticValueParse};
use super::FontChars;

pub type FontItem = StaticItem<FontTypeKeyword, FontItemValue>;

pub struct FontItemValue {
    pub paren_token: Paren,
    pub chars: Punctuated<FontChars, Comma>,
}

impl StaticValueParse<FontTypeKeyword> for FontItemValue {
    fn parse_static_value(input: ParseStream, _ident: &Ident, _ty: &FontTypeKeyword) -> Result<Self> {
        let chars_content;
        Ok(Self {
            paren_token: parenthesized!(chars_content in input),
            chars: Punctuated::parse_terminated(&chars_content)?,
        })
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(Font);
}

pub use self::kw::Font as FontTypeKeyword;

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
        let item: FontItem = parse_str(r#"static MONOSPACE: Font = ();"#).unwrap();

        assert_eq!(item.ident.to_string(), "MONOSPACE");
        assert_eq!(item.value.chars.len(), 0);
    }

    #[test]
    fn single_char() {
        let item: FontItem = parse_str(r#"pub static MONOSPACE: Font = ('a');"#).unwrap();

        assert_eq!(item.ident.to_string(), "MONOSPACE");
        assert_eq!(item.value.chars.len(), 1);
        assert_eq!(item.value.chars[0].range(), 'a'..='a');
    }

    #[test]
    fn single_char_range_inclusive() {
        let item: FontItem = parse_str(r#"static MONOSPACE: Font = ('a'..='z');"#).unwrap();

        assert_eq!(item.ident.to_string(), "MONOSPACE");
        assert_eq!(item.value.chars.len(), 1);
        assert_eq!(item.value.chars[0].range(), 'a'..='z');
    }

    #[test]
    fn single_char_range_exclusive() {
        let item: FontItem = parse_str(r#"static MONOSPACE: Font = ('a'..'z');"#).unwrap();

        assert_eq!(item.ident.to_string(), "MONOSPACE");
        assert_eq!(item.value.chars.len(), 1);
        assert_eq!(item.value.chars[0].range(), 'a'..='y');
    }

    #[test]
    fn two_char_ranges() {
        let item: FontItem = parse_str(r#"pub static MONOSPACE: Font = ('a'..'z', 'A' ..= 'Z');"#).unwrap();

        assert_eq!(item.ident.to_string(), "MONOSPACE");
        assert_eq!(item.value.chars.len(), 2);
        assert_eq!(item.value.chars[0].range(), 'a'..='y');
        assert_eq!(item.value.chars[1].range(), 'A'..='Z');
    }

    #[test]
    fn two_chars_and_two_char_ranges() {
        let item: FontItem = parse_str(r#"static MONOSPACE: Font = (',', ';', 'a'..'z', 'A' ..= 'Z');"#).unwrap();

        assert_eq!(item.ident.to_string(), "MONOSPACE");
        assert_eq!(item.value.chars.len(), 4);
        assert_eq!(item.value.chars[0].range(), ','..=',');
        assert_eq!(item.value.chars[1].range(), ';'..=';');
        assert_eq!(item.value.chars[2].range(), 'a'..='y');
        assert_eq!(item.value.chars[3].range(), 'A'..='Z');
    }

    #[test]
    fn two_chars_and_two_char_ranges_unordered() {
        let item: FontItem = parse_str(r#"pub static MONOSPACE: Font = ('a'..'z', ',', ';', 'A' ..= 'Z');"#).unwrap();

        assert_eq!(item.ident.to_string(), "MONOSPACE");
        assert_eq!(item.value.chars.len(), 4);
        assert_eq!(item.value.chars[0].range(), 'a'..='y');
        assert_eq!(item.value.chars[1].range(), ','..=',');
        assert_eq!(item.value.chars[2].range(), ';'..=';');
        assert_eq!(item.value.chars[3].range(), 'A'..='Z');
    }
}
