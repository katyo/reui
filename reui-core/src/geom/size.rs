use core::ops::{Mul};
use num_traits::AsPrimitive;

/// The size type
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size<Dim> {
    /// Width
    pub w: Dim,
    /// Height
    pub h: Dim,
}

impl<Dim> Size<Dim> {
    pub const fn new(w: Dim, h: Dim) -> Self {
        Self { w, h }
    }

    pub fn is_collapsed(&self) -> bool
    where
        Dim: PartialEq + Default,
    {
        self.w == Dim::default() || self.h == Dim::default()
    }

    pub fn area(&self) -> Dim
    where
        Dim: Mul<Output = Dim> + Copy,
    {
        self.w * self.h
    }

    pub fn as_<ToDim>(&self) -> Size<ToDim>
    where
        Dim: AsPrimitive<ToDim>,
        ToDim: Copy + 'static,
    {
        Size::new(self.w.as_(), self.h.as_())
    }
}

impl<Dim> From<(Dim, Dim)> for Size<Dim> {
    fn from((w, h): (Dim, Dim)) -> Self {
        Self::new(w, h)
    }
}

impl<Dim> From<[Dim; 2]> for Size<Dim>
where Dim: Copy
{
    fn from(wh: [Dim; 2]) -> Self {
        Self::new(wh[0], wh[1])
    }
}
