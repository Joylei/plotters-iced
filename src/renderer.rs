// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use iced_widget::{
    canvas::{Cache, Frame},
    core::{Layout, Size, Vector},
    renderer::Geometry,
};
use plotters::prelude::DrawingArea;

use crate::backend::IcedChartBackend;
use crate::Chart;

/// Graphics Renderer
pub trait Renderer:
    iced_widget::core::Renderer + iced_widget::core::text::Renderer + iced_graphics::geometry::Renderer
{
    /// draw a [Chart]
    fn draw_chart<Message, C>(&mut self, state: &C::State, chart: &C, layout: Layout<'_>)
    where
        C: Chart<Message>;
}

impl<Theme> crate::chart::Renderer for iced_widget::renderer::Renderer<Theme> {
    fn draw<F: Fn(&mut Frame)>(&self, size: Size, f: F) -> Geometry {
        let mut frame = Frame::new(self, size);
        f(&mut frame);
        frame.into_geometry()
    }

    fn draw_cache<F: Fn(&mut Frame)>(&self, cache: &Cache, size: Size, f: F) -> Geometry {
        cache.draw(self, size, f)
    }
}

impl<Theme> Renderer for iced_widget::renderer::Renderer<Theme> {
    fn draw_chart<Message, C>(&mut self, state: &C::State, chart: &C, layout: Layout<'_>)
    where
        C: Chart<Message>,
    {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }
        let geometry = chart.draw(self, bounds.size(), |frame| {
            let backend = IcedChartBackend::new(frame, self);
            let root: DrawingArea<_, _> = backend.into();
            chart.draw_chart(state, root);
        });
        let translation = Vector::new(bounds.x, bounds.y);
        iced_widget::core::Renderer::with_translation(self, translation, |renderer| {
            iced_graphics::geometry::Renderer::draw(renderer, vec![geometry]);
        });
    }
}
