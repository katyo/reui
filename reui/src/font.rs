use core::marker::PhantomData;
use core::ops::{Sub, RangeInclusive};
use num_traits::AsPrimitive;
use crate::{Rect, ColorGet, PixelView};

pub struct GlyphData<Dim, Off> {
    rect: Rect<Dim>,
    off: Off,
}

impl<Dim, Off> GlyphData<Dim, Off> {
    pub const fn new(rect: Rect<Dim>, off: Off) -> Self {
        Self { rect, off }
    }
}

pub struct Font<Fmt, Dim, Off, Code>
where
    Dim: 'static,
    Off: 'static,
    Code: 'static,
{
    codes: &'static [RangeInclusive<Code>],
    glyphs: &'static [GlyphData<Dim, Off>],
    pixels: &'static [u8],
    _phantom: PhantomData<Fmt>,
}

impl<Fmt, Dim, Off, Code> Font<Fmt, Dim, Off, Code> {
    /// Create font
    pub const fn new(codes: &'static [RangeInclusive<Code>],
                     glyphs: &'static [GlyphData<Dim, Off>],
                     pixels: &'static [u8]) -> Self {
        Self {
            codes,
            glyphs,
            pixels,
            _phantom: PhantomData,
        }
    }

    /// Get the number of glyphs
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Get glyph id for char
    pub fn glyph(&self, chr: char) -> Option<usize>
    where
        char: AsPrimitive<Code>,
        Code: PartialOrd + Sub<Output = Code> + AsPrimitive<usize> + Copy,
    {
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

    /// Get rectangle for glyph
    pub fn rect(&self, glyph: usize) -> &Rect<Dim> {
        &self.glyphs[glyph].rect
    }

    /// Get bitmap for glyph
    pub fn pixels(&self, glyph: usize) -> PixelView<Fmt, (Fmt, &[u8])>
    where
        Fmt: ColorGet + Default,
        Dim: AsPrimitive<usize>,
        Off: AsPrimitive<usize>,
    {
        let glyph_data = &self.glyphs[glyph];
        let size = glyph_data.rect.size;
        let data = &self.pixels[glyph_data.off.as_()..];
        PixelView::new((Fmt::default(), data), size.as_())
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn font() {
        static _FONT: Font<format::BW1, i8, u16, u16> = Font::new(
            &[0..=20, 32..=45],
            &[GlyphData::new(Rect::new(Point::new(0, -1), Size::new(4, 6)), 0)],
            &[0, 1, 2, 3],
        );
    }
}
