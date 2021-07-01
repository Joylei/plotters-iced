use crate::Chart;
use iced_native::{
    event, Clipboard, Element, Font, Layout, Length, Point, Rectangle, Size, Widget,
};
use plotters_backend::{FontFamily, FontStyle};
use std::{hash::Hash, marker::PhantomData};

/// Chart container, turns [`Chart`]s to [`Widget`]s
pub struct ChartWidget<'a, Message, C>
where
    C: Chart<Message>,
{
    chart: &'a mut C,
    width: Length,
    height: Length,
    font_resolver: Box<dyn Fn(FontFamily, FontStyle) -> Font>,
    _marker: PhantomData<Message>,
}

impl<'a, Message, C> ChartWidget<'a, Message, C>
where
    C: Chart<Message>,
{
    #[inline]
    pub fn new(chart: &'a mut C) -> Self {
        Self {
            chart,
            width: Length::Fill,
            height: Length::Fill,
            font_resolver: Box::new(|_, _| Default::default()),
            _marker: Default::default(),
        }
    }

    #[inline]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    #[inline]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    #[inline]
    pub fn resolve_font(
        mut self,
        resolver: impl Fn(FontFamily, FontStyle) -> Font + 'static,
    ) -> Self {
        self.font_resolver = Box::new(resolver);
        self
    }
}

impl<'a, Message, Renderer, C> Widget<Message, Renderer> for ChartWidget<'a, Message, C>
where
    C: Chart<Message>,
    Renderer: self::Renderer,
{
    #[inline]
    fn width(&self) -> Length {
        self.width
    }

    #[inline]
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
            self.chart,
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
        renderer.on_event(
            self.chart,
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

impl<'a, Message, Renderer, C> From<ChartWidget<'a, Message, C>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    C: Chart<Message>,
    Renderer: self::Renderer,
{
    #[inline]
    fn from(widget: ChartWidget<'a, Message, C>) -> Self {
        Element::new(widget)
    }
}

impl<'a, Message, C> From<&'a mut C> for ChartWidget<'a, Message, C>
where
    Message: 'static,
    C: Chart<Message>,
{
    #[inline]
    fn from(chart: &'a mut C) -> ChartWidget<'a, Message, C> {
        ChartWidget::new(chart)
    }
}
