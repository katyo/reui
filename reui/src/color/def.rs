use core::ops::Range;
use typenum::Unsigned;

/// Color format definition
pub trait ColorFmt<Dim> {
    /// The type of color data
    type ColorType;

    /// The packed size of color in number of bits (typenum)
    type ColorBits: Unsigned;

    /// The packed size of color in number of bits (constant)
    const COLOR_BITS: usize;

    /// Measures the number of colors which can fit into buffer
    fn num_colors(&self, buffer: &[u8]) -> Dim;
}

/// Color getter
pub trait ColorGet<Dim>: ColorFmt<Dim> {
    /// Gets color which located at specified index
    fn get_color(&self, buffer: &[u8], index: Dim) -> Self::ColorType;
}

/// Color setter
pub trait ColorSet<Dim>: ColorFmt<Dim> {
    /// Sets color which located at specified index
    fn set_color(&self, buffer: &mut [u8], index: Dim, color: Self::ColorType);
}

/// Mass color getter
pub trait ColorsGet<Dim>: ColorFmt<Dim> {
    /// The type of iterator for colors
    type ColorIter: Iterator<Item = Self::ColorType>;

    /// Gets colors at specified range by pieces of specified length with specified stride
    fn get_colors(&self, buffer: &[u8], range: Range<Dim>, length: Dim, stride: Dim) -> Self::ColorIter;
}

/// Mass color setter
pub trait ColorsSet<Dim>: ColorFmt<Dim> {
    /// Sets colors at specified range by pieces of specified length with specified stride
    fn set_colors(&self, buffer: &mut [u8], range: Range<Dim>, length: Dim, stride: Dim, colors: &mut dyn Iterator<Item = Self::ColorType>);
}

pub trait ColorBuf<Fmt, Dim>
where
    Fmt: ColorGet<Dim>,
{
    /// Number of colors in buffer
    fn len(&self) -> Dim;

    /// Get color from buffer by index
    fn get(&self, index: Dim) -> Fmt::ColorType;
}

pub trait ColorBufMut<Fmt, Dim>
where
    Fmt: ColorSet<Dim>,
{
    /// Set color in buffer by index
    fn set(&mut self, index: Dim, color: Fmt::ColorType);
}

impl<Fmt, Dim, Buf> ColorBuf<Fmt, Dim> for (Buf,)
where
    Fmt: ColorGet<Dim> + Default,
    Buf: AsRef<[u8]>,
{
    fn len(&self) -> Dim {
        Fmt::default().num_colors(self.0.as_ref())
    }

    fn get(&self, index: Dim) -> Fmt::ColorType
    where
        Fmt: ColorGet<Dim>,
    {
        Fmt::default().get_color(self.0.as_ref(), index)
    }
}

impl<Fmt, Dim, Buf> ColorBufMut<Fmt, Dim> for (Buf,)
where
    Fmt: ColorSet<Dim> + Default,
    Buf: AsMut<[u8]>,
{
    fn set(&mut self, index: Dim, color: Fmt::ColorType)
    where
        Fmt: ColorSet<Dim>,
    {
        Fmt::default().set_color(self.0.as_mut(), index, color)
    }
}

impl<Fmt, Dim, Buf> ColorBuf<Fmt, Dim> for (Fmt, Buf)
where
    Fmt: ColorGet<Dim> + Default,
    Buf: AsRef<[u8]>,
{
    fn len(&self) -> Dim {
        self.0.num_colors(self.1.as_ref())
    }

    fn get(&self, index: Dim) -> Fmt::ColorType
    where
        Fmt: ColorGet<Dim>,
    {
        self.0.get_color(self.1.as_ref(), index)
    }
}

impl<Fmt, Dim, Buf> ColorBufMut<Fmt, Dim> for (Fmt, Buf)
where
    Fmt: ColorSet<Dim> + Default,
    Buf: AsMut<[u8]> + AsRef<Fmt>,
{
    fn set(&mut self, index: Dim, color: Fmt::ColorType)
    where
        Fmt: ColorSet<Dim>,
    {
        self.0.set_color(self.1.as_mut(), index, color)
    }
}
