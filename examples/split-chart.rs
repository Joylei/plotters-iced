// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

/*!

## build this example as wasm
First, install wasm-bindgen-cli v0.2.69 (iced requires this version)
```sh
cargo install -f wasm-bindgen-cli --version 0.2.69
```

Then build the code and generate wasm bindings
```sh
cargo build --example split-chart --target wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/debug/examples/split-chart.wasm --out-dir ./examples/js --target web
```

Then, use host the folder examples with a http server
```sh
cargo install https
http examples
```
visit `http://localhost:8000/web-demo.html` in your browser.

*/

extern crate iced;
extern crate plotters;

use iced::{
    executor, Alignment, Application, Column, Command, Container, Element, Length, Settings,
    Subscription,
};
use plotters::{coord::Shift, prelude::*};
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget, DrawingArea};

const TITLE_FONT_SIZE: u16 = 22;

fn main() {
    State::run(Settings {
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

    fn update(
        &mut self,
        _message: Self::Message,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Column::new()
            .spacing(20)
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(iced::Text::new("Iced test chart").size(TITLE_FONT_SIZE))
            .push(self.chart.view());

        Container::new(content)
            //.style(style::Container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::Duration;
            iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick)
        }
        #[cfg(target_arch = "wasm32")]
        {
            Subscription::none()
        }
    }
}

#[allow(unused)]
struct MyChart {
    width: u16,  //wasm32 backend requires fixed size
    height: u16, //wasm32 backend requires fixed size
}

impl MyChart {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }

    fn view(&mut self) -> Element<Message> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let chart = ChartWidget::new(self)
                .width(Length::Fill)
                .height(Length::Fill);

            chart.into()
        }
        #[cfg(target_arch = "wasm32")]
        {
            let width = self.width;
            let height = self.height;
            let chart = ChartWidget::new(self)
                .width(Length::Units(width))
                .height(Length::Units(height));

            chart.into()
        }
    }
}

impl Chart<Message> for MyChart {
    // leave it empty
    fn build_chart<DB: DrawingBackend>(&self, _builder: ChartBuilder<DB>) {}

    fn draw_chart<DB: DrawingBackend>(&self, root: DrawingArea<DB, Shift>) {
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
