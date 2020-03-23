/*!

# REUI Rust embedded graphics and user-interface

This crate aims to provide fast hardware accelerated embedded graphics and user-interface.

## Color buffers

Color buffer is an abstraction intended to provide access to packed color data.

There are many different color packing formats which determines how color data stored in frame buffers.
Some of widely used is: RGB332, RGB444, RGB565, RGB888 and etc.

## Pixel views

Pixel view allows access to color buffers as to two-dimensional pixel maps,
which allows simplify and optimize usage of color buffers for rendering graphics.

## Examples

### Readonly image data and pixel view

```
# use reui::{ColorArray, PixelView, format, pixel_view};

static IMAGE_DATA: [u8; 4] = [1, 97, 186, 44];

pixel_view!(IMAGE<2, 2, format::RGB332>: (format::RGB332, &IMAGE_DATA));
```

*/

mod geom;
mod color;
mod pixel;
mod font;
mod draw;

pub use self::geom::*;
pub use self::color::*;
pub use self::pixel::*;
pub use self::font::*;
pub use self::draw::*;

/// Constant default value
pub trait ConstDefault {
    const DEFAULT: Self;
}
