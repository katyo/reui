use super::{ColorFmt, ColorGet, ColorSet, ColorBuf};

/// 1-bit indexed color format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IDX1<Buf> {
    colors: Buf,
}

impl<Buf> IDX1<Buf> {
    pub const fn new(colors: Buf) -> Self {
        Self { colors }
    }
}

impl<Buf> ColorFmt for IDX1<Buf>
where
    Buf: ColorBuf,
{
    type ColorType = <Buf::ColorFmt as ColorFmt>::ColorType;
    type ColorBits = typenum::U1;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 8
    }
}

impl<Buf> ColorGet for IDX1<Buf>
where
    Buf: ColorBuf,
{
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let color_index = ((buffer[index / 8] >> (index % 8)) & 0b1) as usize;

        self.colors.get(color_index)
    }
}

impl<Buf> ColorSet for IDX1<Buf>
where
    Buf: ColorBuf,
    <Buf::ColorFmt as ColorFmt>::ColorType: PartialEq,
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
pub struct IDX2<Buf> {
    colors: Buf,
}

impl<Buf> IDX2<Buf> {
    pub const fn new(colors: Buf) -> Self {
        Self { colors }
    }
}

impl<Buf> ColorFmt for IDX2<Buf>
where
    Buf: ColorBuf,
{
    type ColorType = <Buf::ColorFmt as ColorFmt>::ColorType;
    type ColorBits = typenum::U2;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 4
    }
}

impl<Buf> ColorGet for IDX2<Buf>
where
    Buf: ColorBuf,
{
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 4];
        let off = (index % 4) * 2;
        let color_index = ((byte >> off) & 0b11) as usize;

        self.colors.get(color_index)
    }
}

impl<Buf> ColorSet for IDX2<Buf>
where
    Buf: ColorBuf,
    <Buf::ColorFmt as ColorFmt>::ColorType: PartialEq,
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
pub struct IDX4<Buf> {
    colors: Buf,
}

impl<Buf> IDX4<Buf> {
    pub const fn new(colors: Buf) -> Self {
        Self { colors }
    }
}

impl<Buf> ColorFmt for IDX4<Buf>
where
    Buf: ColorBuf,
{
    type ColorType = <Buf::ColorFmt as ColorFmt>::ColorType;
    type ColorBits = typenum::U4;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len() * 2
    }
}

impl<Buf> ColorGet for IDX4<Buf>
where
    Buf: ColorBuf,
{
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let byte = buffer[index / 2];
        let off = (index % 2) * 4;
        let color_index = ((byte >> off) & 0b1111) as usize;

        self.colors.get(color_index)
    }
}

impl<Buf> ColorSet for IDX4<Buf>
where
    Buf: ColorBuf,
    <Buf::ColorFmt as ColorFmt>::ColorType: PartialEq,
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
pub struct IDX8<Buf> {
    colors: Buf,
}

impl<Buf> IDX8<Buf> {
    pub const fn new(colors: Buf) -> Self {
        Self { colors }
    }
}

impl<Buf> ColorFmt for IDX8<Buf>
where
    Buf: ColorBuf,
{
    type ColorType = <Buf::ColorFmt as ColorFmt>::ColorType;
    type ColorBits = typenum::U8;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        buffer.len()
    }
}

impl<Buf> ColorGet for IDX8<Buf>
where
    Buf: ColorBuf,
{
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        let color_index = buffer[index] as usize;

        self.colors.get(color_index)
    }
}

impl<Buf> ColorSet for IDX8<Buf>
where
    Buf: ColorBuf,
    <Buf::ColorFmt as ColorFmt>::ColorType: PartialEq,
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
