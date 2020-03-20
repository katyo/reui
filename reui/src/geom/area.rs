use core::ops::{
    BitAndAssign, BitAnd,
    BitOrAssign, BitOr,
};
use super::{Point, Size};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Area<Dim> {
    /// Left top point
    pub lt: Point<Dim>,
    /// Right bottom point
    pub rb: Point<Dim>,
}

impl<Dim> Area<Dim> {
    pub fn new(lt: Point<Dim>, rb: Point<Dim>) -> Self {
        Self { lt, rb }
    }

    pub fn size(&self) -> Size<Dim> where Dim: core::ops::Sub<Dim, Output = Dim> + Copy {
        self.rb - self.lt
    }

    pub fn is_empty(&self) -> bool where Dim: PartialOrd {
        self.lt.x >= self.rb.x || self.lt.y >= self.rb.y
    }
}

impl<Dim> From<(Dim, Dim, Dim, Dim)> for Area<Dim> {
    fn from((l, t, r, b): (Dim, Dim, Dim, Dim)) -> Self {
        Self::new(Point::new(l, t), Point::new(r, b))
    }
}

impl<Dim> From<[Dim; 4]> for Area<Dim> where Dim: Copy {
    fn from(ltrb: [Dim; 4]) -> Self {
        Self::new(Point::new(ltrb[0], ltrb[1]), Point::new(ltrb[2], ltrb[3]))
    }
}

impl<Dim> BitAndAssign<Area<Dim>> for Area<Dim> where Dim: PartialOrd {
    fn bitand_assign(&mut self, other: Area<Dim>) {
        if self.lt.x < other.lt.x {
            self.lt.x = other.lt.x;
        }
        if self.lt.y < other.lt.y {
            self.lt.y = other.lt.y;
        }
        if self.rb.x > other.rb.x {
            self.rb.x = other.rb.x;
        }
        if self.rb.y > other.rb.y {
            self.rb.y = other.rb.y;
        }
    }
}

impl<Dim> BitAnd<Area<Dim>> for Area<Dim> where Dim: PartialOrd {
    type Output = Area<Dim>;

    fn bitand(mut self, other: Area<Dim>) -> Self::Output {
        self &= other;
        self
    }
}

impl<Dim> BitOrAssign<Area<Dim>> for Area<Dim> where Dim: PartialOrd {
    fn bitor_assign(&mut self, other: Area<Dim>) {
        if self.lt.x > other.lt.x {
            self.lt.x = other.lt.x;
        }
        if self.lt.y > other.lt.y {
            self.lt.y = other.lt.y;
        }
        if self.rb.x < other.rb.x {
            self.rb.x = other.rb.x;
        }
        if self.rb.y < other.rb.y {
            self.rb.y = other.rb.y;
        }
    }
}

impl<Dim> BitOr<Area<Dim>> for Area<Dim> where Dim: PartialOrd {
    type Output = Area<Dim>;

    fn bitor(mut self, other: Area<Dim>) -> Self::Output {
        self |= other;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(Area::from((0, 1, 2, 3)).is_empty(), false);
        assert_eq!(Area::from((2, 1, 0, 3)).is_empty(), true);
        assert_eq!(Area::from((2, 1, 2, 3)).is_empty(), true);
        assert_eq!(Area::from((0, 3, 2, 1)).is_empty(), true);
        assert_eq!(Area::from((0, 1, 2, 1)).is_empty(), true);
    }

    #[test]
    fn and() {
        assert_eq!(Area::from((0, 1, 2, 3)) & Area::from((0, 1, 2, 3)), Area::from((0, 1, 2, 3)));
        assert_eq!(Area::from((0, 1, 2, 3)) & Area::from((1, 2, 2, 3)), Area::from((1, 2, 2, 3)));
        assert_eq!(Area::from((1, 2, 2, 3)) & Area::from((0, 1, 2, 3)), Area::from((1, 2, 2, 3)));
        assert_eq!(Area::from((1, 2, 3, 4)) & Area::from((2, 3, 2, 3)), Area::from((2, 3, 2, 3)));
    }

    #[test]
    fn or() {
        assert_eq!(Area::from((0, 1, 2, 3)) | Area::from((0, 1, 2, 3)), Area::from((0, 1, 2, 3)));
        assert_eq!(Area::from((0, 1, 2, 3)) | Area::from((1, 2, 2, 3)), Area::from((0, 1, 2, 3)));
        assert_eq!(Area::from((1, 2, 2, 3)) | Area::from((0, 1, 2, 3)), Area::from((0, 1, 2, 3)));
        assert_eq!(Area::from((1, 2, 3, 4)) | Area::from((2, 3, 2, 3)), Area::from((1, 2, 3, 4)));
    }
}
