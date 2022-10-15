use crate::Chart;
use iced_graphics::renderer::Style;
use iced_native::{event, Clipboard, Font, Layout, Point, Rectangle, Shell};
use plotters_backend::{FontFamily, FontStyle};

pub trait Renderer: iced_native::Renderer + iced_native::text::Renderer {
    fn draw_chart<Message, C, F>(
        &mut self,
        chart: &C,
        font_resolver: &F,
        defaults: &Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) where
        C: Chart<Message>,
        F: Fn(FontFamily, FontStyle) -> Font;

    fn on_event<Message, C: Chart<Message>>(
        &self,
        chart: &mut C,
        event: iced_native::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status;
}
