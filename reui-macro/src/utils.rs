use std::{
    ops::RangeInclusive,
    env,
    path::PathBuf,
};
use proc_macro2::{TokenStream, Ident, Span};
use proc_macro_error::abort;
use proc_macro_crate::crate_name;
use quote::quote;

pub fn uint_type_for_value(max: u32) -> TokenStream {
    if max <= u8::max_value() as u32 {
        quote!(u8)
    } else if max <= u16::max_value() as u32 {
        quote!(u16)
    } else {
        quote!(u32)
    }
}

pub fn int_type_for_range(range: RangeInclusive<i32>) -> TokenStream {
    let min = *range.start();
    let max = *range.end();
    if min >= i8::min_value() as i32 && max <= i8::max_value() as i32 {
        quote!(i8)
    } else if min >= i16::min_value() as i32 && max <= i16::max_value() as i32 {
        quote!(i16)
    } else {
        quote!(i32)
    }
}

pub fn get_crate_name(name: impl AsRef<str>, span: &Span) -> Ident {
    let name = name.as_ref();
    if let Ok(name) = crate_name(name) {
        Ident::new(&name, Span::call_site())
    } else {
        abort!(span, "Unable to find '{}' crate.", name);
    }
}

pub fn get_source_path(path: impl AsRef<str>, span: &Span) -> PathBuf {
    if let Ok(root) = env::var("CARGO_MANIFEST_DIR") {
        let path = path.as_ref();
        PathBuf::from(&root).join(&path)
    } else {
        abort!(span, "CARGO_MANIFEST_DIR is set by cargo.");
    }
}
