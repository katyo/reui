use core::{
    mem::size_of,
    marker::PhantomData,
    ops::{Sub, Mul, RangeInclusive},
};
use typenum::Unsigned;
use num_traits::AsPrimitive;
use crate::{Rect, ColorGet, Font, PixelView};

pub struct FontV2<Fmt, Dim, Code>
where
    Dim: 'static,
    Code: 'static,
{
    codes: &'static [RangeInclusive<Code>],
    rect: Rect<Dim>,
    format: Fmt,
    pixels: &'static [u8],
    _phantom: PhantomData<Fmt>,
}

impl<Fmt, Dim, Code> FontV2<Fmt, Dim, Code> {
    /// Create font
    pub const fn new(format: Fmt,
                     codes: &'static [RangeInclusive<Code>],
                     rect: Rect<Dim>,
                     pixels: &'static [u8]) -> Self {
        Self {
            codes,
            rect,
            format,
            pixels,
            _phantom: PhantomData,
        }
    }
}

impl<Fmt, Dim, Code> Font for FontV2<Fmt, Dim, Code>
where
    Fmt: ColorGet,
    Dim: Mul<Output = Dim> + AsPrimitive<usize> + Copy,
    char: AsPrimitive<Code>,
    Code: PartialOrd + Sub<Output = Code> + AsPrimitive<usize> + Copy,
{
    type Glyph = usize;

    type Dim = Dim;

    type Fmt = Fmt;

    fn size(&self) -> usize {
        size_of::<Self>() + size_of::<RangeInclusive<Code>>() * self.codes.len() +
            size_of::<Rect<Dim>>() + size_of::<Fmt>() + self.pixels.len()
    }

    fn len(&self) -> usize {
        self.codes.iter().fold(0, |glyphs, range| glyphs + (*range.end() - *range.start()).as_() + 1)
    }

    fn glyph(&self, chr: char) -> Option<usize> {
        let code = chr.as_();
        let mut num = 0;
        for range in self.codes {
            if range.contains(&code) {
                return Some(num + (code - *range.start()).as_());
            }
            num += (*range.end() - *range.start()).as_() + 1;
        }
        None
    }

    fn rect(&self, _glyph: usize) -> &Rect<Dim> {
        &self.rect
    }

    fn pixels(&self, glyph: usize) -> PixelView<(&Fmt, &[u8])> {
        let size = self.rect.size;
        let off = (size.area().as_() * Fmt::ColorBits::USIZE + 7) / 8;
        let data = &self.pixels[off * glyph ..];
        PixelView::new(size.as_(), (&self.format, data))
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn font() {
        static _FONT: FontV2<format::BW1, i8, u16> = FontV2::new(
            format::BW1,
            &[0..=20, 32..=45],
            Rect::new(Point::new(0, -1), Size::new(4, 6)),
            &[0, 1, 2, 3],
        );
    }
}
