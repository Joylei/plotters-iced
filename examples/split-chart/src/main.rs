// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

/*!
- run the native version

```sh
cargo run --release --example split-chart
```

- run the web version with [trunk](https://trunkrs.dev/)

```sh
cd examples
trunk serve
```

*/

extern crate iced;
extern crate plotters;

use iced::{
    executor,
    widget::{Column, Container, Text},
    Alignment, Application, Command, Element, Length, Settings, Subscription, Theme,
};
use plotters::{coord::Shift, prelude::*};
use plotters_backend::DrawingBackend;
use plotters_iced::{plotters_backend, Chart, ChartWidget, DrawingArea};

const TITLE_FONT_SIZE: u16 = 22;

// antialiasing issue: https://github.com/iced-rs/iced/issues/1159

fn main() {
    State::run(Settings {
        #[cfg(not(target_arch = "wasm32"))]
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap();
}

#[allow(unused)]
#[derive(Debug)]
enum Message {
    Tick,
}

struct State {
    chart: MyChart,
}

impl Application for State {
    type Message = self::Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                chart: MyChart::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Split Chart Example".to_owned()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = Column::new()
            .spacing(20)
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Text::new("Iced test chart").size(TITLE_FONT_SIZE))
            .push(self.chart.view());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        use std::time::Duration;
        iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick)
    }
}

#[allow(unused)]
struct MyChart;

impl MyChart {
    pub fn new() -> Self {
        Self
    }

    fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

impl Chart<Message> for MyChart {
    type State = ();
    // leave it empty
    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, _builder: ChartBuilder<DB>) {}

    fn draw_chart<DB: DrawingBackend>(&self, _state: &Self::State, root: DrawingArea<DB, Shift>) {
        let children = root.split_evenly((2, 2));
        for (i, area) in children.iter().enumerate() {
            let builder = ChartBuilder::on(area);
            draw_chart(builder, i + 1);
        }
    }
}

fn draw_chart<DB: DrawingBackend>(mut chart: ChartBuilder<DB>, power: usize) {
    let mut chart = chart
        .margin(30)
        .caption(format!("y=x^{}", power), ("sans-serif", 22))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(3)
        .y_labels(3)
        // .y_label_style(
        //     ("sans-serif", 15)
        //         .into_font()
        //         .color(&plotters::style::colors::BLACK.mix(0.8))
        //         .transform(FontTransform::RotateAngle(30.0)),
        // )
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (-50..=50)
                .map(|x| x as f32 / 50.0)
                .map(|x| (x, x.powf(power as f32))),
            &RED,
        ))
        .unwrap();
}
