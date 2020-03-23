use std::{
    path::Path,
    convert::{TryFrom, TryInto},
};
use lodepng as png;
use reui_core as base;

#[derive(Clone, Copy)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[derive(Clone)]
pub enum ColorFmt {
    GS(u8),
    GSA(u8, u8),
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
    IDX(u8, Vec<base::RGBA>),
}

impl TryFrom<&png::ColorMode> for ColorFmt {
    type Error = String;

    fn try_from(info: &png::ColorMode) -> Result<Self, Self::Error> {
        use png::ColorType;

        Ok(match (info.colortype(), info.bitdepth()) {
            (ColorType::GREY, bits)
                if [1, 2, 4, 8].iter().any(|b| *b == bits) =>
                ColorFmt::GS(bits as u8),

            (ColorType::GREY_ALPHA, bits)
                if bits == 8 => ColorFmt::GSA(bits as u8, bits as u8),

            (ColorType::RGB, 8) => ColorFmt::RGB(8, 8, 8),
            (ColorType::RGBA, 8) => ColorFmt::RGBA(8, 8, 8, 8),

            (ColorType::PALETTE, bits)
                if [1, 2, 4, 8].iter().any(|b| *b == bits) =>
                ColorFmt::IDX(bits as u8, info.palette().iter()
                              .map(|c| base::RGBA::new(c.r, c.g, c.b, c.a)).collect()),

            (fmt, bits) =>
                return Err(format!("Unsupported color format {:?} {:?}", fmt, bits)),
        })
    }
}

#[derive(Clone)]
pub enum ColorData {
    GS(Vec<base::GS>),
    GSA(Vec<base::GSA>),
    RGB(Vec<base::RGB>),
    RGBA(Vec<base::RGBA>),
    IDX(Vec<u8>),
}

pub struct ImageData {
    pub size: Size,
    pub format: ColorFmt,
    pub pixels: ColorData,
}

impl ImageData {
    pub fn load(path: &Path) -> Result<Self, String> {
        let mut decoder = png::Decoder::new();

        // Decode to get png info
        decoder.decode_file(path).map_err(|err| err.to_string())?;

        let info = decoder.info_png();
        let color = info.color.clone();
        let bpp = color.bpp();
        let format = (&color).try_into()?;

        // Set source image info to decoder
        *decoder.info_raw_mut() = color;

        // Get decoded image
        let image = decoder.decode_file(path).map_err(|err| err.to_string())?;

        use png::Image;

        // Read pixel data
        let (width, height, pixels) = match image {
            Image::RawData(bitmap) => {
                let colors = bitmap.width * bitmap.height;
                let mut buffer = Vec::with_capacity(colors);
                buffer.resize(colors, 0);

                // color per byte
                let cpb = (8 / bpp) as usize;
                let msk = (!0u8) >> (8 - bpp);

                for index in 0..colors {
                    let byte = index / cpb;
                    let bit = (cpb - 1 - (index % cpb)) * bpp as usize;
                    buffer[index] = (bitmap.buffer[byte] >> bit) & msk;
                }

                (bitmap.width, bitmap.height, ColorData::IDX(buffer))
            },
            Image::Grey(bitmap) => (bitmap.width, bitmap.height,
                                    ColorData::GS(bitmap.buffer.iter()
                                                  .map(|c| base::GS::new(c.0)).collect())),
            Image::GreyAlpha(bitmap) => (bitmap.width, bitmap.height,
                                         ColorData::GSA(bitmap.buffer.iter()
                                                        .map(|c| base::GSA::new(c.0, c.1)).collect())),
            Image::RGB(bitmap) => (bitmap.width, bitmap.height,
                                   ColorData::RGB(bitmap.buffer.iter()
                                                  .map(|c| base::RGB::new(c.r, c.g, c.b)).collect())),
            Image::RGBA(bitmap) => (bitmap.width, bitmap.height,
                                    ColorData::RGBA(bitmap.buffer.iter()
                                                    .map(|c| base::RGBA::new(c.r, c.g, c.b, c.a)).collect())),
            _ => return Err("Unsupported pixel format.".into()),
        };

        Ok(Self {
            size: Size { w: width as u32, h: height as u32 },
            format,
            pixels,
        })
    }
}
