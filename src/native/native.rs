// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use crate::Chart;
use iced_native::{
    event, Clipboard, Element, Font, Layout, Length, Point, Rectangle, Size, Widget,
};
use plotters_backend::{FontFamily, FontStyle};
use std::{hash::Hash, marker::PhantomData};

/// Signature for the callback that ChartWidget can trigger when a mouse event
/// happens inside its layout. Return None if the mouse event is not being
/// handled by this callback.
///
/// # Arguments
///
/// * The type of mouse event
/// * The cursor position during the event, relative to the widget origin. Use
///   the chart coord spec to transform this point into the chart's data coordinates.
pub type MouseEventCallback<Message> = Box<dyn Fn(iced_native::mouse::Event, Point) -> Option<Message>>;

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
    pub fn on_mouse_event(
        mut self,
        callback: MouseEventCallback<Message>)
    -> Self {
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
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw_chart(
            &self.chart,
            &self.font_resolver,
            defaults,
            layout,
            cursor_position,
            viewport,
        )
    }

    #[inline]
    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {

        if let iced_native::Event::Mouse(mouse_event) = &event {
            if let Some(callback) = &self.on_mouse_event {
                let bounds = layout.bounds();
                if bounds.contains(cursor_position) {
                    let p_origin = bounds.position();
                    let p = cursor_position - p_origin;
                    if let Some(message) = callback(*mouse_event, Point::new(p.x, p.y)) {
                        messages.push(message);
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
            messages,
        )
    }

    #[inline]
    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

pub trait Renderer: iced_native::Renderer + iced_native::text::Renderer {
    fn draw_chart<Message, C>(
        &self,
        chart: &C,
        font_resolver: &Box<dyn Fn(FontFamily, FontStyle) -> Font>,
        defaults: &Self::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Self::Output
    where
        C: Chart<Message>;

    fn on_event<Message, C: Chart<Message>>(
        &self,
        chart: &mut C,
        event: iced_native::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status;
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
