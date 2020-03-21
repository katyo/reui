use std::{
    ops::RangeInclusive,
    convert::TryFrom,
    path::Path,
    collections::HashSet,
};
use bdf;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub struct GlyphData {
    pub rect: Rect,
    pub off: usize,
}

pub struct FontData {
    pub codes: Vec<RangeInclusive<u32>>,
    pub glyphs: Vec<GlyphData>,
    pub pixels: Vec<u8>,
}

impl FontData {
    /// Determine distinct rectangle kinds for glyphs
    pub fn rect_kinds(&self) -> Vec<Rect> {
        self.glyphs.iter()
            .map(|glyph| glyph.rect)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Determine dimension range
    ///
    /// The minimum and maximum values of x, y, w and h.
    pub fn dim_range(&self) -> Option<RangeInclusive<i32>> {
        self.glyphs.iter().fold(None, |range, glyph| {
            let rect = &glyph.rect;
            Some(if let Some(range) = range {
                (*range.start()).min(rect.x).min(rect.y).min(rect.w).min(rect.h) ..=
                (*range.end()).max(rect.x).max(rect.y).max(rect.w).max(rect.h)
            } else {
                rect.x.min(rect.y).min(rect.w).min(rect.h) ..=
                rect.x.max(rect.y).max(rect.w).max(rect.h)
            })
        })
    }

    /// Determine maximum offset value
    pub fn max_off(&self) -> Option<usize> {
        self.glyphs.iter().fold(None, |value, glyph| {
            Some(if let Some(value) = value {
                value.max(glyph.off)
            } else {
                glyph.off
            })
        })
    }

    /// Determine maximum char code
    pub fn max_code(&self) -> Option<u32> {
        self.codes.iter().fold(None, |value, range| {
            Some(if let Some(value) = value {
                value.max(*range.end())
            } else {
                *range.end()
            })
        })
    }

    pub fn load(path: &Path, chrs: &[RangeInclusive<char>]) -> Result<Self, String> {
        let font = bdf::open(path).map_err(|err| err.to_string())?;

        let mut chars = font.glyphs().keys().cloned().collect::<Vec<char>>();

        chars.sort();

        let mut codes = Vec::new();
        let mut state = None;

        for chr in &chars {
            if char_in_chars(chr, chrs) {
                match state {
                    Some((init_chr, last_chr)) => {
                        if *chr as u32 > last_chr as u32 + 1 {
                            // push range and open next
                            codes.push(init_chr as u32 ..= last_chr as u32);
                            state = Some((*chr, *chr));
                        } else {
                            // update last char
                            state.as_mut().unwrap().1 = *chr;
                        }
                    },
                    None => { // open char range
                        state = Some((*chr, *chr));
                    },
                }
            }
        }

        match state {
            Some((init_chr, last_chr)) => {
                // push range and open next
                codes.push(init_chr as u32 ..= last_chr as u32);
            },
            None => {},
        }

        let mut glyphs = Vec::new();
        let mut pixels = Vec::new();

        for range in &codes {
            for code in range.clone() {
                let chr = char::try_from(code).unwrap();
                let glyph = &font.glyphs()[&chr];
                let bbox = glyph.bounds();
                let map = glyph.map();

                glyphs.push(GlyphData {
                    rect: Rect {
                        x: bbox.x,
                        y: bbox.y,
                        w: bbox.width as _,
                        h: bbox.height as _,
                    },
                    off: pixels.len(),
                });

                let mut byte = 0;
                let mut bit = 0;

                for y in 0..bbox.height {
                    for x in 0..bbox.width {
                        if map.get(x, y) {
                            byte |= 1 << bit;
                        }
                        bit += 1;
                        if bit == 8 {
                            bit = 0;
                            pixels.push(byte);
                        }
                    }
                }
            }
        }

        Ok(FontData {
            codes,
            glyphs,
            pixels,
        })
    }
}

fn char_in_chars(chr: &char, chars: &[RangeInclusive<char>]) -> bool {
    if chars.len() == 0 {
        return true;
    }
    for range in chars {
        if range.contains(chr) {
            return true;
        }
    }
    false
}
