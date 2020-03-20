use core::marker::PhantomData;
use super::{ColorFmt, ColorGet, ColorSet, ColorBuf};
use crate::{IsIndex};

/// 1-bit indexed color format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IDX1<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Buf: ColorBuf<Fmt, Dim>,
{
    colors: Buf,
    _phantom: PhantomData<(Fmt, Dim)>,
}

impl<Fmt, Dim, Buf> IDX1<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Buf: ColorBuf<Fmt, Dim>,
{
    pub fn new(colors: Buf) -> Self {
        Self { colors, _phantom: PhantomData }
    }
}

impl<Fmt, Dim, Buf> ColorFmt<Dim> for IDX1<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    type ColorType = Fmt::ColorType;
    type ColorBits = typenum::U1;
    const COLOR_BITS: usize = 1;

    fn num_colors(&self, buffer: &[u8]) -> Dim {
        Dim::from_index(buffer.len() * 8)
    }
}

impl<Fmt, Dim, Buf> ColorGet<Dim> for IDX1<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    fn get_color(&self, buffer: &[u8], index: Dim) -> Self::ColorType {
        let index = index.into_index();
        let color_index = ((buffer[index / 8] >> (index % 8)) & 0b1) as usize;

        self.colors.get(Dim::from_index(color_index))
    }
}

impl<Fmt, Dim, Buf> ColorSet<Dim> for IDX1<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim> + ColorSet<Dim>,
    Fmt::ColorType: PartialEq,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    fn set_color(&self, buffer: &mut [u8], index: Dim, color: Self::ColorType) {
        let index = index.into_index();

        (0usize..2).into_iter().filter_map(|color_index| {
            if self.colors.get(Dim::from_index(color_index)) == color {
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
pub struct IDX2<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Buf: ColorBuf<Fmt, Dim>,
{
    colors: Buf,
    _phantom: PhantomData<(Fmt, Dim)>,
}

impl<Fmt, Dim, Buf> IDX2<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Buf: ColorBuf<Fmt, Dim>,
{
    pub fn new(colors: Buf) -> Self {
        Self { colors, _phantom: PhantomData }
    }
}

impl<Fmt, Dim, Buf> ColorFmt<Dim> for IDX2<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    type ColorType = Fmt::ColorType;
    type ColorBits = typenum::U2;
    const COLOR_BITS: usize = 2;

    fn num_colors(&self, buffer: &[u8]) -> Dim {
        Dim::from_index(buffer.len() * 4)
    }
}

impl<Fmt, Dim, Buf> ColorGet<Dim> for IDX2<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    fn get_color(&self, buffer: &[u8], index: Dim) -> Self::ColorType {
        let index = index.into_index();

        let byte = buffer[index / 4];
        let off = (index % 4) * 2;
        let color_index = ((byte >> off) & 0b11) as usize;

        self.colors.get(Dim::from_index(color_index))
    }
}

impl<Fmt, Dim, Buf> ColorSet<Dim> for IDX2<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim> + ColorSet<Dim>,
    Fmt::ColorType: PartialEq,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    fn set_color(&self, buffer: &mut [u8], index: Dim, color: Self::ColorType) {
        let index = index.into_index();

        (0usize..4).into_iter().filter_map(|color_index| {
            if self.colors.get(Dim::from_index(color_index)) == color {
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
pub struct IDX4<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Buf: ColorBuf<Fmt, Dim>,
{
    colors: Buf,
    _phantom: PhantomData<(Fmt, Dim)>,
}

impl<Fmt, Dim, Buf> IDX4<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Buf: ColorBuf<Fmt, Dim>,
{
    pub fn new(colors: Buf) -> Self {
        Self { colors, _phantom: PhantomData }
    }
}

impl<Fmt, Dim, Buf> ColorFmt<Dim> for IDX4<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    type ColorType = Fmt::ColorType;
    type ColorBits = typenum::U4;
    const COLOR_BITS: usize = 4;

    fn num_colors(&self, buffer: &[u8]) -> Dim {
        Dim::from_index(buffer.len() * 2)
    }
}

impl<Fmt, Dim, Buf> ColorGet<Dim> for IDX4<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    fn get_color(&self, buffer: &[u8], index: Dim) -> Self::ColorType {
        let index = index.into_index();

        let byte = buffer[index / 2];
        let off = (index % 2) * 4;
        let color_index = ((byte >> off) & 0b1111) as usize;

        self.colors.get(Dim::from_index(color_index))
    }
}

impl<Fmt, Dim, Buf> ColorSet<Dim> for IDX4<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim> + ColorSet<Dim>,
    Fmt::ColorType: PartialEq,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    fn set_color(&self, buffer: &mut [u8], index: Dim, color: Self::ColorType) {
        let index = index.into_index();

        (0usize..16).into_iter().filter_map(|color_index| {
            if self.colors.get(Dim::from_index(color_index)) == color {
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
pub struct IDX8<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Buf: ColorBuf<Fmt, Dim>,
{
    colors: Buf,
    _phantom: PhantomData<(Fmt, Dim)>,
}

impl<Fmt, Dim, Buf> IDX8<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Buf: ColorBuf<Fmt, Dim>,
{
    pub fn new(colors: Buf) -> Self {
        Self { colors, _phantom: PhantomData }
    }
}

impl<Fmt, Dim, Buf> ColorFmt<Dim> for IDX8<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    type ColorType = Fmt::ColorType;
    type ColorBits = typenum::U8;
    const COLOR_BITS: usize = 8;

    fn num_colors(&self, buffer: &[u8]) -> Dim {
        Dim::from_index(buffer.len())
    }
}

impl<Fmt, Dim, Buf> ColorGet<Dim> for IDX8<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim>,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    fn get_color(&self, buffer: &[u8], index: Dim) -> Self::ColorType {
        let index = index.into_index();
        let color_index = buffer[index] as usize;

        self.colors.get(Dim::from_index(color_index))
    }
}

impl<Fmt, Dim, Buf> ColorSet<Dim> for IDX8<Fmt, Dim, Buf>
where
    Fmt: ColorGet<Dim> + ColorSet<Dim>,
    Fmt::ColorType: PartialEq,
    Dim: IsIndex,
    Buf: ColorBuf<Fmt, Dim>,
{
    fn set_color(&self, buffer: &mut [u8], index: Dim, color: Self::ColorType) {
        let index = index.into_index();

        (0usize..256).into_iter().filter_map(|color_index| {
            if self.colors.get(Dim::from_index(color_index)) == color {
                Some(color_index as u8)
            } else {
                None
            }
        }).next().map(|color_index| {
            buffer[index] = color_index;
        });
    }
}
