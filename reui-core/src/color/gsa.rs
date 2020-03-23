use crate::{ConstDefault};
use super::{ColorFmt, ColorGet, ColorSet, GS};

/// Grayscale with alpha
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GSA {
    pub v: u8,
    pub a: u8,
}

impl GSA {
    pub fn new(v: u8, a: u8) -> Self {
        GSA { v, a }
    }
}

impl Default for GSA {
    fn default() -> Self {
        Self { v: 0, a: 255 }
    }
}

impl From<(u8, u8)> for GSA {
    fn from((v, a): (u8, u8)) -> Self {
        Self { v, a }
    }
}

impl Into<(u8, u8)> for GSA {
    fn into(self) -> (u8, u8) {
        (self.v, self.a)
    }
}

impl From<u8> for GSA {
    fn from(v: u8) -> Self {
        Self { v, a: 255 }
    }
}

impl Into<u8> for GSA {
    fn into(self) -> u8 {
        self.v
    }
}

impl From<GS> for GSA {
    fn from(gs: GS) -> Self {
        GSA::new(gs.into(), 255)
    }
}

impl Into<GS> for GSA {
    fn into(self) -> GS {
        GS::new(self.into())
    }
}

/// 1-bit grayscale color with 1-bit alpha channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GSA11;

impl ConstDefault for GSA11 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for GSA11 {
    type ColorType = GSA;
    type ColorBits = typenum::U2;
    const COLOR_BITS: usize = 2;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 4
    }
}

impl ColorGet for GSA11 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 4];
        let off = (index % 4) * 2;
        Self::ColorType {
            v: if (byte & (0b10 << off)) > 0 { 255 } else { 0 },
            a: if (byte & (0b01 << off)) > 0 { 255 } else { 0 },
        }
    }
}

impl ColorSet for GSA11 {
    fn set_color(&self, buffer: &mut [u8], index: usize, GSA { v, a }: Self::ColorType) {
        let byte = &mut buffer[index / 4];
        let off = (index % 4) * 2;
        if v > 127 {
            *byte |= 0b10 << off;
        } else {
            *byte &= !(0b10 << off);
        }
        if a > 127 {
            *byte |= 0b01 << off;
        } else {
            *byte &= !(0b01 << off);
        }
    }
}

/// 2-bit grayscale color with 2-bit alpha channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GSA22;

impl ConstDefault for GSA22 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for GSA22 {
    type ColorType = GSA;
    type ColorBits = typenum::U4;
    const COLOR_BITS: usize = 4;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 2
    }
}

impl ColorGet for GSA22 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 2];
        let off = (index % 2) * 4;
        Self::ColorType {
            v: ((byte >> (off + 1)) & 0b11) * (255/3),
            a: ((byte >> off) & 0b11) * (255/3),
        }
    }
}

impl ColorSet for GSA22 {
    fn set_color(&self, buffer: &mut [u8], index: usize, GSA { v, a }: Self::ColorType) {
        let byte = &mut buffer[index / 2];
        let off = (index % 2) * 4;
        *byte &= !(0b1111 << off);
        *byte |= (((v >> 6) << 2) | (a >> 6)) << off;
    }
}

/// 3-bit grayscale color with 1-bit alpha channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GSA31;

impl ConstDefault for GSA31 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for GSA31 {
    type ColorType = GSA;
    type ColorBits = typenum::U4;
    const COLOR_BITS: usize = 4;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 2
    }
}

impl ColorGet for GSA31 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 2];
        let off = (index % 2) * 4;
        Self::ColorType {
            v: ((byte >> (off + 1)) & 0b111) * (255/7),
            a: if (byte & (0b1 << off)) > 0 { 255 } else { 0 },
        }
    }
}

impl ColorSet for GSA31 {
    fn set_color(&self, buffer: &mut [u8], index: usize, GSA { v, a }: Self::ColorType) {
        let byte = &mut buffer[index / 2];
        let off = (index % 2) * 4;
        *byte &= !(0b1111 << off);
        *byte |= (((v >> 5) << 1) | (a >> 7)) << off;
    }
}

/// 4-bit grayscale color with 4-bit alpha channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GSA44;

impl ConstDefault for GSA44 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for GSA44 {
    type ColorType = GSA;
    type ColorBits = typenum::U8;
    const COLOR_BITS: usize = 8;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len()
    }
}

impl ColorGet for GSA44 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        Self::ColorType {
            v: (buffer[index] >> 4) * (255/15),
            a: (buffer[index] & 0b1111) * (255/15),
        }
    }
}

impl ColorSet for GSA44 {
    fn set_color(&self, buffer: &mut [u8], index: usize, GSA { v, a }: Self::ColorType) {
        buffer[index] = (v & (0b1111 << 4)) | (a >> 4);
    }
}

/// 7-bit grayscale color with 1-bit alpha channel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GSA71;

impl ConstDefault for GSA71 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for GSA71 {
    type ColorType = GSA;
    type ColorBits = typenum::U8;
    const COLOR_BITS: usize = 8;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len()
    }
}

impl ColorGet for GSA71 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        Self::ColorType {
            v: (buffer[index] >> 1) * (255/127),
            a: if (buffer[index] & 0b1) > 0 { 255 } else { 0 },
        }
    }
}

impl ColorSet for GSA71 {
    fn set_color(&self, buffer: &mut [u8], index: usize, GSA { v, a }: Self::ColorType) {
        buffer[index] = (v & (0b1111111 << 1)) | (a >> (8 - 1));
    }
}
