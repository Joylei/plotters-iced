#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub extern crate plotters_backend;

#[doc(no_inline)]
pub use plotters::{chart::ChartBuilder, drawing::DrawingArea};
#[doc(no_inline)]
pub use plotters_backend::DrawingBackend;

#[doc(inline)]
pub use chart::Chart;
#[doc(inline)]
pub use chart::Renderer;
#[doc(inline)]
pub use error::Error;
pub use widget::ChartWidget;

mod backend;
mod chart;
mod error;
mod renderer;
/// data point sampling
pub mod sample;
mod utils;
mod widget;
