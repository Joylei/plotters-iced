// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

extern crate iced;
extern crate plotters;
extern crate rand;

use chrono::{DateTime, Utc};
use iced::{
    executor,
    widget::{
        canvas::{Cache, Frame, Geometry},
        Column, Container, Text,
    },
    Alignment, Application, Command, Element, Font, Length, Settings, Size, Subscription, Theme,
};
use plotters::prelude::ChartBuilder;
use plotters_backend::DrawingBackend;
use plotters_iced::{
    sample::{DataPoint, LttbSource},
    Chart, ChartWidget,
};
use rand::Rng;
use std::collections::VecDeque;
use std::time::Duration;

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

struct Wrapper<'a>(&'a DateTime<Utc>, &'a f32);

impl DataPoint for Wrapper<'_> {
    fn x(&self) -> f64 {
        self.0.timestamp() as f64
    }

    fn y(&self) -> f64 {
        *self.1 as f64
    }
}

#[derive(Debug)]
enum Message {}

struct State {
    chart: ExampleChart,
}

impl Application for State {
    type Message = self::Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let data = generate_data();
        let sampled: Vec<_> = (&data[..])
            .cast(|v| Wrapper(&v.0, &v.1))
            .lttb(5000)
            .map(|w| (*w.0, *w.1))
            .collect();
        (
            Self {
                chart: ExampleChart::new(sampled.into_iter()),
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
                Text::new("Iced test chart")
                    .size(TITLE_FONT_SIZE)
                    .font(FONT_BOLD),
            )
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
        Subscription::none()
    }
}

struct ExampleChart {
    cache: Cache,
    data_points: VecDeque<(DateTime<Utc>, f32)>,
}

impl ExampleChart {
    fn new(data: impl Iterator<Item = (DateTime<Utc>, f32)>) -> Self {
        let data_points: VecDeque<_> = data.collect();
        Self {
            cache: Cache::new(),
            data_points,
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
    // fn update(
    //     &mut self,
    //     event: Event,
    //     bounds: Rectangle,
    //     cursor: Cursor,
    // ) -> (event::Status, Option<Message>) {
    //     self.cache.clear();
    //     (event::Status::Ignored, None)
    // }

    #[inline]
    fn draw<F: Fn(&mut Frame)>(&self, bounds: Size, draw_fn: F) -> Geometry {
        self.cache.draw(bounds, draw_fn)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        use plotters::{prelude::*, style::Color};

        const PLOT_LINE_COLOR: RGBColor = RGBColor(0, 175, 255);

        // Acquire time range
        let newest_time = self
            .data_points
            .back()
            .unwrap()
            .0
            .checked_add_signed(chrono::Duration::from_std(Duration::from_secs(10)).unwrap())
            .unwrap();
        //let oldest_time = newest_time - chrono::Duration::seconds(PLOT_SECONDS as i64);
        let oldest_time = self
            .data_points
            .front()
            .unwrap()
            .0
            .checked_sub_signed(chrono::Duration::from_std(Duration::from_secs(10)).unwrap())
            .unwrap();
        //dbg!(&newest_time);
        //dbg!(&oldest_time);
        let mut chart = chart
            .x_label_area_size(0)
            .y_label_area_size(28)
            .margin(20)
            .build_cartesian_2d(oldest_time..newest_time, -10.0_f32..110.0_f32)
            .expect("failed to build chart");

        chart
            .configure_mesh()
            .bold_line_style(&plotters::style::colors::BLUE.mix(0.1))
            .light_line_style(&plotters::style::colors::BLUE.mix(0.05))
            .axis_style(ShapeStyle::from(&plotters::style::colors::BLUE.mix(0.45)).stroke_width(1))
            .y_labels(10)
            .y_label_style(
                ("sans-serif", 15)
                    .into_font()
                    .color(&plotters::style::colors::BLUE.mix(0.65))
                    .transform(FontTransform::Rotate90),
            )
            .y_label_formatter(&|y| format!("{}", y))
            .draw()
            .expect("failed to draw chart mesh");

        chart
            .draw_series(
                AreaSeries::new(
                    self.data_points.iter().cloned(),
                    0_f32,
                    &PLOT_LINE_COLOR.mix(0.175),
                )
                .border_style(ShapeStyle::from(&PLOT_LINE_COLOR).stroke_width(2)),
            )
            .expect("failed to draw chart data");
    }
}

fn generate_data() -> Vec<(DateTime<Utc>, f32)> {
    let total = 10_000_000;
    let mut data = Vec::new();
    let mut rng = rand::thread_rng();
    let time_range = (24 * 3600 * 30) as f32;
    let interval = (3600 * 12) as f32;
    let start = Utc::now()
        .checked_sub_signed(
            chrono::Duration::from_std(Duration::from_secs_f32(time_range)).unwrap(),
        )
        .unwrap();
    while data.len() < total {
        let secs = rng.gen_range(0.1..time_range);
        let time = start
            .checked_sub_signed(chrono::Duration::from_std(Duration::from_secs_f32(secs)).unwrap())
            .unwrap();

        let value =
            (((secs % interval) - interval / 2.0) / (interval / 2.0) * std::f32::consts::PI).sin()
                * 50_f32
                + 50_f32;
        data.push((time, value));
    }
    data.sort_by_cached_key(|x| x.0);
    //dbg!(&data[..100]);
    data
}
