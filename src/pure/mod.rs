// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT
mod pure;

use crate::Chart;

/// Chart container, turns [`Chart`]s to [`iced_native::Widget`]s
pub type ChartWidget<Message, C> = pure::ChartWidget<Message, C>;

impl<'a, Message, C> From<C> for ChartWidget<Message, C>
where
    Message: 'static,
    C: Chart<Message>,
{
    #[inline]
    fn from(chart: C) -> ChartWidget<Message, C> {
        ChartWidget::new(chart)
    }
}
