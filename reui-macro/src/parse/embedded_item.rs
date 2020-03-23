use syn::{
    Result,
    Ident,
    parse::{Parse, ParseStream},
};

use crate::{
    font::{FontTypeKeyword, FontItemValue},
    image::{ImageTypeKeyword, ImageItemValue},
};

use super::{StaticItem, StaticValueParse};

pub type EmbeddedItem = StaticItem<EmbeddedType, EmbeddedValue>;

#[derive(Clone, Copy)]
pub enum EmbeddedType {
    Font(FontTypeKeyword),
    Image(ImageTypeKeyword),
}

impl Parse for EmbeddedType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(FontTypeKeyword) {
            Ok(EmbeddedType::Font(input.parse()?))
        } else if input.peek(ImageTypeKeyword) {
            Ok(EmbeddedType::Image(input.parse()?))
        } else {
            Err(input.error("Either of `Font` or `Image` type was expected."))
        }
    }
}

impl From<EmbeddedType> for FontTypeKeyword {
    fn from(ty: EmbeddedType) -> Self {
        match ty {
            EmbeddedType::Font(ty) => ty,
            _ => unreachable!(),
        }
    }
}

impl From<EmbeddedType> for ImageTypeKeyword {
    fn from(ty: EmbeddedType) -> Self {
        match ty {
            EmbeddedType::Image(ty) => ty,
            _ => unreachable!(),
        }
    }
}

pub enum EmbeddedValue {
    Font(FontItemValue),
    Image(ImageItemValue),
}

impl StaticValueParse<EmbeddedType> for EmbeddedValue {
    fn parse_static_value(input: ParseStream, ident: &Ident, ty: &EmbeddedType) -> Result<Self> {
        Ok(match ty {
            EmbeddedType::Font(ty) => EmbeddedValue::Font(
                FontItemValue::parse_static_value(input, ident, ty)?
            ),
            EmbeddedType::Image(ty) => EmbeddedValue::Image(
                ImageItemValue::parse_static_value(input, ident, ty)?
            ),
        })
    }
}

impl From<EmbeddedValue> for FontItemValue {
    fn from(val: EmbeddedValue) -> Self {
        match val {
            EmbeddedValue::Font(val) => val,
            _ => unreachable!(),
        }
    }
}

impl From<EmbeddedValue> for ImageItemValue {
    fn from(val: EmbeddedValue) -> Self {
        match val {
            EmbeddedValue::Image(val) => val,
            _ => unreachable!(),
        }
    }
}
