use syn::{
    Result,
    token::Comma,
    LitStr,
    punctuated::Punctuated,
    parse::{Parse, ParseStream},
};

use super::FontChars;

pub struct FontAttr {
    pub path: LitStr,
    pub chars: Punctuated<FontChars, Comma>,
}

impl Parse for FontAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let path = input.parse::<LitStr>()?;

        input.parse::<Option<Comma>>()?;

        let chars = Punctuated::parse_terminated(input)?;

        Ok(Self {
            path,
            chars,
        })
    }
}

#[cfg(test)]
mod test {
    use syn::parse_str;
    use super::*;

    #[test]
    fn no_chars() {
        let params: FontAttr = parse_str(r#""path/to/monospace.bdf""#).unwrap();

        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chars.len(), 0);
    }

    #[test]
    fn single_char() {
        let params: FontAttr = parse_str(r#""path/to/monospace.bdf", 'a'"#).unwrap();

        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chars.len(), 1);
        assert_eq!(params.chars[0].range(), 'a'..='a');
    }

    #[test]
    fn single_char_range_inclusive() {
        let params: FontAttr = parse_str(r#""path/to/monospace.bdf", 'a'..='z'"#).unwrap();

        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chars.len(), 1);
        assert_eq!(params.chars[0].range(), 'a'..='z');
    }

    #[test]
    fn single_char_range_exclusive() {
        let params: FontAttr = parse_str(r#""path/to/monospace.bdf", 'a'..'z'"#).unwrap();

        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chars.len(), 1);
        assert_eq!(params.chars[0].range(), 'a'..='y');
    }

    #[test]
    fn two_char_ranges() {
        let params: FontAttr = parse_str(r#""path/to/monospace.bdf", 'a'..'z', 'A' ..= 'Z'"#).unwrap();

        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chars.len(), 2);
        assert_eq!(params.chars[0].range(), 'a'..='y');
        assert_eq!(params.chars[1].range(), 'A'..='Z');
    }

    #[test]
    fn two_chars_and_two_char_ranges() {
        let params: FontAttr = parse_str(r#""path/to/monospace.bdf", ',', ';', 'a'..'z', 'A' ..= 'Z'"#).unwrap();

        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chars.len(), 4);
        assert_eq!(params.chars[0].range(), ','..=',');
        assert_eq!(params.chars[1].range(), ';'..=';');
        assert_eq!(params.chars[2].range(), 'a'..='y');
        assert_eq!(params.chars[3].range(), 'A'..='Z');
    }

    #[test]
    fn two_chars_and_two_char_ranges_unordered() {
        let params: FontAttr = parse_str(r#""path/to/monospace.bdf", 'a'..'z', ',', ';', 'A' ..= 'Z'"#).unwrap();

        assert_eq!(params.path.value(), "path/to/monospace.bdf");
        assert_eq!(params.chars.len(), 4);
        assert_eq!(params.chars[0].range(), 'a'..='y');
        assert_eq!(params.chars[1].range(), ','..=',');
        assert_eq!(params.chars[2].range(), ';'..=';');
        assert_eq!(params.chars[3].range(), 'A'..='Z');
    }
}
