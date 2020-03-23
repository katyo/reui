use core::ops::{Add, Sub};
use super::{Point, Size};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect<Dim> {
    /// Left top point
    pub point: Point<Dim>,
    /// Right bottom point
    pub size: Size<Dim>,
}

impl<Dim> Rect<Dim> {
    pub const fn new(point: Point<Dim>, size: Size<Dim>) -> Self {
        Self { point, size }
    }

    pub fn size(&self) -> Size<Dim>
    where
        Dim: Copy,
    {
        self.size
    }

    pub fn left(&self) -> Dim
    where
        Dim: Copy,
    {
        self.point.x
    }

    pub fn top(&self) -> Dim
    where
        Dim: Copy,
    {
        self.point.y
    }

    pub fn right(&self) -> Dim
    where
        Dim: Add<Output = Dim> + Copy,
    {
        self.point.x + self.size.w
    }

    pub fn bottom(&self) -> Dim
    where
        Dim: Add<Output = Dim> + Copy,
    {
        self.point.y + self.size.h
    }

    pub fn left_top(&self) -> Point<Dim>
    where
        Dim: Copy,
    {
        self.point
    }

    pub fn left_bottom(&self) -> Point<Dim>
    where
        Dim: Add<Output = Dim> + Copy,
    {
        Point::new(self.point.x, self.bottom())
    }

    pub fn right_top(&self) -> Point<Dim>
    where
        Dim: Add<Output = Dim> + Copy,
    {
        Point::new(self.point.x + self.size.w, self.point.y)
    }

    pub fn right_bottom(&self) -> Point<Dim>
    where
        Dim: Add<Output = Dim> + Copy,
    {
        Point::new(self.point.x + self.size.w, self.point.y + self.size.h)
    }

    pub fn is_collapsed(&self) -> bool
    where
        Dim: Default + PartialEq,
    {
        self.size.is_collapsed()
    }

    pub fn to_local(&self, point: Point<Dim>) -> Point<Dim>
    where
        Dim: Sub<Output = Dim> + Copy,
    {
        Point::new(point.x - self.point.x, point.y - self.point.y)
    }

    pub fn to_global(&self, point: Point<Dim>) -> Point<Dim>
    where
        Dim: Add<Output = Dim> + Copy,
    {
        Point::new(point.x + self.point.x, point.y + self.point.y)
    }
}

impl<Dim> From<(Dim, Dim, Dim, Dim)> for Rect<Dim>
where
    Dim: Sub<Output = Dim> + Copy,
{
    fn from((l, t, r, b): (Dim, Dim, Dim, Dim)) -> Self {
        Self::new(Point::new(l, t), Size::new(r - l, b - t))
    }
}

impl<Dim> From<[Dim; 4]> for Rect<Dim>
where
    Dim: Sub<Output = Dim> + Copy,
{
    fn from(ltrb: [Dim; 4]) -> Self {
        Self::new(Point::new(ltrb[0], ltrb[1]), Size::new(ltrb[2] - ltrb[0], ltrb[3] - ltrb[1]))
    }
}
