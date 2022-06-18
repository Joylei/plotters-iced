// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

pub(crate) trait AndExt {
    fn and<F: Fn(Self) -> Self>(self, f: F) -> Self
    where
        Self: Sized;
}

impl<T> AndExt for T {
    #[inline(always)]
    fn and<F: Fn(Self) -> Self>(self, f: F) -> Self
    where
        Self: Sized,
    {
        f(self)
    }
}
