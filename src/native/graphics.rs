// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use super::backend::IcedChartBackend;
use super::native::Renderer as ChartRenderer;
use crate::Chart;
use iced_graphics::{
    backend, canvas, canvas::Cursor, renderer::Style, Backend, Primitive, Renderer,
};
use iced_native::{event, Font, Point, Rectangle, Shell, Vector};
use plotters::prelude::DrawingArea;
use plotters_backend::{FontFamily, FontStyle};

/// Chart container, turns [`Chart`]s to [`iced_native::Widget`]s
pub type ChartWidget<Message, C> = super::native::ChartWidget<Message, C>;

impl<B: Backend + backend::Text> ChartRenderer for Renderer<B> {
    #[inline]
    fn draw_chart<Message, C>(
        &mut self,
        chart: &C,
        font_resolver: &Box<dyn Fn(FontFamily, FontStyle) -> Font>,
        _style: &Style,
        layout: iced_native::Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) where
        C: Chart<Message>,
    {
        let bounds = layout.bounds();
        let geometry = chart.draw(bounds.size(), |frame| {
            let backend = IcedChartBackend::new(frame, self.backend(), font_resolver);
            let root: DrawingArea<_, _> = backend.into();
            chart.draw_chart(root);
        });
        let translation = Vector::new(bounds.x, bounds.y);

        self.draw_primitive(Primitive::Translate {
            translation,
            content: Box::new(geometry.into_primitive()),
        })
    }

    #[inline]
    fn on_event<Message, C: Chart<Message>>(
        &self,
        chart: &mut C,
        event: iced_native::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        _clipboard: &mut dyn iced_native::Clipboard,
        messages: &mut Shell<'_, Message>,
    ) -> iced_native::event::Status {
        let bounds = layout.bounds();

        let canvas_event = match event {
            iced_native::Event::Mouse(mouse_event) => Some(canvas::Event::Mouse(mouse_event)),
            iced_native::Event::Keyboard(keyboard_event) => {
                Some(canvas::Event::Keyboard(keyboard_event))
            }
            _ => None,
        };
        if let Some(canvas_event) = canvas_event {
            let cursor = Cursor::Available(cursor_position);
            let (status, message) = chart.update(canvas_event, bounds, cursor);
            if let Some(m) = message {
                messages.publish(m);
            }
            return status;
        }
        event::Status::Ignored
    }
}
