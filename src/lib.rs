// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

/*!
   The Plotters Iced backend, for both native and wasm applications.

   This is an implementation of a Iced backend for Plotters.

   This backend has been optimized as for speed. Note that some specific plotting features supported in the Bitmap backend may not be implemented there, though.

   See the examples for more details.

   ## How to install?

   Include `plotters-iced` in your `Cargo.toml` dependencies:

   ```toml
   [dependencies]
   plotters-iced = "0.5"
   iced = { version = "0.6", features = ["canvas", "tokio"] }
   plotters="0.3"
   ```

   ## Showcase

   ![CPU Monitor Example](https://cdn.jsdelivr.net/gh/Joylei/plotters-iced@0.1.2/images/plotter_iced_demo.png)

   ![WASM Example](https://cdn.jsdelivr.net/gh/Joylei/plotters-iced@0.1.2/images/split-chart-web.png)

   ## Example
   ```rust,ignore
   struct MyChart;
   impl Chart<Message> for MyChart {
      type State = ();
      fn build_chart<DB:DrawingBackend>(&self, state: &Self::State, builder: ChartBuilder<DB>) {
         //build your chart here, please refer to plotters for more details
      }
   }

   impl MyChart {
      fn view(&mut self)->Element<Message> {
         ChartWidget::new(self)
         .width(Length::Unit(200))
               .height(Length::Unit(200))
               .into()
      }
   }
   ```
   See the [examples](https://github.com/Joylei/plotters-iced/tree/master/examples) for more details.
*/
#![warn(missing_docs)]

pub extern crate plotters_backend;
mod backend;
mod chart;
mod error;
mod renderer;
/// data point sampling
pub mod sample;
mod utils;
mod widget;

#[doc(inline)]
pub use chart::Chart;
#[doc(inline)]
pub use error::Error;

#[doc(no_inline)]
pub use plotters::{chart::ChartBuilder, drawing::DrawingArea};
#[doc(no_inline)]
pub use plotters_backend::DrawingBackend;

/// Chart container, turns [`Chart`]s to [`iced_native::Widget`]s
pub use widget::ChartWidget;
