use super::{ColorFmt, ColorGet, ColorSet, GS, GSA, RGB};

/// RGB with alpha
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    /// Create RGBA color from components
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA { r, g, b, a }
    }
}

impl Default for RGBA {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl From<(u8, u8, u8, u8)> for RGBA {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self { r, g, b, a }
    }
}

impl Into<(u8, u8, u8, u8)> for RGBA {
    fn into(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

impl From<(u8, u8, u8)> for RGBA {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl Into<(u8, u8, u8)> for RGBA {
    fn into(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl From<u8> for RGBA {
    fn from(v: u8) -> Self {
        Self::new(v, v, v, 255)
    }
}

impl Into<u8> for RGBA {
    fn into(self) -> u8 {
        let RGBA { r, g, b, .. } = self;
        ((r as u16 + g as u16 + b as u16) / 3) as u8
    }
}

impl From<GS> for RGBA {
    fn from(gs: GS) -> Self {
        let v = gs.into();
        Self::new(v, v, v, 255)
    }
}

impl Into<GS> for RGBA {
    fn into(self) -> GS {
        GS::new(self.into())
    }
}

impl From<GSA> for RGBA {
    fn from(GSA { v, a }: GSA) -> Self {
        Self::new(v, v, v, a)
    }
}

impl Into<GSA> for RGBA {
    fn into(self) -> GSA {
        GSA::new(self.into(), self.a)
    }
}

impl From<RGB> for RGBA {
    fn from(RGB { r, g, b }: RGB) -> Self {
        Self::new(r, g, b, 255)
    }
}

impl Into<RGB> for RGBA {
    fn into(self) -> RGB {
        let RGBA { r, g, b, .. } = self;
        RGB::new(r, g, b)
    }
}

/// RGBA with 4-bits per component
///
/// Single color per each two bytes:
///
/// `0bRRRRGGGG 0bBBBBAAAA`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGBA4444;

impl ColorFmt for RGBA4444 {
    type ColorType = RGBA;
    type ColorBits = typenum::U8;
    const COLOR_BITS: usize = 16;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() / 2
    }
}

impl ColorGet for RGBA4444 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let i = index * 2;
        let bs = &buffer[i .. i + 2];

        // (MSB) rrrrgggg bbbbaaaa
        Self::ColorType {
            r: (bs[0] >> 4) * (255/15),
            g: (bs[0] & 0b1111) * (255/15),
            b: (bs[1] >> 4) * (255/15),
            a: (bs[1] & 0b1111) * (255/15),
        }
    }
}

impl ColorSet for RGBA4444 {
    fn set_color(&self, buffer: &mut [u8], index: usize, RGBA { r, g, b, a }: Self::ColorType) {
        let i = index * 2;
        let bs = &mut buffer[i .. i + 2];

        // (MSB) rrrrgggg bbbbaaaa
        bs[0] = (r & (0b1111 << 4)) | (g >> 4);
        bs[1] = (b & (0b1111 << 4)) | (a >> 4);
    }
}

/// RGBA with 5-bits per color component and 1-bit for alpha
///
/// Single color per each two bytes:
///
/// `0bRRRRRGGG 0bGGBBBBBA`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGBA5551;

impl ColorFmt for RGBA5551 {
    type ColorType = RGBA;
    type ColorBits = typenum::U8;
    const COLOR_BITS: usize = 16;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() / 2
    }
}

impl ColorGet for RGBA5551 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let i = index * 2;
        let bs = &buffer[i .. i + 2];

        // (MSB) rrrrrggg ggbbbbba
        Self::ColorType {
            r: (bs[0] >> 3) * (255/31),
            g: (((bs[0] & 7) << 2) | (bs[1] >> 6)) * (255/31),
            b: ((bs[1] >> 1) & 31) * (255/31),
            a: if (bs[1] & 1) > 0 { 255 } else { 0 },
        }
    }
}

impl ColorSet for RGBA5551 {
    fn set_color(&self, buffer: &mut [u8], index: usize, RGBA { r, g, b, a }: Self::ColorType) {
        let i = index * 2;
        let bs = &mut buffer[i .. i + 2];

        // (MSB) rrrrrggg ggbbbbba
        bs[0] = (r & (31 << 3)) | (g >> 5);
        bs[1] = ((g & (3 << 3)) << 3) | (b >> 3) | (a >> 7);
    }
}

/// RGBA with 8-bits per component
///
/// Single color per each four bytes:
///
/// `0bRRRRRRRR 0bGGGGGGGG 0bBBBBBBBB 0bAAAAAAAA`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGBA8888;

impl ColorFmt for RGBA8888 {
    type ColorType = RGBA;
    type ColorBits = typenum::U8;
    const COLOR_BITS: usize = 16;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() / 4
    }
}

impl ColorGet for RGBA8888 {
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let i = index * 4;
        let bs = &buffer[i .. i + 4];

        Self::ColorType {
            r: bs[0],
            g: bs[1],
            b: bs[2],
            a: bs[3],
        }
    }
}

impl ColorSet for RGBA8888 {
    fn set_color(&self, buffer: &mut [u8], index: usize, RGBA { r, g, b, a }: Self::ColorType) {
        let i = index * 4;
        let bs = &mut buffer[i .. i + 4];

        bs[0] = r;
        bs[1] = g;
        bs[2] = b;
        bs[3] = a;
    }
}
