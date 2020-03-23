use core::ops::{
    AddAssign, Add,
    SubAssign, Sub,
};
use super::{Size};

/// The point type
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point<Dim> {
    /// Horizontal coordinate
    pub x: Dim,
    /// Vertical coordinate
    pub y: Dim,
}

impl<Dim> Point<Dim> {
    pub const fn new(x: Dim, y: Dim) -> Self {
        Self { x, y }
    }
}

impl<Dim> From<(Dim, Dim)> for Point<Dim> {
    fn from((x, y): (Dim, Dim)) -> Self {
        Self::new(x, y)
    }
}

impl<Dim> From<[Dim; 2]> for Point<Dim> where Dim: Copy {
    fn from(xy: [Dim; 2]) -> Self {
        Self::new(xy[0], xy[1])
    }
}

impl<Dim> AddAssign<Size<Dim>> for Point<Dim> where Dim: AddAssign {
    fn add_assign(&mut self, other: Size<Dim>) {
        self.x += other.w;
        self.y += other.h;
    }
}

impl<Dim> Add<Size<Dim>> for Point<Dim> where Dim: AddAssign + Copy {
    type Output = Point<Dim>;
    fn add(mut self, other: Size<Dim>) -> Self::Output {
        self += other;
        self
    }
}

impl<Dim> SubAssign<Size<Dim>> for Point<Dim> where Dim: SubAssign {
    fn sub_assign(&mut self, other: Size<Dim>) {
        self.x -= other.w;
        self.y -= other.h;
    }
}

impl<Dim> Sub<Size<Dim>> for Point<Dim> where Dim: SubAssign + Copy {
    type Output = Point<Dim>;
    fn sub(mut self, other: Size<Dim>) -> Self::Output {
        self -= other;
        self
    }
}

impl<Dim> Sub<Point<Dim>> for Point<Dim> where Dim: Sub<Output = Dim> {
    type Output = Size<Dim>;
    fn sub(self, other: Point<Dim>) -> Self::Output {
        Size { w: self.x - other.x, h: self.y - other.y }
    }
}
