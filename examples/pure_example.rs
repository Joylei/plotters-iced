// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT
extern crate iced;
extern crate plotters;
extern crate rand;

use iced::pure::{
    widget::canvas::{Cache, Frame, Geometry},
    Application,
};
use iced::pure::{
    widget::{Column, Container},
    Element,
};
use iced::{executor, Alignment, Command, Font, Length, Settings, Size, Subscription};
use plotters::{prelude::ChartBuilder, series::LineSeries, style::RED};
use plotters_backend::DrawingBackend;
use plotters_iced::pure::{Chart, ChartWidget};

const TITLE_FONT_SIZE: u16 = 22;

const FONT_BOLD: Font = Font::External {
    name: "sans-serif-bold",
    bytes: include_bytes!("./fonts/notosans-bold.ttf"),
};

fn main() {
    State::run(Settings {
        antialiasing: true,
        default_font: Some(include_bytes!("./fonts/notosans-regular.ttf")),
        ..Settings::default()
    })
    .unwrap();
}

#[derive(Debug)]
enum Message {
    /// message that cause charts' data lazily updated
    Tick,
}

struct State {
    chart: ExampleChart,
}

impl Application for State {
    type Message = self::Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                chart: ExampleChart::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Large Data Example".to_owned()
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
            .push(
                iced::Text::new("Iced test chart")
                    .size(TITLE_FONT_SIZE)
                    .font(FONT_BOLD),
            )
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
        Subscription::none()
    }
}

struct ExampleChart {
    cache: Cache,
}

impl ExampleChart {
    fn new() -> Self {
        Self {
            cache: Cache::new(),
        }
    }

    fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

impl Chart<Message> for ExampleChart {
    type State = ();

    #[inline]
    fn draw<F: Fn(&mut Frame)>(&self, bounds: Size, draw_fn: F) -> Geometry {
        self.cache.draw(bounds, draw_fn)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        let power = 2;
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
}

mod style {
    use iced::Color;

    pub struct ChartContainer;
    impl iced::container::StyleSheet for ChartContainer {
        fn style(&self) -> iced::container::Style {
            iced::container::Style {
                background: Some(Color::BLACK.into()),
                text_color: Some(Color::WHITE),
                ..Default::default()
            }
        }
    }
}
