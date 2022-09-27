// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use iced_graphics::canvas::Cursor;
use iced_native::Point;

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

/// same as Cursor::from_window_position
pub(crate) fn cursor_from_window_position(cursor_position: Point) -> Cursor {
    if cursor_position.x <= 0_f32 || cursor_position.y <= 0_f32 {
        Cursor::Unavailable
    } else {
        Cursor::Available(cursor_position)
    }
}
