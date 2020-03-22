use core::marker::PhantomData;
use super::{ColorFmt, ColorGet, ColorSet, ColorBuf};

/// 1-bit indexed color format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IDX1<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    colors: Buf,
    _phantom: PhantomData<Fmt>,
}

impl<Fmt, Buf> IDX1<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    pub fn new(colors: Buf) -> Self {
        Self { colors, _phantom: PhantomData }
    }
}

impl<Fmt, Buf> ColorFmt for IDX1<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    type ColorType = Fmt::ColorType;
    type ColorBits = typenum::U1;
    const COLOR_BITS: usize = 1;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 8
    }
}

impl<Fmt, Buf> ColorGet for IDX1<Fmt, Buf>
where
    Fmt: ColorGet,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let color_index = ((buffer[index / 8] >> (index % 8)) & 0b1) as usize;

        self.colors.get(color_index)
    }
}

impl<Fmt, Buf> ColorSet for IDX1<Fmt, Buf>
where
    Fmt: ColorGet + ColorSet,
    Fmt::ColorType: PartialEq,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        (0usize..2).into_iter().filter_map(|color_index| {
            if self.colors.get(color_index) == color {
                Some(color_index)
            } else {
                None
            }
        }).next().map(|color_index| {
            let cell = &mut buffer[index / 8];
            let bit = 1u8 << (index % 8);
            if color_index > 0 {
                *cell |= bit;
            } else {
                *cell &= !bit;
            }
        });
    }
}

/// 2-bit indexed color format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IDX2<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    colors: Buf,
    _phantom: PhantomData<Fmt>,
}

impl<Fmt, Buf> IDX2<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    pub fn new(colors: Buf) -> Self {
        Self { colors, _phantom: PhantomData }
    }
}

impl<Fmt, Buf> ColorFmt for IDX2<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    type ColorType = Fmt::ColorType;
    type ColorBits = typenum::U2;
    const COLOR_BITS: usize = 2;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 4
    }
}

impl<Fmt, Buf> ColorGet for IDX2<Fmt, Buf>
where
    Fmt: ColorGet,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 4];
        let off = (index % 4) * 2;
        let color_index = ((byte >> off) & 0b11) as usize;

        self.colors.get(color_index)
    }
}

impl<Fmt, Buf> ColorSet for IDX2<Fmt, Buf>
where
    Fmt: ColorGet + ColorSet,
    Fmt::ColorType: PartialEq,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        (0usize..4).into_iter().filter_map(|color_index| {
            if self.colors.get(color_index) == color {
                Some(color_index as u8)
            } else {
                None
            }
        }).next().map(|color_index| {
            let byte = &mut buffer[index / 4];
            let off = (index % 4) * 2;
            *byte &= !(0b11 << off);
            *byte |= (color_index >> 6) << off;
        });
    }
}

/// 4-bit indexed color format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IDX4<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    colors: Buf,
    _phantom: PhantomData<Fmt>,
}

impl<Fmt, Buf> IDX4<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    pub fn new(colors: Buf) -> Self {
        Self { colors, _phantom: PhantomData }
    }
}

impl<Fmt, Buf> ColorFmt for IDX4<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    type ColorType = Fmt::ColorType;
    type ColorBits = typenum::U4;
    const COLOR_BITS: usize = 4;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 2
    }
}

impl<Fmt, Buf> ColorGet for IDX4<Fmt, Buf>
where
    Fmt: ColorGet,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 2];
        let off = (index % 2) * 4;
        let color_index = ((byte >> off) & 0b1111) as usize;

        self.colors.get(color_index)
    }
}

impl<Fmt, Buf> ColorSet for IDX4<Fmt, Buf>
where
    Fmt: ColorGet + ColorSet,
    Fmt::ColorType: PartialEq,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        (0usize..16).into_iter().filter_map(|color_index| {
            if self.colors.get(color_index) == color {
                Some(color_index as u8)
            } else {
                None
            }
        }).next().map(|color_index| {
            let byte = &mut buffer[index / 2];
            let off = (index % 2) * 4;
            *byte &= !(0b1111 << off);
            *byte |= (color_index >> 6) << off;
        });
    }
}

/// 8-bit indexed color format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IDX8<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    colors: Buf,
    _phantom: PhantomData<Fmt>,
}

impl<Fmt, Buf> IDX8<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    pub fn new(colors: Buf) -> Self {
        Self { colors, _phantom: PhantomData }
    }
}

impl<Fmt, Buf> ColorFmt for IDX8<Fmt, Buf>
where
    Fmt: ColorFmt,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    type ColorType = Fmt::ColorType;
    type ColorBits = typenum::U8;
    const COLOR_BITS: usize = 8;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len()
    }
}

impl<Fmt, Buf> ColorGet for IDX8<Fmt, Buf>
where
    Fmt: ColorGet,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let color_index = buffer[index] as usize;

        self.colors.get(color_index)
    }
}

impl<Fmt, Buf> ColorSet for IDX8<Fmt, Buf>
where
    Fmt: ColorGet + ColorSet,
    Fmt::ColorType: PartialEq,
    Buf: ColorBuf<ColorFmt = Fmt>,
{
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        (0usize..256).into_iter().filter_map(|color_index| {
            if self.colors.get(color_index) == color {
                Some(color_index as u8)
            } else {
                None
            }
        }).next().map(|color_index| {
            buffer[index] = color_index;
        });
    }
}
