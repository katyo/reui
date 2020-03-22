use proc_macro2::{TokenStream, Literal};
use proc_macro_error::abort;
use quote::quote;

use super::{FontParams, FontData, GlyphData, Rect};
use crate::utils::{uint_type_for_value, int_type_for_range, get_crate_name, get_source_path};

impl FontParams {
    pub fn embed(&self) -> TokenStream {
        let params = &self;

        let core_crate = get_crate_name("reui", &params.name.span());
        let name = &params.name;

        let path = get_source_path(&params.path.value(), &params.path.span());

        let chrs = params.chrs.iter().map(|chr| chr.range()).collect::<Vec<_>>();

        let font_data = match FontData::load(&path, &chrs) {
            Ok(font_data) => font_data,
            Err(error) => abort!(params.path, "Error when loading font: {}", error),
        };

        let codes_list = font_data.codes.iter().map(|range| {
            let start = Literal::u32_unsuffixed(*range.start());
            let end = Literal::u32_unsuffixed(*range.end());
            quote! { #start ..= #end }
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
                let Rect { x, y, w, h } = &rect_kinds[0];

                let x = Literal::i32_unsuffixed(*x);
                let y = Literal::i32_unsuffixed(*y);
                let w = Literal::i32_unsuffixed(*w);
                let h = Literal::i32_unsuffixed(*h);

                quote! {
                    #core_crate::Rect::new(
                        #core_crate::Point::new(#x, #y),
                        #core_crate::Size::new(#w, #h),
                    )
                }
            };

            quote! {
                pub static #name: #core_crate::FontV2<#core_crate::format::GS1, #dim_type, #code_type> =
                    #core_crate::FontV2::new(
                        #core_crate::format::GS1,
                        &[#(#codes_list),*],
                        #glyph_rect,
                        &[#(#pixels_list),*],
                    );
            }
        } else { // Font V1
            let glyphs_list = font_data.glyphs.iter().map(|GlyphData { rect: Rect { x, y, w, h }, off }| {
                let x = Literal::i32_unsuffixed(*x);
                let y = Literal::i32_unsuffixed(*y);
                let w = Literal::i32_unsuffixed(*w);
                let h = Literal::i32_unsuffixed(*h);
                let off = Literal::usize_unsuffixed(*off);
                quote! {
                    #core_crate::GlyphData::new(
                        #core_crate::Rect::new(
                            #core_crate::Point::new(#x, #y),
                            #core_crate::Size::new(#w, #h)),
                        #off
                    )
                }
            });

            let off_type = uint_type_for_value(font_data.max_off().unwrap() as u32);

            quote! {
                pub static #name: #core_crate::FontV1<#core_crate::format::GS1, #dim_type, #off_type, #code_type> =
                    #core_crate::FontV1::new(
                        #core_crate::format::GS1,
                        &[#(#codes_list),*],
                        &[#(#glyphs_list),*],
                        &[#(#pixels_list),*],
                    );
            }
        }
    }
}
