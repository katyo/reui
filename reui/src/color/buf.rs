use core::ops::{Add, Mul, Div, AddAssign};
use typenum::{U7, U8, Sum, Prod, Quot};
use generic_array::{GenericArray, ArrayLength};
use crate::{Stepable, ColorFmt, ColorGet, ColorSet, ColorBuf, ColorBufMut};

/// Static-sized buffer for colors
pub struct ColorArray<Len, Fmt, Dim>
where
    Fmt: ColorFmt<Dim>,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    format: Fmt,
    data: GenericArray<u8, Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>>,
}

impl<Len, Fmt, Dim> ColorArray<Len, Fmt, Dim>
where
    Fmt: ColorFmt<Dim>,
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

impl<Len, Fmt, Dim> Default for ColorArray<Len, Fmt, Dim>
where
    Dim: Default + AddAssign + PartialOrd + Stepable + Copy,
    Fmt: ColorSet<Dim> + Default,
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
        for i in Stepable::forward_iter(Dim::default()..buffer.format.num_colors(data)) {
            buffer.format.set_color(data, i, c);
        }
        buffer
    }
}

impl<Len, Fmt, Dim> AsRef<[u8]> for ColorArray<Len, Fmt, Dim>
where
    Dim: Default + AddAssign + PartialOrd + Stepable + Copy,
    Fmt: ColorSet<Dim>,
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

impl<Len, Fmt, Dim> AsMut<[u8]> for ColorArray<Len, Fmt, Dim>
where
    Dim: Default + AddAssign + PartialOrd + Stepable + Copy,
    Fmt: ColorSet<Dim>,
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

impl<Len, Fmt, Dim> AsRef<Fmt> for ColorArray<Len, Fmt, Dim>
where
    Dim: Default + AddAssign + PartialOrd + Stepable + Copy,
    Fmt: ColorSet<Dim>,
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

impl<Len, Fmt, Dim> ColorBuf<Fmt, Dim> for ColorArray<Len, Fmt, Dim>
where
    Dim: Default + AddAssign + PartialOrd + Stepable + Copy,
    Fmt: ColorGet<Dim>,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn len(&self) -> Dim {
        self.format.num_colors(self.data.as_ref())
    }

    fn get(&self, index: Dim) -> Fmt::ColorType
    where
        Fmt: ColorGet<Dim>,
    {
        self.format.get_color(self.data.as_ref(), index)
    }
}

impl<Len, Fmt, Dim> ColorBufMut<Fmt, Dim> for ColorArray<Len, Fmt, Dim>
where
    Dim: Default + AddAssign + PartialOrd + Stepable + Copy,
    Fmt: ColorSet<Dim>,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn set(&mut self, index: Dim, color: Fmt::ColorType)
    where
        Fmt: ColorSet<Dim>,
    {
        self.format.set_color(self.data.as_mut(), index, color)
    }
}

impl<'a, Len, Fmt, Dim> ColorBuf<Fmt, Dim> for &'a ColorArray<Len, Fmt, Dim>
where
    Dim: Default + AddAssign + PartialOrd + Stepable + Copy,
    Fmt: ColorGet<Dim>,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn len(&self) -> Dim {
        self.format.num_colors(self.data.as_ref())
    }

    fn get(&self, index: Dim) -> Fmt::ColorType
    where
        Fmt: ColorGet<Dim>,
    {
        self.format.get_color(self.data.as_ref(), index)
    }
}

impl<'a, Len, Fmt, Dim> ColorBuf<Fmt, Dim> for &'a mut ColorArray<Len, Fmt, Dim>
where
    Dim: Default + AddAssign + PartialOrd + Stepable + Copy,
    Fmt: ColorGet<Dim>,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn len(&self) -> Dim {
        self.format.num_colors(self.data.as_ref())
    }

    fn get(&self, index: Dim) -> Fmt::ColorType
    where
        Fmt: ColorGet<Dim>,
    {
        self.format.get_color(self.data.as_ref(), index)
    }
}

impl<'a, Len, Fmt, Dim> ColorBufMut<Fmt, Dim> for &'a mut ColorArray<Len, Fmt, Dim>
where
    Dim: Default + AddAssign + PartialOrd + Stepable + Copy,
    Fmt: ColorSet<Dim>,
    Fmt::ColorType: Default,
    Len: Mul<Fmt::ColorBits>,
    Prod<Len, Fmt::ColorBits>: Add<U7>,
    Sum<Prod<Len, Fmt::ColorBits>, U7>: Div<U8>,
    Quot<Sum<Prod<Len, Fmt::ColorBits>, U7>, U8>: ArrayLength<u8>,
{
    fn set(&mut self, index: Dim, color: Fmt::ColorType)
    where
        Fmt: ColorSet<Dim>,
    {
        self.format.set_color(self.data.as_mut(), index, color)
    }
}
