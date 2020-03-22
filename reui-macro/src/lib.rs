pub(crate) mod utils;
pub(crate) mod font;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;

use font::FontParams;

#[proc_macro]
#[proc_macro_error]
pub fn embed_font(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as FontParams).embed().into()
}
