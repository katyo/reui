use core::ops::{Add, Mul, Div, Range};
use typenum::{U7, U8, Sum, Prod, Quot};
use generic_array::{GenericArray, ArrayLength};
use super::{ColorFmt, ColorGet, ColorSet, ColorBuf, ColorBufMut};

/// Static-sized buffer for colors
pub struct ColorArray<Len, Fmt>
where
    Fmt: ColorFmt,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    format: Fmt,
    data: GenericArray<u8, Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>>,
}

impl<Len, Fmt> ColorArray<Len, Fmt>
where
    Fmt: ColorFmt,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    /// Create buffer without initializing
    pub fn new() -> Self
    where
        Fmt: Default,
    {
        Self::new_with_format(Fmt::default())
    }

    /// Create buffer without initializing using specified format
    pub fn new_with_format(format: Fmt) -> Self {
        let data = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        Self { format, data }
    }
}

impl<Len, Fmt> Default for ColorArray<Len, Fmt>
where
    Fmt: ColorSet + Default,
    Fmt::ColorType: Default + Copy,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn default() -> Self {
        let mut buffer = Self::new();
        let data = buffer.data.as_mut();
        let c = Fmt::ColorType::default();
        for i in 0..buffer.format.num_colors(data) {
            buffer.format.set_color(data, i, c);
        }
        buffer
    }
}

impl<Len, Fmt> AsRef<[u8]> for ColorArray<Len, Fmt>
where
    Fmt: ColorSet,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl<Len, Fmt> AsMut<[u8]> for ColorArray<Len, Fmt>
where
    Fmt: ColorSet,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn as_mut(&mut self) -> &mut [u8] {
        self.data.as_mut()
    }
}

impl<Len, Fmt> AsRef<Fmt> for ColorArray<Len, Fmt>
where
    Fmt: ColorSet,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn as_ref(&self) -> &Fmt {
        &self.format
    }
}

impl<Len, Fmt> ColorBuf<Fmt> for ColorArray<Len, Fmt>
where
    Fmt: ColorGet,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn len(&self) -> usize {
        self.format.num_colors(self.data.as_ref())
    }

    fn get(&self, index: usize) -> Fmt::ColorType {
        self.format.get_color(self.data.as_ref(), index)
    }

    /*fn gets(&self, range: Range<usize>, length: usize, stride: usize) -> Fmt::ColorIter {
        self.format.get_colors(self.data.as_ref(), range, length, stride)
    }*/
}

impl<Len, Fmt> ColorBufMut<Fmt> for ColorArray<Len, Fmt>
where
    Fmt: ColorSet,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn set(&mut self, index: usize, color: Fmt::ColorType)
    where
        Fmt: ColorSet,
    {
        self.format.set_color(self.data.as_mut(), index, color)
    }
}

impl<'a, Len, Fmt> ColorBuf<Fmt> for &'a ColorArray<Len, Fmt>
where
    Fmt: ColorGet,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn len(&self) -> usize {
        self.format.num_colors(self.data.as_ref())
    }

    fn get(&self, index: usize) -> Fmt::ColorType {
        self.format.get_color(self.data.as_ref(), index)
    }

    /*fn gets(&self, range: Range<usize>, length: usize, stride: usize) -> Fmt::ColorIter {
        self.format.get_colors(self.data.as_ref(), range, length, stride)
    }*/
}

impl<'a, Len, Fmt> ColorBuf<Fmt> for &'a mut ColorArray<Len, Fmt>
where
    Fmt: ColorGet,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn len(&self) -> usize {
        self.format.num_colors(self.data.as_ref())
    }

    fn get(&self, index: usize) -> Fmt::ColorType {
        self.format.get_color(self.data.as_ref(), index)
    }

    /*fn gets(&self, range: Range<usize>, length: usize, stride: usize) -> Fmt::ColorIter {
        self.format.get_colors(self.data.as_ref(), range, length, stride)
    }*/
}

impl<'a, Len, Fmt> ColorBufMut<Fmt> for &'a mut ColorArray<Len, Fmt>
where
    Fmt: ColorSet,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn set(&mut self, index: usize, color: Fmt::ColorType) {
        self.format.set_color(self.data.as_mut(), index, color)
    }
}
