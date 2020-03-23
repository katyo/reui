pub(crate) mod utils;
pub(crate) mod parse;
pub(crate) mod font;
pub(crate) mod image;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;

use font::{FontMacro, FontAttr, FontItem};
use image::{ImageMacro, ImageAttr, ImageItem};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn embed_font(attr_input: TokenStream, item_input: TokenStream) -> TokenStream {
    FontMacro::new(
        parse_macro_input!(attr_input as FontAttr),
        parse_macro_input!(item_input as FontItem),
    ).embed().into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn embed_image(attr_input: TokenStream, item_input: TokenStream) -> TokenStream {
    ImageMacro::new(
        parse_macro_input!(attr_input as ImageAttr),
        parse_macro_input!(item_input as ImageItem),
    ).embed().into()
}
