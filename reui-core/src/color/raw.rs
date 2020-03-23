use crate::{ConstDefault};
use super::{ColorFmt, ColorGet, ColorSet};

/// 1-bit raw value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RAW1;

impl ConstDefault for RAW1 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for RAW1 {
    type ColorType = u8;
    type ColorBits = typenum::U1;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 8
    }
}

impl ColorGet for RAW1 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 8];
        let bit = 1u8 << (index % 8);
        if (byte & bit) > 0 { 255 } else { 0 }
    }
}

impl ColorSet for RAW1 {
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        let byte = &mut buffer[index / 8];
        let bit = 1u8 << (index % 8);
        if color > 0 {
            *byte |= bit;
        } else {
            *byte &= !bit;
        }
    }
}

/// 2-bit raw value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RAW2;

impl ConstDefault for RAW2 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for RAW2 {
    type ColorType = u8;
    type ColorBits = typenum::U2;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 4
    }
}

impl ColorGet for RAW2 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 4];
        let off = (index % 4) * 2;
        (byte >> off) & 0b11
    }
}

impl ColorSet for RAW2 {
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        let byte = &mut buffer[index / 4];
        let off = (index % 4) * 2;
        *byte &= !(0b11 << off);
        *byte |= (color & 0b11) << off;
    }
}

/// 4-bit raw value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RAW4;

impl ConstDefault for RAW4 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for RAW4 {
    type ColorType = u8;
    type ColorBits = typenum::U4;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 2
    }
}

impl ColorGet for RAW4 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 2];
        let off = (index % 2) * 4;
        (byte >> off) & 0b1111
    }
}

impl ColorSet for RAW4 {
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        let byte = &mut buffer[index / 2];
        let off = (index % 2) * 4;
        *byte &= !(0b1111 << off);
        *byte |= (color & 0b1111) << off;
    }
}

/// 8-bit grayscale color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RAW8;

impl ConstDefault for RAW8 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for RAW8 {
    type ColorType = u8;
    type ColorBits = typenum::U8;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len()
    }
}

impl ColorGet for RAW8 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        buffer[index]
    }
}

impl ColorSet for RAW8 {
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        buffer[index] = color;
    }
}
