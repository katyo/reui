use crate::{ConstDefault};
use super::{ColorFmt, ColorGet, ColorSet};

/// Grayscale
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GS {
    pub v: u8,
}

impl GS {
    pub fn new(v: u8) -> Self {
        GS { v }
    }
}

impl From<u8> for GS {
    fn from(v: u8) -> Self {
        Self { v }
    }
}

impl Into<u8> for GS {
    fn into(self) -> u8 {
        self.v
    }
}

/// 1-bit grayscale color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GS1;

impl ConstDefault for GS1 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for GS1 {
    type ColorType = GS;
    type ColorBits = typenum::U1;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 8
    }
}

impl ColorGet for GS1 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 8];
        let bit = 1u8 << (index % 8);
        Self::ColorType {
            v: if (byte & bit) > 0 { 255 } else { 0 }
        }
    }
}

impl ColorSet for GS1 {
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        let byte = &mut buffer[index / 8];
        let bit = 1u8 << (index % 8);
        if color.v > 127 {
            *byte |= bit;
        } else {
            *byte &= !bit;
        }
    }
}

/// 2-bit grayscale color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GS2;

impl ConstDefault for GS2 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for GS2 {
    type ColorType = GS;
    type ColorBits = typenum::U2;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 4
    }
}

impl ColorGet for GS2 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 4];
        let off = (index % 4) * 2;
        Self::ColorType {
            v: ((byte >> off) & 0b11) * (255/3)
        }
    }
}

impl ColorSet for GS2 {
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        let byte = &mut buffer[index / 4];
        let off = (index % 4) * 2;
        *byte &= !(0b11 << off);
        *byte |= (color.v >> 6) << off;
    }
}

/// 4-bit grayscale color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GS4;

impl ConstDefault for GS4 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for GS4 {
    type ColorType = GS;
    type ColorBits = typenum::U4;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 2
    }
}

impl ColorGet for GS4 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 2];
        let off = (index % 2) * 4;
        Self::ColorType {
            v: ((byte >> off) & 0b1111) * (255/15)
        }
    }
}

impl ColorSet for GS4 {
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        let byte = &mut buffer[index / 2];
        let off = (index % 2) * 4;
        *byte &= !(0b1111 << off);
        *byte |= (color.v >> 4) << off;
    }
}

/// 8-bit grayscale color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GS8;

impl ConstDefault for GS8 {
    const DEFAULT: Self = Self;
}

impl ColorFmt for GS8 {
    type ColorType = GS;
    type ColorBits = typenum::U8;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len()
    }
}

impl ColorGet for GS8 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        Self::ColorType { v: buffer[index] }
    }
}

impl ColorSet for GS8 {
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        buffer[index] = color.v;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gs1_get() {
        let buf = [1, 6];

        assert_eq!(GS1.num_colors(&buf), 16);
        assert_eq!(GS1.get_color(&buf, 0), 255.into());
        for i in 1..9 {
            assert_eq!(GS1.get_color(&buf, i), 0.into());
        }
        assert_eq!(GS1.get_color(&buf, 9), 255.into());
        assert_eq!(GS1.get_color(&buf, 10), 255.into());
        for i in 11..16 {
            assert_eq!(GS1.get_color(&buf, i), 0.into());
        }
    }

    #[test]
    fn gs1_set() {
        let mut buf = [1, 6];

        assert_eq!(GS1.num_colors(&mut buf), 16);

        GS1.set_color(&mut buf, 0, 0.into());
        GS1.set_color(&mut buf, 1, 255.into());
        GS1.set_color(&mut buf, 8, 255.into());
        GS1.set_color(&mut buf, 9, 0.into());

        assert_eq!(&buf, &[2, 5]);
    }

    #[test]
    fn gs2_get() {
        let buf = [1, 131];

        assert_eq!(GS2.num_colors(&buf), 8);
        assert_eq!(GS2.get_color(&buf, 0), 85.into());
        for i in 1..4 {
            assert_eq!(GS2.get_color(&buf, i), 0.into());
        }
        assert_eq!(GS2.get_color(&buf, 4), 255.into());
        for i in 5..7 {
            assert_eq!(GS2.get_color(&buf, i), 0.into());
        }
        assert_eq!(GS2.get_color(&buf, 7), 170.into());
    }

    #[test]
    fn gs2_set() {
        let mut buf = [1, 131];

        assert_eq!(GS2.num_colors(&mut buf), 8);

        GS2.set_color(&mut buf, 0, 0.into());
        GS2.set_color(&mut buf, 1, 255.into());
        GS2.set_color(&mut buf, 4, 175.into());
        GS2.set_color(&mut buf, 7, 90.into());

        assert_eq!(&buf, &[12, 66]);
    }
}
