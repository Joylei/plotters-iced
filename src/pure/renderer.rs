// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use crate::backend::IcedChartBackend;

use super::Chart;
use iced_graphics::{self, backend, renderer::Style, Backend, Primitive, Vector};
use iced_native::{Font, Layout, Point, Rectangle};
use plotters::prelude::DrawingArea;
use plotters_backend::{FontFamily, FontStyle};

pub trait Renderer: iced_native::Renderer + iced_native::text::Renderer {
    fn draw_chart<Message, C>(
        &mut self,
        state: &C::State,
        chart: &C,
        font_resolver: &Box<dyn Fn(FontFamily, FontStyle) -> Font>,
        defaults: &Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) where
        C: Chart<Message>;
}

impl<B: Backend + backend::Text> Renderer for iced_graphics::Renderer<B> {
    fn draw_chart<Message, C>(
        &mut self,
        state: &C::State,
        chart: &C,
        font_resolver: &Box<dyn Fn(FontFamily, FontStyle) -> Font>,
        _style: &Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) where
        C: Chart<Message>,
    {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }

        let geometry = chart.draw(bounds.size(), |frame| {
            let backend = IcedChartBackend::new(frame, self.backend(), font_resolver);
            let root: DrawingArea<_, _> = backend.into();
            chart.draw_chart(state, root);
        });
        let translation = Vector::new(bounds.x, bounds.y);
        self.draw_primitive(Primitive::Translate {
            translation,
            content: Box::new(geometry.into_primitive()),
        });
    }
}
