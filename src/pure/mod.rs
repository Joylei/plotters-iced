// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

mod renderer;
mod widget;

use iced_graphics::{
    canvas::Cursor,
    pure::canvas::{Event, Frame, Geometry},
};
use iced_native::{event::Status, Rectangle, Size};
use plotters::{chart::ChartBuilder, coord::Shift, drawing::DrawingArea};
use plotters_backend::DrawingBackend;

pub(crate) use renderer::Renderer;
pub use widget::ChartWidget;

impl<'a, Message, Renderer, C> From<C> for ChartWidget<'a, Message, Renderer, C>
where
    Message: 'a,
    Renderer: self::Renderer,
    C: Chart<Message> + 'a,
{
    #[inline]
    fn from(chart: C) -> ChartWidget<'a, Message, Renderer, C> {
        ChartWidget::new(chart)
    }
}

impl<Message, C> Chart<Message> for &C
where
    C: Chart<Message>,
{
    type State = C::State;
    #[inline]
    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, builder: ChartBuilder<DB>) {
        C::build_chart(self, state, builder)
    }
    #[inline]
    fn draw_chart<DB: DrawingBackend>(&self, state: &Self::State, root: DrawingArea<DB, Shift>) {
        C::draw_chart(self, state, root)
    }
    #[inline]
    fn draw<F: Fn(&mut Frame)>(&self, size: Size, f: F) -> Geometry {
        C::draw(self, size, f)
    }

    #[inline]
    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Message>) {
        C::update(self, state, event, bounds, cursor)
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> iced_native::mouse::Interaction {
        C::mouse_interaction(self, state, bounds, cursor)
    }
}

/// Chart View Model
///
/// ## Example
/// ```rust,ignore
/// use plotters::prelude::*;
/// use plotters_iced::{Chart,ChartWidget};
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
    /// state data of chart
    type State: Default + 'static;

    /// draw chart with [`ChartBuilder`]
    ///
    /// for simple chart, you impl this method
    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, builder: ChartBuilder<DB>);

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
    /// ```
    #[inline]
    fn draw_chart<DB: DrawingBackend>(&self, state: &Self::State, root: DrawingArea<DB, Shift>) {
        let builder = ChartBuilder::on(&root);
        self.build_chart(state, builder);
    }

    /// draw on [`iced_graphics::canvas::Canvas`]
    ///
    /// override this method if you want to use [`iced_graphics::canvas::Cache`]
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

    /// update the state of [`Chart`]
    #[inline]
    fn update(
        &self,
        _state: &mut Self::State,
        _event: Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> (iced_graphics::canvas::event::Status, Option<Message>) {
        (iced_graphics::canvas::event::Status::Ignored, None)
    }

    /// Returns the current mouse interaction of the [`Chart`]
    #[inline]
    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> iced_native::mouse::Interaction {
        iced_native::mouse::Interaction::Idle
    }
}
