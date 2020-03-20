use super::{Point, Area};

#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    Overwrite,
}

impl Default for DrawMode {
    fn default() -> Self {
        Self::Overwrite
    }
}

pub trait Drawing<Dim, Pix> {
    fn draw_pixel(&mut self, mode: DrawMode, point: Point<Dim>, pixel: Pix);
}

pub trait Drawable<Dim, Pix> {
    //fn area(&self) -> Area<Dim>;
    fn draw(&self, area: Area<Dim>, ctx: &mut dyn Drawing<Dim, Pix>);
}
