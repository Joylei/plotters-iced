// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use crate::{renderer::Renderer, Chart, MouseEventCallback};
use core::marker::PhantomData;
use iced_graphics::renderer::Style;
use iced_native::{event, Clipboard, Font, Layout, Length, Point, Rectangle, Shell, Size};
use iced_pure::widget::tree::Tree;
use iced_pure::{Element, Widget};
use plotters_backend::{FontFamily, FontStyle};

/// Chart container, turns [`Chart`]s to [`Widget`]s
pub struct ChartWidget<Message, C>
where
    C: Chart<Message>,
{
    chart: C,
    width: Length,
    height: Length,
    font_resolver: Box<dyn Fn(FontFamily, FontStyle) -> Font>,
    on_mouse_event: Option<MouseEventCallback<Message>>,
    _marker: PhantomData<Message>,
}

impl<'a, Message, C> ChartWidget<Message, C>
where
    C: Chart<Message> + 'a,
{
    #[inline(always)]
    pub fn new(chart: C) -> Self {
        Self {
            chart,
            width: Length::Fill,
            height: Length::Fill,
            font_resolver: Box::new(|_, _| Default::default()),
            on_mouse_event: None,
            _marker: Default::default(),
        }
    }

    #[inline(always)]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    #[inline(always)]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    #[inline(always)]
    pub fn resolve_font(
        mut self,
        resolver: impl Fn(FontFamily, FontStyle) -> Font + 'static,
    ) -> Self {
        self.font_resolver = Box::new(resolver);
        self
    }

    #[inline(always)]
    pub fn on_mouse_event(mut self, callback: MouseEventCallback<Message>) -> Self {
        self.on_mouse_event = Some(callback);
        self
    }
}

impl<'a, Message, Renderer, C> Widget<Message, Renderer> for ChartWidget<Message, C>
where
    C: Chart<Message>,
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    #[inline]
    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let size = limits
            .width(self.width)
            .height(self.height)
            .resolve(Size::ZERO);
        iced_native::layout::Node::new(size)
    }

    #[inline]
    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        style: &Style,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        renderer.draw_chart(
            &self.chart,
            &self.font_resolver,
            style,
            layout,
            cursor_position,
            viewport,
        )
    }

    #[inline]
    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: iced_native::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        if let iced_native::Event::Mouse(mouse_event) = &event {
            if let Some(callback) = &self.on_mouse_event {
                let bounds = layout.bounds();
                if bounds.contains(cursor_position) {
                    let p_origin = bounds.position();
                    let p = cursor_position - p_origin;
                    if let Some(message) = callback(*mouse_event, Point::new(p.x, p.y)) {
                        shell.publish(message);
                        return event::Status::Captured;
                    }
                }
            }
        }

        renderer.on_event(
            &mut self.chart,
            event,
            layout,
            cursor_position,
            clipboard,
            shell,
        )
    }
}

impl<'a, Message, Renderer, C> From<ChartWidget<Message, C>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    C: Chart<Message> + 'a,
    Renderer: self::Renderer,
{
    fn from(widget: ChartWidget<Message, C>) -> Self {
        Element::new(widget)
    }
}
