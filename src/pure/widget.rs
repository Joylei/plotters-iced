// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use super::Chart;
use super::Renderer;
use crate::MouseEventCallback;
use core::marker::PhantomData;
use iced_graphics::{
    canvas::{Cursor, Event},
    renderer::Style,
};
use iced_native::{event, Clipboard, Font, Layout, Length, Point, Rectangle, Shell, Size};
use iced_pure::widget::tree::{self, Tree};
use iced_pure::{Element, Widget};
use plotters_backend::{FontFamily, FontStyle};

/// Chart container, turns [`Chart`]s to [`Widget`]s
pub struct ChartWidget<'a, Message, Renderer, C>
where
    C: Chart<Message>,
{
    chart: C,
    width: Length,
    height: Length,
    font_resolver: Box<dyn Fn(FontFamily, FontStyle) -> Font>,
    on_mouse_event: Option<MouseEventCallback<Message>>,
    _marker: PhantomData<&'a (Renderer, Message)>,
}

impl<'a, Message, Renderer, C> ChartWidget<'a, Message, Renderer, C>
where
    C: Chart<Message> + 'a,
{
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

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn resolve_font(
        mut self,
        resolver: impl Fn(FontFamily, FontStyle) -> Font + 'static,
    ) -> Self {
        self.font_resolver = Box::new(resolver);
        self
    }
}

impl<'a, Message, Renderer, C> Widget<Message, Renderer> for ChartWidget<'a, Message, Renderer, C>
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

    fn tag(&self) -> iced_pure::widget::tree::Tag {
        struct Tag<T>(T);
        tree::Tag::of::<Tag<C::State>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(C::State::default())
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
        tree: &Tree,
        renderer: &mut Renderer,
        style: &Style,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<C::State>();
        renderer.draw_chart(
            state,
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
        tree: &mut Tree,
        event: iced_native::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let bounds = layout.bounds();
        let canvas_event = match event {
            iced_native::Event::Mouse(mouse_event) => Some(Event::Mouse(mouse_event)),
            iced_native::Event::Keyboard(keyboard_event) => Some(Event::Keyboard(keyboard_event)),
            _ => None,
        };
        let cursor = Cursor::Available(cursor_position);
        if let Some(canvas_event) = canvas_event {
            let state = tree.state.downcast_mut::<C::State>();

            let (event_status, message) = self.chart.update(state, canvas_event, bounds, cursor);

            if let Some(message) = message {
                shell.publish(message);
            }

            return event_status;
        }
        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> iced_native::mouse::Interaction {
        let state = tree.state.downcast_ref::<C::State>();
        let bounds = layout.bounds();
        //let cursor = Cursor::from_window_position(cursor_position);
        let cursor = if cursor_position.x <= 0_f32 || cursor_position.y <= 0_f32 {
            Cursor::Unavailable
        } else {
            Cursor::Available(cursor_position)
        };
        self.chart.mouse_interaction(state, bounds, cursor)
    }
}

impl<'a, Message, Renderer, C> From<ChartWidget<'a, Message, Renderer, C>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    C: Chart<Message> + 'a,
    Renderer: self::Renderer,
{
    fn from(widget: ChartWidget<'a, Message, Renderer, C>) -> Self {
        Element::new(widget)
    }
}
