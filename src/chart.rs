use crate::backend::IcedChartBackend;
use iced_graphics::{
    backend,
    canvas::{self, Cursor, Frame, Geometry},
    Backend, Defaults, Point, Primitive, Renderer, Size,
};
use iced_native::{
    event, mouse::Interaction, Clipboard, Element, Font, Layout, Length, Rectangle, Vector, Widget,
};
use plotters::{chart::ChartBuilder, coord::Shift, drawing::DrawingArea};
use plotters_backend::{DrawingBackend, FontFamily, FontStyle};
use std::hash::Hash;
use std::marker::PhantomData;

/// Chart container, turns [`Chart`]s to [`Widget`]s
pub struct ChartWidget<'a, Message, C, B>
where
    C: Chart<Message>,
    B: Backend + backend::Text,
{
    chart: &'a mut C,
    width: Length,
    height: Length,
    font_resolver: Box<dyn Fn(FontFamily, FontStyle) -> Font>,
    _marker: PhantomData<(Message, B)>,
}

impl<'a, Message, C, B> ChartWidget<'a, Message, C, B>
where
    C: Chart<Message>,
    B: Backend + backend::Text,
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

impl<'a, Message, C, B> Widget<Message, Renderer<B>> for ChartWidget<'a, Message, C, B>
where
    C: Chart<Message>,
    B: Backend + backend::Text + 'static,
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
        _renderer: &Renderer<B>,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let size = limits
            .width(self.width)
            .height(self.height)
            .resolve(Size::ZERO);
        iced_native::layout::Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        _defaults: &Defaults,
        layout: iced_native::Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) -> (Primitive, Interaction) {
        let bounds = layout.bounds();
        let geometry = self.chart.draw(bounds.size(), |frame| {
            let backend = IcedChartBackend::new(frame, renderer.backend(), &self.font_resolver);
            let root: DrawingArea<_, _> = backend.into();
            self.chart.draw_chart(root);
        });
        let translation = Vector::new(bounds.x, bounds.y);
        let cursor = Interaction::default();
        (
            Primitive::Translate {
                translation,
                content: Box::new(geometry.into()),
            },
            cursor,
        )
    }

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer<B>,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
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
            let (status, message) = self.chart.update(canvas_event, bounds, cursor);
            if let Some(m) = message {
                messages.push(m);
            }
            return status;
        }
        event::Status::Ignored
    }

    #[inline]
    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

/// Chart View Model
///
/// use it with [`ChartWidget`].
///
/// ## Example
/// ```rust,ignore
/// struct MyChart;
/// impl Chart<Message> for MyChart {
///     fn build_chart<DB:DrawingBackend>(&self, builder: ChartBuilder<DB>) {
///         //build your chart here, please refer to plotters for more details
///     }
/// }
///
/// impl MyChart {
///     fn view(&mut self)->Element<Message> {
///         ChartWidget::new(self)
///             .width(Length::Unit(200))
///             .height(Length::Unit(200))
///             .into()
///     }
/// }
/// ```
pub trait Chart<Message> {
    /// draw chart with [`ChartBuilder`]
    ///
    /// for simple chart, you impl this method
    fn build_chart<DB: DrawingBackend>(&self, builder: ChartBuilder<DB>);

    /// override this method if you want more freedom of drawing area
    ///
    /// ## Example
    /// ```rust,ignore
    /// use plotters::prelude::*;
    /// use plotters_iced::{Chart,ChartWidget};
    ///
    /// struct MyChart{}
    ///
    /// impl Chart<Message> for MyChart {
    ///     // leave it empty
    ///     fn build_chart<DB: DrawingBackend>(&self, builder: ChartBuilder<DB>){}
    ///     fn draw_chart<DB: DrawingBackend>(&self, root: DrawingArea<DB, Shift>){
    ///          let children = root.split_evenly((3,3));
    ///          for (area, color) in children.into_iter().zip(0..) {
    ///                area.fill(&Palette99::pick(color)).unwrap();
    ///          }
    ///      }
    /// }
    #[inline]
    fn draw_chart<DB: DrawingBackend>(&self, root: DrawingArea<DB, Shift>) {
        let builder = ChartBuilder::on(&root);
        self.build_chart(builder);
    }

    /// draw on [`iced::Canvas`]
    ///
    /// override this method if you want to use [`iced::canvas::Cache`]
    ///
    /// ## Example
    /// ```rust,ignore
    ///  
    /// impl Chart<Message> for CpuUsageChart {
    ///
    ///       #[inline]
    ///       fn draw<F: Fn(&mut Frame)>(&self, bounds: Size, draw_fn: F) -> Geometry {
    ///            self.cache.draw(bounds, draw_fn)
    ///       }
    ///      //...
    /// }
    /// ```
    #[inline]
    fn draw<F: Fn(&mut Frame)>(&self, size: Size, f: F) -> Geometry {
        let mut frame = Frame::new(size);
        f(&mut frame);
        frame.into_geometry()
    }

    #[allow(unused_variables)]
    #[inline]
    fn update(
        &mut self,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        (event::Status::Ignored, None)
    }
}

impl<'a, Message, C, B> From<ChartWidget<'a, Message, C, B>> for Element<'a, Message, Renderer<B>>
where
    Message: 'static,
    C: Chart<Message>,
    B: Backend + backend::Text + 'static,
{
    #[inline]
    fn from(widget: ChartWidget<'a, Message, C, B>) -> Self {
        Element::new(widget)
    }
}

impl<'a, Message, C, B> From<&'a mut C> for ChartWidget<'a, Message, C, B>
where
    Message: 'static,
    C: Chart<Message>,
    B: Backend + backend::Text + 'static,
{
    #[inline]
    fn from(chart: &'a mut C) -> ChartWidget<'a, Message, C, B> {
        ChartWidget::new(chart)
    }
}
