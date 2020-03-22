use crate::{Rect, ColorGet, PixelView};

/// Font interface
pub trait Font {
    /// Glyph id type
    type Glyph;

    /// Dimension type
    type Dim;

    /// Pixel format type
    type Fmt: ColorGet;

    /// Get font size in bytes
    fn size(&self) -> usize;

    /// Get the number of glyphs
    fn len(&self) -> Self::Glyph;

    /// Get glyph id for char
    fn glyph(&self, chr: char) -> Option<Self::Glyph>;

    /// Get rectangle for glyph
    fn rect(&self, glyph: Self::Glyph) -> &Rect<Self::Dim>;

    /// Get bitmap for glyph
    fn pixels(&self, glyph: usize) -> PixelView<Self::Fmt, (&Self::Fmt, &[u8])>;
}
