// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use iced_graphics::canvas;
use plotters_backend::{BackendColor, BackendCoord, BackendStyle};

pub(crate) fn cvt_color(color: &BackendColor) -> iced_graphics::Color {
    let ((r, g, b), a) = (color.rgb, color.alpha);
    iced_graphics::Color::from_rgba8(r, g, b, a as f32)
}

pub(crate) fn cvt_stroke<S: BackendStyle>(style: &S) -> canvas::Stroke {
    canvas::Stroke::default()
        .with_color(cvt_color(&style.color()))
        .with_width(style.stroke_width() as f32)
}

pub(crate) trait CvtPoint {
    fn cvt_point(self) -> iced_graphics::Point;
}

impl CvtPoint for BackendCoord {
    fn cvt_point(self) -> iced_graphics::Point {
        iced_graphics::Point::new(self.0 as f32, self.1 as f32)
    }
}

impl CvtPoint for [f64; 2] {
    fn cvt_point(self) -> iced_graphics::Point {
        iced_graphics::Point::new(self[0] as f32, self[1] as f32)
    }
}
