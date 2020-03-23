use syn::{
    token::{Paren},
    Result,
    parenthesized,
    parse::{Parse, ParseStream},
};

use crate::parse::StaticItem;

pub type ImageItem = StaticItem<ImageTypeKeyword, ImageItemValue>;

pub struct ImageItemValue {
    pub paren_token: Paren,
}

impl Parse for ImageItemValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let _content;
        Ok(Self {
            paren_token: parenthesized!(_content in input),
        })
    }
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(Image);
}

pub use self::kw::Image as ImageTypeKeyword;

#[cfg(test)]
mod test {
    use syn::parse_str;
    use super::*;

    #[test]
    fn no_value() {
        assert!(parse_str::<ImageItem>(r#"static IMAGE: Image"#).is_err());
    }

    #[test]
    fn empty_value() {
        let item: ImageItem = parse_str(r#"static IMAGE: Image = ();"#).unwrap();

        assert_eq!(item.ident.to_string(), "IMAGE");
    }
}
