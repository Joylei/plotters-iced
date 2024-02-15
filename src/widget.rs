// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use core::marker::PhantomData;

use iced_widget::{
    canvas::Event,
    core::{
        event,
        mouse::Cursor,
        renderer::Style,
        widget::{tree, Tree},
        Element, Layout, Length, Rectangle, Shell, Size, Widget,
    },
    text::Shaping,
};

use crate::renderer::Renderer;

use super::Chart;

/// Chart container, turns [`Chart`]s to [`Widget`]s
pub struct ChartWidget<'a, Message, Theme, Renderer, C>
where
    C: Chart<Message>,
{
    chart: C,
    width: Length,
    height: Length,
    shaping: Shaping,
    _marker: PhantomData<&'a (Renderer, Theme, Message)>,
}

impl<'a, Message, Theme, Renderer, C> ChartWidget<'a, Message, Theme, Renderer, C>
where
    C: Chart<Message> + 'a,
{
    /// create a new [`ChartWidget`]
    pub fn new(chart: C) -> Self {
        Self {
            chart,
            width: Length::Fill,
            height: Length::Fill,
            shaping: Default::default(),
            _marker: Default::default(),
        }
    }

    /// set width
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// set height
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// set text shaping
    pub fn text_shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = shaping;
        self
    }
}

impl<'a, Message, Theme, Renderer, C> Widget<Message, Theme, Renderer>
    for ChartWidget<'a, Message, Theme, Renderer, C>
where
    C: Chart<Message>,
    Renderer: self::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn tag(&self) -> tree::Tag {
        struct Tag<T>(T);
        tree::Tag::of::<Tag<C::State>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(C::State::default())
    }

    #[inline]
    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &iced_widget::core::layout::Limits,
    ) -> iced_widget::core::layout::Node {
        let size = limits.resolve(self.width, self.height, Size::ZERO);
        iced_widget::core::layout::Node::new(size)
    }

    #[inline]
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        _cursor_position: Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<C::State>();
        renderer.draw_chart(state, &self.chart, layout, self.shaping);
    }

    #[inline]
    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: iced_widget::core::Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_widget::core::Clipboard,
        shell: &mut Shell<'_, Message>,
        _rectangle: &Rectangle,
    ) -> event::Status {
        let bounds = layout.bounds();
        let canvas_event = match event {
            iced_widget::core::Event::Mouse(mouse_event) => Some(Event::Mouse(mouse_event)),
            iced_widget::core::Event::Keyboard(keyboard_event) => {
                Some(Event::Keyboard(keyboard_event))
            }
            _ => None,
        };
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
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> iced_widget::core::mouse::Interaction {
        let state = tree.state.downcast_ref::<C::State>();
        let bounds = layout.bounds();
        self.chart.mouse_interaction(state, bounds, cursor)
    }
}

impl<'a, Message, Theme, Renderer, C> From<ChartWidget<'a, Message, Theme, Renderer, C>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    C: Chart<Message> + 'a,
    Renderer: self::Renderer,
{
    fn from(widget: ChartWidget<'a, Message, Theme, Renderer, C>) -> Self {
        Element::new(widget)
    }
}
