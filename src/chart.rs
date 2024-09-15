// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use iced_widget::canvas::Cache;
use iced_widget::core::event::Status;
use iced_widget::core::mouse::Interaction;
use iced_widget::core::Rectangle;
use iced_widget::{
    canvas::{Event, Frame, Geometry},
    core::{mouse::Cursor, Size},
};
use plotters::{chart::ChartBuilder, coord::Shift, drawing::DrawingArea};
use plotters_backend::DrawingBackend;

/// graphics renderer
pub trait Renderer {
    /// draw frame
    fn draw<F: Fn(&mut Frame)>(&self, bounds: Size, draw_fn: F) -> Geometry;

    /// draw frame with cache
    fn draw_cache<F: Fn(&mut Frame)>(&self, cache: &Cache, bounds: Size, draw_fn: F) -> Geometry;
}

impl<Message, C: ?Sized> Chart<Message> for &C
where
    C: Chart<Message>,
{
    type State = C::State;
    #[inline]
    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, builder: ChartBuilder<DB>) {
        C::build_chart(self, state, builder);
    }
    #[inline]
    fn draw_chart<DB: DrawingBackend>(&self, state: &Self::State, root: DrawingArea<DB, Shift>) {
        C::draw_chart(self, state, root);
    }
    #[inline]
    fn draw<R: Renderer, F: Fn(&mut Frame)>(&self, renderer: &R, size: Size, f: F) -> Geometry {
        C::draw(self, renderer, size, f)
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
    #[inline]
    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Interaction {
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
///     type State = ();
///     fn build_chart<DB:DrawingBackend>(&self, state: &Self::State, builder: ChartBuilder<DB>) {
///         //build your chart here, please refer to plotters for more details
///     }
/// }
///
/// impl MyChart {
///     fn view(&mut self)->Element<Message> {
///         ChartWidget::new(self)
///             .width(Length::Fixed(200))
///             .height(Length::Fixed(200))
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
    ///     fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, builder: ChartBuilder<DB>){}
    ///     fn draw_chart<DB: DrawingBackend>(&self, state: &Self::State, root: DrawingArea<DB, Shift>){
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

    /// draw on [`iced_widget::canvas::Canvas`]
    ///
    /// override this method if you want to use [`iced_widget::canvas::Cache`]
    ///
    /// ## Example
    /// ```rust,ignore
    ///
    /// impl Chart<Message> for CpuUsageChart {
    ///
    ///       #[inline]
    ///       fn draw<R: Renderer, F: Fn(&mut Frame)>(&self, renderer: &R, bounds: Size, draw_fn: F) -> Geometry {
    ///            R::draw_cache(renderer, &self.cache, size, draw_fn)
    ///       }
    ///      //...
    /// }
    /// ```
    #[inline]
    fn draw<R: Renderer, F: Fn(&mut Frame)>(&self, renderer: &R, size: Size, f: F) -> Geometry {
        R::draw(renderer, size, f)
    }

    /// react on event
    #[allow(unused_variables)]
    #[inline]
    #[allow(unused)]
    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Message>) {
        (Status::Ignored, None)
    }

    /// Returns the current mouse interaction of the [`Chart`]
    #[inline]
    #[allow(unused)]
    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Interaction {
        Interaction::Idle
    }
}
