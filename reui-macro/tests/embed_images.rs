use reui::{embed_image, Size, Point, GS, GSA, RGB, RGBA};

#[embed_image("../images/gs_2x2.png")]
pub static GS_2X2: Image = ();

#[embed_image("../images/gsa_2x2.png")]
pub static GSA_2X2: Image = ();

#[embed_image("../images/rgb_2x2.png")]
pub static RGB_2X2: Image = ();

#[embed_image("../images/rgba_2x2.png")]
pub static RGBA_2X2: Image = ();

#[embed_image("../images/idx_2x2.png")]
pub static IDX_2X2: Image = ();

#[embed_image("../images/idxa_2x2.png")]
pub static IDXA_2X2: Image = ();

#[test]
fn test_gs_2x2() {
    assert_eq!(GS_2X2.size(), Size::new(2, 2));
    assert_eq!(GS_2X2.get(Point::new(0, 0)), GS::new(0));
    assert_eq!(GS_2X2.get(Point::new(1, 0)), GS::new(130));
    assert_eq!(GS_2X2.get(Point::new(0, 1)), GS::new(70));
    assert_eq!(GS_2X2.get(Point::new(1, 1)), GS::new(255));
}

#[test]
fn test_gsa_2x2() {
    assert_eq!(GSA_2X2.size(), Size::new(2, 2));
    assert_eq!(GSA_2X2.get(Point::new(0, 0)), GSA::new(0, 255));
    assert_eq!(GSA_2X2.get(Point::new(1, 0)), GSA::new(119, 0));
    assert_eq!(GSA_2X2.get(Point::new(0, 1)), GSA::new(70, 128));
    assert_eq!(GSA_2X2.get(Point::new(1, 1)), GSA::new(255, 255));
}

#[test]
fn test_rgb_2x2() {
    assert_eq!(RGB_2X2.size(), Size::new(2, 2));
    assert_eq!(RGB_2X2.get(Point::new(0, 0)), RGB::new(0, 0, 0));
    assert_eq!(RGB_2X2.get(Point::new(1, 0)), RGB::new(255, 0, 0));
    assert_eq!(RGB_2X2.get(Point::new(0, 1)), RGB::new(0, 0, 255));
    assert_eq!(RGB_2X2.get(Point::new(1, 1)), RGB::new(255, 255, 255));
}

#[test]
fn test_rgba_2x2() {
    assert_eq!(RGBA_2X2.size(), Size::new(2, 2));
    assert_eq!(RGBA_2X2.get(Point::new(0, 0)), RGBA::new(0, 0, 0, 255));
    assert_eq!(RGBA_2X2.get(Point::new(1, 0)), RGBA::new(255, 0, 0, 1));
    assert_eq!(RGBA_2X2.get(Point::new(0, 1)), RGBA::new(0, 0, 255, 128));
    assert_eq!(RGBA_2X2.get(Point::new(1, 1)), RGBA::new(255, 255, 255, 255));
}

#[test]
fn test_idx_2x2() {
    assert_eq!(IDX_2X2.size(), Size::new(2, 2));
    assert_eq!(IDX_2X2.get(Point::new(0, 0)), RGBA::new(0, 0, 0, 255));
    assert_eq!(IDX_2X2.get(Point::new(1, 0)), RGBA::new(255, 0, 0, 255));
    assert_eq!(IDX_2X2.get(Point::new(0, 1)), RGBA::new(0, 0, 255, 255));
    assert_eq!(IDX_2X2.get(Point::new(1, 1)), RGBA::new(255, 255, 255, 255));
}

#[test]
fn test_idxa_2x2() {
    println!("{:?}", IDXA_2X2);
    assert_eq!(IDXA_2X2.size(), Size::new(2, 2));
    assert_eq!(IDXA_2X2.get(Point::new(0, 0)), RGBA::new(0, 0, 0, 255));
    assert_eq!(IDXA_2X2.get(Point::new(1, 0)), RGBA::new(13, 123, 0, 0));
    assert_eq!(IDXA_2X2.get(Point::new(0, 1)), RGBA::new(0, 0, 255, 255));
    assert_eq!(IDXA_2X2.get(Point::new(1, 1)), RGBA::new(255, 255, 255, 255));
}
