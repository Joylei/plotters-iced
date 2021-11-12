// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

mod triangulate;
mod utils;

use self::utils::{
    converter::{cvt_color, cvt_stroke, CvtPoint},
    path, shape,
};
use crate::error::Error;
use iced_graphics::{
    backend,
    canvas,
    Backend,
    HorizontalAlignment,
    Size,
    //Vector,
    VerticalAlignment,
};
use iced_native::Font;
use plotters_backend::{
    text_anchor,
    BackendColor,
    BackendCoord,
    BackendStyle,
    BackendTextStyle,
    DrawingBackend,
    DrawingErrorKind,
    FontFamily,
    FontStyle,
    //FontTransform,
};

/// The Iced drawing backend
pub(crate) struct IcedChartBackend<'a, B, F>
where
    B: Backend + backend::Text,
    F: Fn(FontFamily, FontStyle) -> Font,
{
    frame: &'a mut canvas::Frame,
    backend: &'a B,
    font_resolver: &'a F,
}

impl<'a, B, F> IcedChartBackend<'a, B, F>
where
    B: Backend + backend::Text,
    F: Fn(FontFamily, FontStyle) -> Font,
{
    #[inline]
    pub fn new(frame: &'a mut canvas::Frame, backend: &'a B, font_resolver: &'a F) -> Self {
        Self {
            frame,
            backend,
            font_resolver,
        }
    }
}

impl<'a, B, F> DrawingBackend for IcedChartBackend<'a, B, F>
where
    B: Backend + backend::Text,
    F: Fn(FontFamily, FontStyle) -> Font,
{
    type ErrorType = Error;

    #[inline]
    fn get_size(&self) -> (u32, u32) {
        let Size { width, height } = self.frame.size();
        (width as u32, height as u32)
    }

    #[inline]
    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    #[inline]
    fn present(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        Ok(())
    }

    #[inline]
    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if color.alpha == 0.0 {
            return Ok(());
        }
        self.frame
            .fill_rectangle(point.cvt_point(), Size::new(1.0, 1.0), cvt_color(&color));
        Ok(())
    }

    #[inline]
    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let line = canvas::Path::line(from.cvt_point(), to.cvt_point());
        self.frame.stroke(&line, cvt_stroke(style));
        Ok(())
    }

    #[inline]
    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let height = (bottom_right.1 - upper_left.1) as f32;
        let width = (bottom_right.0 - upper_left.0) as f32;
        let upper_left = upper_left.cvt_point();
        if fill {
            self.frame.fill_rectangle(
                upper_left,
                Size::new(width, height),
                cvt_color(&style.color()),
            );
        } else {
            let rect = canvas::Path::rectangle(upper_left, Size::new(width, height));
            self.frame.stroke(&rect, cvt_stroke(style));
        }

        Ok(())
    }

    #[inline]
    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let path = canvas::Path::new(move |builder| {
            for (i, point) in path.into_iter().enumerate() {
                if i > 0 {
                    builder.line_to(point.cvt_point());
                } else {
                    builder.move_to(point.cvt_point());
                }
            }
        });

        self.frame.stroke(&path, cvt_stroke(style));
        Ok(())
    }

    #[inline]
    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        let circle = canvas::Path::circle(center.cvt_point(), radius as f32);

        if fill {
            self.frame.fill(&circle, cvt_color(&style.color()));
        } else {
            self.frame.stroke(&circle, cvt_stroke(style));
        }

        Ok(())
    }

    #[inline]
    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        // Paint a simplified path, where empty areas are removed and un-necessary points are \
        //   cleared. This is required for triangulation to work properly, and it reduces \
        //   the number of triangles on screen to a strict minimum.
        let simplified_path: Vec<_> =
            path::PathSimplifier::from(vert.into_iter().map(|(x, y)| [x, y])).collect();

        // Find closed shapes (eg. when the plot area goes from positive to negative, we need \
        //   to split the path into two distinct paths, otherwise we will not be able to \
        //   triangulate properly, and thus we will not be able to fill the shape)
        if let Ok(mut shape_splitter) = shape::ShapeSplitter::try_from(&simplified_path) {
            // Triangulate the polygon points, giving back a list of triangles that can be \
            //   filled into a contiguous area.
            // Notice: this method takes into account concave shapes

            let path = canvas::Path::new(move |builder| {
                for shape_points in shape_splitter.collect() {
                    // Is that enough points to form at least a triangle?
                    if shape_points.len() < 3 {
                        continue;
                    }
                    let triangles = triangulate::triangulate_points(shape_points.iter());
                    for index in 0..triangles.size() {
                        let shape = triangles.get_triangle(index);
                        let points = shape.points.iter().copied();
                        for (i, point) in points.into_iter().enumerate() {
                            if i > 0 {
                                builder.line_to(point.cvt_point());
                            } else {
                                builder.move_to(point.cvt_point());
                            }
                        }
                        builder.close();
                    }
                }
            });
            self.frame.fill(&path, cvt_color(&style.color()));
        }
        Ok(())
    }

    #[inline]
    fn draw_text<S: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &S,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let horizontal_alignment = match style.anchor().h_pos {
            text_anchor::HPos::Left => HorizontalAlignment::Left,
            text_anchor::HPos::Right => HorizontalAlignment::Right,
            text_anchor::HPos::Center => HorizontalAlignment::Center,
        };
        let vertical_alignment = match style.anchor().v_pos {
            text_anchor::VPos::Top => VerticalAlignment::Top,
            text_anchor::VPos::Center => VerticalAlignment::Center,
            text_anchor::VPos::Bottom => VerticalAlignment::Bottom,
        };
        let font = (self.font_resolver)(style.family(), style.style());
        let pos = pos.cvt_point();

        //let (w, h) = self.estimate_text_size(text, style)?;
        let text = canvas::Text {
            content: text.to_owned(),
            position: pos,
            color: cvt_color(&style.color()),
            size: style.size() as f32,
            font,
            horizontal_alignment,
            vertical_alignment,
        };
        //TODO: fix rotation util text rotation is supported by Iced
        // let rotate = match style.transform() {
        //     FontTransform::None => None,
        //     FontTransform::Rotate90 => Some(90.0),
        //     FontTransform::Rotate180 => Some(180.0),
        //     FontTransform::Rotate270 => Some(270.0),
        //     FontTransform::RotateAngle(angle) => Some(angle),
        // };
        // if let Some(rotate) = rotate {
        //     dbg!(rotate);
        //     self.frame.with_save(move |frame| {
        //         frame.fill_text(text);
        //         frame.translate(Vector::new(pos.x + w as f32 / 2.0, pos.y + h as f32 / 2.0));
        //         let angle = 2.0 * std::f32::consts::PI * rotate / 360.0;
        //         frame.rotate(angle);
        //     });
        // } else {
        //     self.frame.fill_text(text);
        // }
        self.frame.fill_text(text);

        Ok(())
    }

    #[inline]
    fn estimate_text_size<S: BackendTextStyle>(
        &self,
        text: &str,
        style: &S,
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        let font = (self.font_resolver)(style.family(), style.style());
        let bounds = self.frame.size();
        let size = self
            .backend
            .measure(text, style.size() as f32, font, bounds);
        Ok((size.0 as u32, size.1 as u32))
    }

    #[inline]
    fn blit_bitmap(
        &mut self,
        _pos: BackendCoord,
        (_iw, _ih): (u32, u32),
        _src: &[u8],
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Not supported yet (rendering ignored)
        // Notice: currently Iced has limitations, because widgets are not rendered in the order of creation, and different primitives go to different render pipelines.

        Ok(())
    }
}
