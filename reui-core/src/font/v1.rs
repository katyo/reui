use core::{
    mem::size_of,
    marker::PhantomData,
    ops::{Sub, RangeInclusive},
};
use num_traits::AsPrimitive;
use crate::{Rect, ColorGet, Font, PixelView};

pub struct GlyphData<Dim, Off> {
    rect: Rect<Dim>,
    off: Off,
}

impl<Dim, Off> GlyphData<Dim, Off> {
    pub const fn new(rect: Rect<Dim>, off: Off) -> Self {
        Self { rect, off }
    }
}

pub struct FontV1<Fmt, Dim, Off, Code>
where
    Dim: 'static,
    Off: 'static,
    Code: 'static,
{
    codes: &'static [RangeInclusive<Code>],
    glyphs: &'static [GlyphData<Dim, Off>],
    format: Fmt,
    pixels: &'static [u8],
    _phantom: PhantomData<Fmt>,
}

impl<Fmt, Dim, Off, Code> FontV1<Fmt, Dim, Off, Code> {
    /// Create font
    pub const fn new(format: Fmt,
                     codes: &'static [RangeInclusive<Code>],
                     glyphs: &'static [GlyphData<Dim, Off>],
                     pixels: &'static [u8]) -> Self {
        Self {
            codes,
            glyphs,
            format,
            pixels,
            _phantom: PhantomData,
        }
    }
}

impl<Fmt, Dim, Off, Code> Font for FontV1<Fmt, Dim, Off, Code>
where
    Fmt: ColorGet,
    Dim: AsPrimitive<usize>,
    Off: AsPrimitive<usize>,
    char: AsPrimitive<Code>,
    Code: PartialOrd + Sub<Output = Code> + AsPrimitive<usize> + Copy,
{
    type Glyph = usize;

    type Dim = Dim;

    type Fmt = Fmt;

    fn size(&self) -> usize {
        size_of::<Self>() + size_of::<RangeInclusive<Code>>() * self.codes.len() +
            size_of::<GlyphData<Dim, Off>>() * self.glyphs.len() + size_of::<Fmt>() +
            self.pixels.len()
    }

    fn len(&self) -> usize {
        self.glyphs.len()
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

    fn rect(&self, glyph: usize) -> &Rect<Dim> {
        &self.glyphs[glyph].rect
    }

    fn pixels(&self, glyph: usize) -> PixelView<(&Self::Fmt, &[u8])> {
        let glyph_data = &self.glyphs[glyph];
        let size = glyph_data.rect.size;
        let data = &self.pixels[glyph_data.off.as_()..];
        PixelView::new(size.as_(), (&self.format, data))
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn font() {
        static _FONT: FontV1<format::BW1, i8, u16, u16> = FontV1::new(
            format::BW1,
            &[0..=20, 32..=45],
            &[GlyphData::new(Rect::new(Point::new(0, -1), Size::new(4, 6)), 0)],
            &[0, 1, 2, 3],
        );
    }
}
