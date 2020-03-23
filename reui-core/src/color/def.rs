use typenum::Unsigned;

/// Color format definition
pub trait ColorFmt {
    /// The type of color data
    type ColorType;

    /// The packed size of color in number of bits (typenum)
    type ColorBits: Unsigned;

    /// The packed size of color in number of bits (constant)
    const COLOR_BITS: usize;

    /// Measures the number of colors which can fit into buffer
    fn num_colors(&self, buffer: &[u8]) -> usize;
}

/// Color getter
pub trait ColorGet: ColorFmt {
    /// Gets color which located at specified index
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType;

    //// The type of iterator for colors
    //type ColorIter: Iterator<Item = Self::ColorType>;

    //// Gets colors at specified range by pieces of specified length with specified stride
    //fn get_colors(&self, buffer: &[u8], range: Range<usize>, length: usize, stride: usize) -> Self::ColorIter;
}

/// Color setter
pub trait ColorSet: ColorFmt {
    /// Sets color which located at specified index
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType);

    //// Sets colors at specified range by pieces of specified length with specified stride
    //fn set_colors(&self, buffer: &mut [u8], range: Range<usize>, length: usize, stride: usize, colors: &mut dyn Iterator<Item = Self::ColorType>);
}

pub trait ColorBuf {
    type ColorFmt: ColorFmt;

    /// Number of colors in buffer
    fn len(&self) -> usize;

    /// Get color from buffer by index
    fn get(&self, index: usize) -> <Self::ColorFmt as ColorFmt>::ColorType;

    //// Get colors at specified range by pieces of specified length with specified stride
    //fn gets(&self, range: Range<usize>, length: usize, stride: usize) -> Fmt::ColorIter;
}

pub trait ColorBufMut: ColorBuf {
    /// Set color in buffer by index
    fn set(&mut self, index: usize, color: <Self::ColorFmt as ColorFmt>::ColorType);
}

impl<Fmt> ColorFmt for &Fmt
where
    Fmt: ColorFmt,
{
    type ColorType = Fmt::ColorType;

    type ColorBits = Fmt::ColorBits;

    const COLOR_BITS: usize = Fmt::COLOR_BITS;

    fn num_colors(&self, buffer: &[u8]) -> usize {
        (*self).num_colors(buffer)
    }
}

impl<Fmt> ColorGet for &Fmt
where
    Fmt: ColorGet,
{
    fn get_color(&self, buffer: &[u8], index: usize) -> Self::ColorType {
        (*self).get_color(buffer, index)
    }
}

impl<Fmt> ColorSet for &Fmt
where
    Fmt: ColorSet,
{
    fn set_color(&self, buffer: &mut [u8], index: usize, color: Self::ColorType) {
        (*self).set_color(buffer, index, color);
    }
}

impl<Fmt, Buf> ColorBuf for (Fmt, Buf)
where
    Fmt: ColorGet,
    Buf: AsRef<[u8]>,
{
    type ColorFmt = Fmt;

    fn len(&self) -> usize {
        self.0.num_colors(self.1.as_ref())
    }

    fn get(&self, index: usize) -> Fmt::ColorType {
        self.0.get_color(self.1.as_ref(), index)
    }

    /*fn gets(&self, range: Range<usize>, length: usize, stride: usize) -> Fmt::ColorIter {
        self.0.get_colors(self.1.as_ref(), range, length, stride)
    }*/
}

impl<Fmt, Buf> ColorBufMut for (Fmt, Buf)
where
    Fmt: ColorGet + ColorSet,
    Buf: AsRef<[u8]> + AsMut<[u8]>,
{
    fn set(&mut self, index: usize, color: Fmt::ColorType) {
        self.0.set_color(self.1.as_mut(), index, color)
    }
}

/*impl<Fmt, Buf> ColorBuf for (&Fmt, Buf)
where
    Fmt: ColorGet,
    Buf: AsRef<[u8]>,
{
    type ColorFmt = Fmt;

    fn len(&self) -> usize {
        self.0.num_colors(self.1.as_ref())
    }

    fn get(&self, index: usize) -> Fmt::ColorType {
        self.0.get_color(self.1.as_ref(), index)
    }

    /*fn gets(&self, range: Range<usize>, length: usize, stride: usize) -> Fmt::ColorIter {
        self.0.get_colors(self.1.as_ref(), range, length, stride)
    }*/
}

impl<Fmt, Buf> ColorBufMut for (&Fmt, Buf)
where
    Fmt: ColorSet,
    Buf: AsMut<[u8]> + AsRef<Fmt>,
{
    fn set(&mut self, index: usize, color: Fmt::ColorType) {
        self.0.set_color(self.1.as_mut(), index, color)
    }
}*/
