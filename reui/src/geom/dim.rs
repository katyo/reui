use core::ops::{Range, AddAssign};

pub trait Stepable: Sized {
    const STEP: Self;

    fn forward_iter(range: Range<Self>) -> StepForward<Self> {
        StepForward { cur: range.start, end: range.end }
    }
}

pub struct StepForward<T> {
    cur: T,
    end: T,
}

impl<T> Iterator for StepForward<T>
where
    T: Stepable + AddAssign<T> + PartialOrd<T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.cur < self.end {
            let cur = self.cur;
            self.cur += T::STEP;
            Some(cur)
        } else {
            None
        }
    }
}

macro_rules! stepable_impl {
    ($($type: ty;)+) => {
        $(
            impl Stepable for $type {
                const STEP: Self = 1;
            }
        )+
    };
}

stepable_impl! {
    u8; i8;
    u16; i16;
    u32; i32;
    usize; isize;
}

pub trait IsIndex {
    fn from_index(index: usize) -> Self;
    fn into_index(self) -> usize;
}

macro_rules! is_index_impl {
    ($($type: ty;)+) => {
        $(
            impl IsIndex for $type {
                fn from_index(index: usize) -> Self {
                    index as _
                }

                fn into_index(self) -> usize {
                    self as _
                }
            }
        )+
    };
}

is_index_impl! {
    u8; i8;
    u16; i16;
    u32; i32;
    usize; isize;
}
