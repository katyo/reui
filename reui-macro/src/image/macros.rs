use proc_macro2::{TokenStream, Literal};
use proc_macro_error::abort;
use quote::{quote, format_ident};
use reui_core as base;

use super::{ImageAttr, ImageItem, ImageData, ColorFmt, ColorData};
use crate::utils::{get_crate_name, get_source_path};

pub struct ImageMacro {
    attr: ImageAttr,
    item: ImageItem,
}

impl ImageMacro {
    pub fn new(attr: ImageAttr, item: ImageItem) -> Self {
        Self { attr, item }
    }

    pub fn embed(&self) -> TokenStream {
        let core_crate = get_crate_name("reui-core", &self.item.ident.span());

        let attrs = &self.item.attrs;

        let vis = &self.item.vis;

        let static_token = &self.item.static_token;

        let ident = &self.item.ident;

        let path = get_source_path(&self.attr.path.value(), &self.attr.path.span());

        let image_data = match ImageData::load(&path) {
            Ok(image_data) => image_data,
            Err(error) => abort!(self.attr.path, "Error when loading image: {}", error),
        };

        let image_size = image_data.size;

        let size_width = Literal::u32_unsuffixed(image_size.w);
        let size_height = Literal::u32_unsuffixed(image_size.h);

        macro_rules! pixel_format {
            ($type: ident, $fmt: literal, $($bits: expr),+) => {
                {
                    let fmt = format_ident!($fmt, $($bits),+);
                    ($($bits +)+ 0,
                     quote! { #core_crate::format::#fmt },
                     quote! { <#core_crate::format::#fmt as #core_crate::ConstDefault>::DEFAULT })
                }
            };
        }

        let (fmt_bits, fmt_type, fmt_value) = match image_data.format {
            ColorFmt::GS(bits) => pixel_format!(GS, "GS{}", bits),
            ColorFmt::GSA(bits, abits) => pixel_format!(GSA, "GSA{}{}", bits, abits),
            ColorFmt::RGB(rbits, gbits, bbits) => pixel_format!(RGB, "RGB{}{}{}", rbits, gbits, bbits),
            ColorFmt::RGBA(rbits, gbits, bbits, abits) => pixel_format!(RGBA, "RGBA{}{}{}{}", rbits, gbits, bbits, abits),
            ColorFmt::IDX(bits, ref palette) => {
                let palette_data = palette.iter().map(|c| {
                    let r = Literal::u8_unsuffixed(c.r);
                    let g = Literal::u8_unsuffixed(c.g);
                    let b = Literal::u8_unsuffixed(c.b);
                    let a = Literal::u8_unsuffixed(c.a);
                    quote! { #r, #g, #b, #a }
                });

                let fmt = format_ident!("IDX{}", bits);
                (bits,
                 quote! { #core_crate::format::#fmt<(&#core_crate::format::RGBA8888, &[u8])> },
                 quote! { #core_crate::format::#fmt::new((
                     &#core_crate::format::RGBA8888,
                     &[#(#palette_data),*]
                 )) })
            },
        };

        let pixel_bytes = ((image_size.w * image_size.h * fmt_bits as u32 + 7) / 8) as usize;
        let mut pixel_data = Vec::with_capacity(pixel_bytes);
        pixel_data.resize(pixel_bytes, 0);

        use base::{ConstDefault, ColorBufMut};

        macro_rules! convert_pixels {
            ($fmt: ident, $src: ident, $dst: ident) => {
                {
                    let mut dest = (&base::format::$fmt::DEFAULT, $dst.as_mut() as &mut [u8]);
                    for (index, color) in $src.iter().enumerate() {
                        dest.set(index, *color);
                    }
                }
            };
        }

        match &image_data.pixels {
            ColorData::GS(pixels) => {
                match &image_data.format {
                    ColorFmt::GS(1) => convert_pixels!(GS1, pixels, pixel_data),
                    ColorFmt::GS(2) => convert_pixels!(GS2, pixels, pixel_data),
                    ColorFmt::GS(4) => convert_pixels!(GS4, pixels, pixel_data),
                    ColorFmt::GS(8) => convert_pixels!(GS8, pixels, pixel_data),
                    ColorFmt::GS(bits) => abort!(self.item.ident, "Unsupported color format GS{}", bits),
                    _ => unreachable!(),
                }
            },
            ColorData::GSA(pixels) => {
                match &image_data.format {
                    ColorFmt::GSA(3, 1) => convert_pixels!(GSA31, pixels, pixel_data),
                    ColorFmt::GSA(4, 4) => convert_pixels!(GSA44, pixels, pixel_data),
                    ColorFmt::GSA(7, 1) => convert_pixels!(GSA71, pixels, pixel_data),
                    ColorFmt::GSA(8, 8) => convert_pixels!(GSA88, pixels, pixel_data),
                    ColorFmt::GSA(bits, abits) => abort!(self.item.ident, "Unsupported color format GSA{}{}", bits, abits),
                    _ => unreachable!(),
                }
            },
            ColorData::RGB(pixels) => {
                match &image_data.format {
                    ColorFmt::RGB(3, 3, 2) => convert_pixels!(RGB332, pixels, pixel_data),
                    ColorFmt::RGB(4, 4, 4) => convert_pixels!(RGB444, pixels, pixel_data),
                    ColorFmt::RGB(5, 6, 5) => convert_pixels!(RGB565, pixels, pixel_data),
                    ColorFmt::RGB(8, 8, 8) => convert_pixels!(RGB888, pixels, pixel_data),
                    ColorFmt::RGB(rbits, gbits, bbits) => abort!(self.item.ident, "Unsupported color format RGB{}{}{}", rbits, gbits, bbits),
                    _ => unreachable!(),
                }
            },
            ColorData::RGBA(pixels) => {
                match &image_data.format {
                    ColorFmt::RGBA(4, 4, 4, 4) => convert_pixels!(RGBA4444, pixels, pixel_data),
                    ColorFmt::RGBA(5, 5, 5, 1) => convert_pixels!(RGBA5551, pixels, pixel_data),
                    ColorFmt::RGBA(8, 8, 8, 8) => convert_pixels!(RGBA8888, pixels, pixel_data),
                    ColorFmt::RGBA(rbits, gbits, bbits, abits) => abort!(self.item.ident, "Unsupported color format RGBA{}{}{}{}", rbits, gbits, bbits, abits),
                    _ => unreachable!(),
                }
            },
            ColorData::IDX(indexes) => {
                match &image_data.format {
                    ColorFmt::IDX(1, _) => convert_pixels!(RAW1, indexes, pixel_data),
                    ColorFmt::IDX(2, _) => convert_pixels!(RAW2, indexes, pixel_data),
                    ColorFmt::IDX(4, _) => convert_pixels!(RAW4, indexes, pixel_data),
                    ColorFmt::IDX(8, _) => convert_pixels!(RAW8, indexes, pixel_data),
                    ColorFmt::IDX(bits, _) => abort!(self.item.ident, "Unsupported color format IDX{}", bits),
                    _ => unreachable!(),
                }
            },
        };

        let pixel_data = pixel_data.iter().map(|byte| {
            let byte = Literal::u8_unsuffixed(*byte);
            quote! { #byte }
        });

        quote! {
            #(#attrs)*
            #vis #static_token #ident: #core_crate::PixelView<(&#fmt_type, &[u8])> =
            #core_crate::PixelView::new(
                #core_crate::Size::new(#size_width, #size_height),
                (&#fmt_value, &[#(#pixel_data),*]),
            );
        }
    }
}
