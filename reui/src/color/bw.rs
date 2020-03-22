//use core::ops::Range;
use super::{ColorFmt, ColorGet, ColorSet};

/// Black and white color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BW {
    pub v: bool,
}

impl From<bool> for BW {
    fn from(v: bool) -> Self {
        Self { v }
    }
}

impl Into<bool> for BW {
    fn into(self) -> bool {
        self.v
    }
}

/// 1-bit black/white color format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BW1;

impl ColorFmt for BW1 {
    type ColorType = BW;
    type ColorBits = typenum::U1;
    const COLOR_BITS: usize = 1;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 8
    }
}

impl ColorGet for BW1 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        Self::ColorType { v: buffer[index / 8] & (1u8 << (index % 8)) > 0 }
    }

    /*type ColorIter = BW1Iter<'a>;

    fn get_colors(&self, buffer: &[u8], range: Range<usize>, length: usize, stride: usize) -> Self::ColorIter {

    }*/
}

/*pub struct BW1Iter<'b> {
    buffer: &'b [u8],
    byte: usize,
    bit: u8,
}

impl<'b> Iterator for BW1Iter<'b> {
    type Item = BW;

    fn next(&mut self) -> Option<Self::Item> {

    }
}*/

impl ColorSet for BW1 {
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        let cell = &mut buffer[index / 8];
        let bit = 1u8 << (index % 8);
        if color.v {
            *cell |= bit;
        } else {
            *cell &= !bit;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bw1_get() {
        let buf = [1, 2, 0];

        assert_eq!(BW1.num_colors(&buf), 8 * 3);
        assert_eq!(BW1.get_color(&buf, 0), true.into());
        assert_eq!(BW1.get_color(&buf, 1), false.into());
        assert_eq!(BW1.get_color(&buf, 2), false.into());
        assert_eq!(BW1.get_color(&buf, 8), false.into());
        assert_eq!(BW1.get_color(&buf, 9), true.into());
        assert_eq!(BW1.get_color(&buf, 10), false.into());
    }

    #[test]
    fn bw1_set() {
        let mut buf = [1, 2, 0];

        assert_eq!(BW1.num_colors(&buf), 8 * 3);
        assert_eq!(BW1.get_color(&buf, 0), true.into());
        assert_eq!(BW1.get_color(&buf, 1), false.into());
        assert_eq!(BW1.get_color(&buf, 2), false.into());
        assert_eq!(BW1.get_color(&buf, 8), false.into());
        assert_eq!(BW1.get_color(&buf, 9), true.into());
        assert_eq!(BW1.get_color(&buf, 10), false.into());

        BW1.set_color(&mut buf, 0, false.into());
        BW1.set_color(&mut buf, 1, true.into());
        BW1.set_color(&mut buf, 8, false.into());
        BW1.set_color(&mut buf, 9, true.into());

        assert_eq!(BW1.get_color(&buf, 0), false.into());
        assert_eq!(BW1.get_color(&buf, 1), true.into());
        assert_eq!(BW1.get_color(&buf, 2), false.into());
        assert_eq!(BW1.get_color(&buf, 8), false.into());
        assert_eq!(BW1.get_color(&buf, 9), true.into());
        assert_eq!(BW1.get_color(&buf, 10), false.into());
        assert_eq!(&buf, &[2, 2, 0]);

        BW1.set_color(&mut buf, 8, true.into());
        BW1.set_color(&mut buf, 9, false.into());

        assert_eq!(BW1.get_color(&buf, 0), false.into());
        assert_eq!(BW1.get_color(&buf, 1), true.into());
        assert_eq!(BW1.get_color(&buf, 2), false.into());
        assert_eq!(BW1.get_color(&buf, 8), true.into());
        assert_eq!(BW1.get_color(&buf, 9), false.into());
        assert_eq!(BW1.get_color(&buf, 10), false.into());
        assert_eq!(&buf, &[2, 1, 0]);
    }
}
