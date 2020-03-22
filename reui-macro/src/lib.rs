mod font;

use std::{
    env,
    path::Path,
    ops::RangeInclusive,
};
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Ident, Span, Literal};
use proc_macro_error::{proc_macro_error, abort};
use proc_macro_crate::crate_name;
use syn::parse_macro_input;
use quote::quote;

use font::{FontParams, FontData};

#[proc_macro]
#[proc_macro_error]
pub fn embed_font(input: TokenStream) -> TokenStream {
    let params = parse_macro_input!(input as FontParams);

    let core_crate = if let Ok(name) = crate_name("reui") {
        Ident::new(&name, Span::call_site())
    } else {
        abort!(params.name, "Unable to find 'reui' crate.");
    };

    let name = params.name;

    let root = env::var("CARGO_MANIFEST_DIR").unwrap();

    let path = params.path.value();
    let path = Path::new(&root).join(&path);

    let chrs = params.chrs.iter().map(|chr| chr.range()).collect::<Vec<_>>();

    let font_data = match FontData::load(&path, &chrs) {
        Ok(font_data) => font_data,
        Err(error) => abort!(params.path, "Error when loading font: {}", error),
    };

    let codes_list = font_data.codes.iter().map(|range| {
        let start = Literal::u32_unsuffixed(*range.start());
        let end = Literal::u32_unsuffixed(*range.end());
        quote! {
            #start ..= #end
        }
    });

    let dim_type = int_type_for_range(font_data.dim_range().unwrap());

    let code_type = uint_type_for_value(font_data.max_code().unwrap());

    let pixels_list = font_data.pixels.iter().map(|byte| {
        let byte = Literal::u8_unsuffixed(*byte);
        quote! { #byte }
    });

    let rect_kinds = font_data.rect_kinds();

    if rect_kinds.len() == 1 { // Font V2
        let glyph_rect = {
            let font::Rect { x, y, w, h } = &rect_kinds[0];

            let x = Literal::i32_unsuffixed(*x);
            let y = Literal::i32_unsuffixed(*y);
            let w = Literal::i32_unsuffixed(*w);
            let h = Literal::i32_unsuffixed(*h);

            quote! {
                #core_crate::Rect::new(#core_crate::Point::new(#x, #y), #core_crate::Size::new(#w, #h))
            }
        };

        quote! {
            pub static #name: #core_crate::FontV2<#core_crate::format::GS1, #dim_type, #code_type> = #core_crate::FontV2::new(
                #core_crate::format::GS1,
                &[#(#codes_list),*],
                #glyph_rect,
                &[#(#pixels_list),*],
            );
        }
    } else { // Font V1
        let glyphs_list = font_data.glyphs.iter().map(|font::GlyphData { rect: font::Rect { x, y, w, h }, off }| {
            let x = Literal::i32_unsuffixed(*x);
            let y = Literal::i32_unsuffixed(*y);
            let w = Literal::i32_unsuffixed(*w);
            let h = Literal::i32_unsuffixed(*h);
            let off = Literal::usize_unsuffixed(*off);
            quote! {
                #core_crate::GlyphData::new(#core_crate::Rect::new(#core_crate::Point::new(#x, #y), #core_crate::Size::new(#w, #h)), #off)
            }
        });

        let off_type = uint_type_for_value(font_data.max_off().unwrap() as u32);

        quote! {
            pub static #name: #core_crate::FontV1<#core_crate::format::GS1, #dim_type, #off_type, #code_type> = #core_crate::FontV1::new(
                #core_crate::format::GS1,
                &[#(#codes_list),*],
                &[#(#glyphs_list),*],
                &[#(#pixels_list),*],
            );
        }
    }.into()
}

fn uint_type_for_value(max: u32) -> TokenStream2 {
    if max <= u8::max_value() as u32 {
        quote!(u8)
    } else if max <= u16::max_value() as u32 {
        quote!(u16)
    } else {
        quote!(u32)
    }
}

fn int_type_for_range(range: RangeInclusive<i32>) -> TokenStream2 {
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
