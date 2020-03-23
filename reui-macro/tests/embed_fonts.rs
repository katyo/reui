use reui::{embed, Font};

/// Monospace font 4x6 (all glyphs)
#[embed("../fonts/4x6.bdf")]
pub static MONO4X6: Font = ();

/// Monospace font 4x6 (hex glyphs)
#[embed("../fonts/4x6.bdf", '0'..='9', 'A'..='F')]
pub static MONO4X6_HEX: Font = ();

/// Monospace font 4x6 (ASCII + Cyrillic glyphs)
#[embed("../fonts/4x6.bdf", ' ')]
pub static MONO4X6_CYR: Font = (
    '0'..='9', '+', '-', '*', '/', '=',
    'a'..='z', 'A'..='Z', ',', '.', ';', '!', '?', '-', '"',
    'а'..='я', 'А'..='Я',
);

#[test]
fn test_font_4x6() {
    assert_eq!(MONO4X6.size(), 3155);
    assert_eq!(MONO4X6.len(), 919);
    assert_eq!(MONO4X6.glyph('0'), Some(17));
    assert_eq!(MONO4X6.glyph('9'), Some(26));
    assert_eq!(MONO4X6.glyph('a'), Some(66));
    assert_eq!(MONO4X6.glyph('z'), Some(91));
    assert_eq!(MONO4X6.glyph('A'), Some(34));
    assert_eq!(MONO4X6.glyph('Z'), Some(59));
    assert_eq!(MONO4X6.glyph(' '), Some(1));
    assert_eq!(MONO4X6.glyph('.'), Some(15));
}

#[test]
fn test_font_4x6_hex() {
    assert_eq!(MONO4X6_HEX.size(), 98);
    assert_eq!(MONO4X6_HEX.len(), 16);
    assert_eq!(MONO4X6_HEX.glyph('0'), Some(0));
    assert_eq!(MONO4X6_HEX.glyph('9'), Some(9));
    assert_eq!(MONO4X6_HEX.glyph('A'), Some(10));
    assert_eq!(MONO4X6_HEX.glyph('F'), Some(15));
    assert_eq!(MONO4X6_HEX.glyph('G'), None);
    assert_eq!(MONO4X6_HEX.glyph(' '), None);
    assert_eq!(MONO4X6_HEX.glyph('.'), None);
}

#[test]
fn test_font_4x6_cyr() {
    assert_eq!(MONO4X6_CYR.size(), 506);
    assert_eq!(MONO4X6_CYR.len(), 138);
    assert_eq!(MONO4X6_CYR.glyph('0'), Some(9));
    assert_eq!(MONO4X6_CYR.glyph('9'), Some(18));
    assert_eq!(MONO4X6_CYR.glyph('='), Some(20));
    assert_eq!(MONO4X6_CYR.glyph('A'), Some(22));
    assert_eq!(MONO4X6_CYR.glyph('Z'), Some(47));
    assert_eq!(MONO4X6_CYR.glyph('a'), Some(48));
    assert_eq!(MONO4X6_CYR.glyph('z'), Some(73));
    assert_eq!(MONO4X6_CYR.glyph('А'), Some(74));
    assert_eq!(MONO4X6_CYR.glyph('Я'), Some(105));
    assert_eq!(MONO4X6_CYR.glyph('а'), Some(106));
    assert_eq!(MONO4X6_CYR.glyph('я'), Some(137));
    assert_eq!(MONO4X6_CYR.glyph(' '), Some(0));
    assert_eq!(MONO4X6_CYR.glyph('.'), Some(7));
    assert_eq!(MONO4X6_CYR.glyph('"'), Some(2));
    assert_eq!(MONO4X6_CYR.glyph('\''), None);
}
