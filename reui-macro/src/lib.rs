pub(crate) mod utils;
pub(crate) mod parse;
pub(crate) mod font;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;

use font::{FontMacro, FontAttr, FontItem};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn embed_font(attr_input: TokenStream, item_input: TokenStream) -> TokenStream {
    FontMacro::new(
        parse_macro_input!(attr_input as FontAttr),
        parse_macro_input!(item_input as FontItem),
    ).embed().into()
}
