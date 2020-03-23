use crate::{ConstDefault};
use super::{ColorFmt, ColorGet, ColorSet, GS};

/// RGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    /// Create RGB color from components
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<(u8, u8, u8)> for RGB {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

impl Into<(u8, u8, u8)> for RGB {
    fn into(self) -> (u8, u8, u8) {
        let RGB { r, g, b} = self;
        (r, g, b)
    }
}

impl From<u8> for RGB {
    fn from(v: u8) -> Self {
        Self::new(v, v, v)
    }
}

impl Into<u8> for RGB {
    fn into(self) -> u8 {
        let RGB { r, g, b} = self;
        ((r as u16 + g as u16 + b as u16) / 3) as u8
    }
}

impl From<GS> for RGB {
    fn from(gs: GS) -> Self {
        let v = gs.into();
        Self::new(v, v, v)
    }
}

impl Into<GS> for RGB {
    fn into(self) -> GS {
        GS::new(self.into())
    }
}

/// RGB with 3 bits for red and green components and 2 bits for blue component
///
/// Single color per byte:
///
/// `0bRRRGGGBB ...`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGB332;

impl ConstDefault for RGB332 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for RGB332 {
    type ColorType = RGB;
    type ColorBits = typenum::U8;
    const COLOR_BITS: usize = 8;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len()
    }
}

impl ColorGet for RGB332 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index];

        Self::ColorType {
            r: (byte >> 5) * (255 / 7),
            g: ((byte >> 2) & 0b111) * (255 / 7),
            b: (byte & 0b11) * (255 / 3),
        }
    }
}

impl ColorSet for RGB332 {
    fn set_color(&self, buffer: &mut [u8], index: usize, RGB { r, g, b }: Self::ColorType) {
        buffer[index] = ((r >> (8 - 3)) << 5) | ((g >> (8 - 3)) << 2) | (b >> (8 - 2));
    }
}

/// RGB with 4 bits per component
///
/// Two colors per three bytes:
///
/// `0bRRRRGGGG 0bBBBBRRRR 0bGGGGBBBB ...`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGB444;

impl ConstDefault for RGB444 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for RGB444 {
    type ColorType = RGB;
    type ColorBits = typenum::U12;
    const COLOR_BITS: usize = 12;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 2 / 3
    }
}

impl ColorGet for RGB444 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let i = index * 3 / 2;
        let b0 = buffer[i];
        let b1 = buffer[i + 1];

        if (index % 2) > 0 {
            // (MSB) ----rrrr ggggbbbb
            Self::ColorType {
                r: (b0 & 0b1111) * (255 / 15),
                g: (b1 >> 4) * (255 / 15),
                b: (b1 & 0b1111) * (255 / 15),
            }
        } else {
            // (MSB) rrrrgggg bbbb----
            Self::ColorType {
                r: (b0 >> 4) * (255 / 15),
                g: (b0 & 0b1111) * (255 / 15),
                b: (b1 >> 4) * (255 / 15),
            }
        }
    }
}

impl ColorSet for RGB444 {
    fn set_color(&self, buffer: &mut [u8], index: usize, RGB { r, g, b }: Self::ColorType) {
        let i = index * 3 / 2;
        let bs = &mut buffer[i..i + 1];

        if (index % 2) > 0 {
            // (MSB) ----rrrr ggggbbbb
            bs[0] &= !0b1111;
            bs[0] |= r >> 4;
            bs[1] = (g & (0b1111 << 4)) | (b >> 4);
        } else {
            // (MSB) rrrrgggg bbbb----
            bs[0] = (r & (0b1111 << 4)) | (g >> 4);
            bs[1] &= !(0b1111 << 4);
            bs[1] |= b & (0b1111 << 4);
        }
    }
}

/// RGB with 5 bits for red and blue components and 6 bits for green component
///
/// Single color per each two bytes:
///
/// `0bRRRRRGGG 0bGGGBBBBB ...`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGB565;

impl ConstDefault for RGB565 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for RGB565 {
    type ColorType = RGB;
    type ColorBits = typenum::U16;
    const COLOR_BITS: usize = 16;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() / 2
    }
}

impl ColorGet for RGB565 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let i = index * 2;
        let b0 = buffer[i + 0];
        let b1 = buffer[i + 1];

        // (MSB) rrrrrggg gggbbbbb
        Self::ColorType {
            r: (b0 >> 3) * (255 / 31),
            g: (((b0 & 0b111) << 3) | (b1 >> 5)) * (255 / 63),
            b: (b1 & 0b11111) * (255 / 31),
        }
    }
}

impl ColorSet for RGB565 {
    fn set_color(&self, buffer: &mut [u8], index: usize, RGB { r, g, b }: Self::ColorType) {
        let i = index * 2;

        // (MSB) rrrrrggg gggbbbbb
        buffer[i] = (r & (0b11111 << 3)) | (g >> 5);
        buffer[i + 1] = ((g & (0b111 << 2)) << 3) | (b >> 3);
    }
}

/// RGB with 8 bits per component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGB888;

impl ConstDefault for RGB888 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for RGB888 {
    type ColorType = RGB;
    type ColorBits = typenum::U24;
    const COLOR_BITS: usize = 24;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() / 3
    }
}

impl ColorGet for RGB888 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let i = index * 3;

        let r = buffer[i + 0];
        let g = buffer[i + 1];
        let b = buffer[i + 2];

        Self::ColorType { r, g, b }
    }
}

impl ColorSet for RGB888 {
    fn set_color(&self, buffer: &mut [u8], index: usize, RGB { r, g, b }: Self::ColorType) {
        let i = index * 3;

        buffer[i + 0] = r;
        buffer[i + 1] = g;
        buffer[i + 2] = b;
    }
}
