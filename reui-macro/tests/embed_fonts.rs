use reui::embed_font;

embed_font!(MONO4X6, "../fonts/4x6.bdf");
embed_font!(MONO4X6_HEX, "../fonts/4x6.bdf", '0'..='9', 'A'..='F');

#[test]
fn test_font_4x6() {
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
    assert_eq!(MONO4X6_HEX.len(), 16);
    assert_eq!(MONO4X6_HEX.glyph('0'), Some(0));
    assert_eq!(MONO4X6_HEX.glyph('9'), Some(9));
    assert_eq!(MONO4X6_HEX.glyph('A'), Some(10));
    assert_eq!(MONO4X6_HEX.glyph('F'), Some(15));
    assert_eq!(MONO4X6_HEX.glyph('G'), None);
    assert_eq!(MONO4X6_HEX.glyph(' '), None);
    assert_eq!(MONO4X6_HEX.glyph('.'), None);
}
