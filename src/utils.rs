// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

#[cfg(target_arch = "wasm32")]
use plotters_backend::FontTransform;

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

#[cfg(target_arch = "wasm32")]
pub(crate) trait RotateAngle {
    fn angle(&self) -> Option<f32>;
}

#[cfg(target_arch = "wasm32")]
impl RotateAngle for FontTransform {
    #[inline]
    fn angle(&self) -> Option<f32> {
        match self {
            FontTransform::Rotate180 => Some(180.0),
            FontTransform::Rotate270 => Some(270.0),
            FontTransform::Rotate90 => Some(90.0),
            //&FontTransform::RotateAngle(v) => Some(v),
            _ => None,
        }
    }
}
