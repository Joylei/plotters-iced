// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use crate::backend::IcedChartBackend;
use crate::Chart;
use iced_graphics::{self, backend, Backend, Primitive, Vector};
use iced_native::{Font, Layout, Theme};
use plotters::prelude::DrawingArea;
use plotters_backend::{FontFamily, FontStyle};

pub trait Renderer: iced_native::Renderer + iced_native::text::Renderer {
    fn draw_chart<Message, C, F>(
        &mut self,
        state: &C::State,
        chart: &C,
        font_resolver: &F,
        layout: Layout<'_>,
    ) where
        C: Chart<Message>,
        F: Fn(FontFamily, FontStyle) -> Font;
}

impl<B: Backend + backend::Text> Renderer for iced_graphics::Renderer<B, Theme> {
    fn draw_chart<Message, C, F>(
        &mut self,
        state: &C::State,
        chart: &C,
        font_resolver: &F,
        layout: Layout<'_>,
    ) where
        C: Chart<Message>,
        F: Fn(FontFamily, FontStyle) -> Font,
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
