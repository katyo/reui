use core::marker::PhantomData;
use typenum::{Prod};
use crate::{Point, Size, ColorGet, ColorSet, ColorBuf, ColorBufMut, ColorArray};

pub type PixelArray<W, H, Fmt> = ColorArray<Prod<W, H>, Fmt>;

pub struct PixelView<Fmt, Buf>
where
    Fmt: ColorGet,
    Buf: ColorBuf<Fmt>,
{
    pub data: Buf,
    pub size: Size<usize>,
    pub _phantom: PhantomData<Fmt>,
}

impl<Fmt, Buf> PixelView<Fmt, Buf>
where
    Fmt: ColorGet,
    Buf: ColorBuf<Fmt>,
{
    pub fn new(data: Buf, size: Size<usize>) -> Self {
        Self { data, size, _phantom: PhantomData }
    }

    pub fn wrap(data: Buf) -> Self {
        Self::new(data, Size::default())
    }

    pub fn with_size(mut self, size: Size<usize>) -> Self {
        self.size = size;
        self
    }

    pub fn unwrap(self) -> Buf {
        self.data
    }

    pub fn size(&self) -> Size<usize> {
        self.size
    }

    pub fn get(&self, point: Point<usize>) -> Fmt::ColorType {
        let index = point.x + point.y * self.size.w;

        self.data.get(index)
    }

    pub fn set(&mut self, point: Point<usize>, color: Fmt::ColorType)
    where
        Fmt: ColorSet,
        Buf: ColorBufMut<Fmt>,
    {
        let index = point.x + point.y * self.size.w;

        self.data.set(index, color)
    }
}

#[macro_export]
macro_rules! pixel_view {
    ($name: ident < $width: tt, $height: tt, $fmt: path > : $($data:tt)+ ) => {
        static $name: $crate::PixelView<$fmt, ($fmt, &[u8])> =
            $crate::PixelView {
                data: $($data)+,
                size: $crate::Size { w: $width, h: $height },
                _phantom: core::marker::PhantomData,
            };
    };
}

#[cfg(test)]
mod test {
    use typenum::*;
    use crate::*;

    #[test]
    fn test_image() {
        static IMAGE_DATA: [u8; 4] = [1, 97, 186, 44];

        pixel_view!(IMAGE<2, 2, format::RGB332>: (format::RGB332, &IMAGE_DATA));

        assert_eq!(IMAGE.get(Point::new(0, 0)), RGB::new(0, 0, 85));
        assert_eq!(IMAGE.get(Point::new(1, 0)), RGB::new(108, 0, 85));
        assert_eq!(IMAGE.get(Point::new(0, 1)), RGB::new(180, 216, 170));
        assert_eq!(IMAGE.get(Point::new(1, 1)), RGB::new(36, 108, 0));
    }

    #[test]
    fn test_rgb888() {
        type Format = format::RGB888;
        type Width = U128;
        type Height = U162;

        let mut data = PixelArray::<Width, Height, Format>::new();
        let mut view = PixelView::<Format, _>::new(&mut data, Size::new(Width::USIZE, Height::USIZE));

        let size = view.size();

        assert_eq!(size, (Width::USIZE, Height::USIZE).into());

        let colors = [
            RGB::new(0, 0, 248),
            RGB::new(0, 252, 72),
            RGB::new(56, 16, 0),
            RGB::new(136, 192, 88),
            RGB::new(164, 132, 200),
            RGB::new(20, 208, 24),
        ];

        let mut c = 0;

        for _ in 0..11 {
            for x in 0..size.w {
                for y in 0..size.h {
                    let coord = (x, y).into();
                    let color = colors[c];
                    view.set(coord, color);
                    assert_eq!(view.get(coord), color);
                    c += 1;
                    if c == colors.len() {
                        c = 0;
                    }
                }
            }
        }
    }

    #[test]
    fn test_rgb565() {
        type Format = format::RGB565;
        type Width = U128;
        type Height = U162;

        let mut data = PixelArray::<Width, Height, Format>::new();
        let mut view = PixelView::<Format, _>::new(&mut data, Size::new(Width::USIZE, Height::USIZE));

        let size = view.size();

        assert_eq!(size, (Width::USIZE, Height::USIZE).into());

        let colors = [
            RGB::new(0, 0, 248),
            RGB::new(0, 252, 72),
            RGB::new(56, 16, 0),
            RGB::new(136, 192, 88),
            RGB::new(160, 132, 200),
            RGB::new(24, 208, 24),
        ];

        let mut c = 0;

        for _ in 0..11 {
            for x in 0..size.w {
                for y in 0..size.h {
                    let coord = (x, y).into();
                    let color = colors[c];
                    view.set(coord, color);
                    assert_eq!(view.get(coord), color);
                    c += 1;
                    if c == colors.len() {
                        c = 0;
                    }
                }
            }
        }
    }
}
