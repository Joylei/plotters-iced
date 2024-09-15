// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

extern crate iced;
extern crate plotters;
extern crate rand;
extern crate tokio;

use chrono::{DateTime, Utc};
use iced::{
    font,
    widget::{
        canvas::{Cache, Frame, Geometry},
        Column, Container, Text,
    },
    Alignment, Element, Font, Length, Size, Task,
};
use plotters::prelude::ChartBuilder;
use plotters_backend::DrawingBackend;
use plotters_iced::{
    sample::lttb::{DataPoint, LttbSource},
    Chart, ChartWidget, Renderer,
};
use rand::Rng;
use std::time::Duration;
use std::{collections::VecDeque, time::Instant};

const TITLE_FONT_SIZE: u16 = 22;

const FONT_BOLD: Font = Font {
    family: font::Family::Name("Noto Sans"),
    weight: font::Weight::Bold,
    ..Font::DEFAULT
};

fn main() {
    iced::application("Large Data Example", State::update, State::view)
        .antialiasing(true)
        .default_font(Font::with_name("Noto Sans"))
        .run_with(State::new)
        .unwrap();
}

struct Wrapper<'a>(&'a DateTime<Utc>, &'a f32);

impl DataPoint for Wrapper<'_> {
    #[inline]
    fn x(&self) -> f64 {
        self.0.timestamp() as f64
    }
    #[inline]
    fn y(&self) -> f64 {
        *self.1 as f64
    }
}

#[derive(Debug)]
enum Message {
    FontLoaded(Result<(), font::Error>),
    DataLoaded(Vec<(DateTime<Utc>, f32)>),
    Sampled(Vec<(DateTime<Utc>, f32)>),
}

struct State {
    chart: Option<ExampleChart>,
}

impl State {
    fn new() -> (Self, Task<Message>) {
        (
            Self { chart: None },
            Task::batch([
                font::load(include_bytes!("./fonts/notosans-regular.ttf").as_slice())
                    .map(Message::FontLoaded),
                font::load(include_bytes!("./fonts/notosans-bold.ttf").as_slice())
                    .map(Message::FontLoaded),
                Task::perform(tokio::task::spawn_blocking(generate_data), |data| {
                    Message::DataLoaded(data.unwrap())
                }),
            ]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DataLoaded(data) => Task::perform(
                tokio::task::spawn_blocking(move || {
                    let now = Instant::now();
                    let sampled: Vec<_> = (&data[..])
                        .cast(|v| Wrapper(&v.0, &v.1))
                        .lttb(1000)
                        .map(|w| (*w.0, *w.1))
                        .collect();
                    dbg!(now.elapsed().as_millis());
                    sampled
                }),
                |data| Message::Sampled(data.unwrap()),
            ),
            Message::Sampled(sampled) => {
                self.chart = Some(ExampleChart::new(sampled.into_iter()));
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = Column::new()
            .spacing(20)
            .align_x(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Text::new("Iced test chart")
                    .size(TITLE_FONT_SIZE)
                    .font(FONT_BOLD),
            )
            .push(match self.chart {
                Some(ref chart) => chart.view(),
                None => Text::new("Loading...").into(),
            });

        Container::new(content)
            .padding(5)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
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
    fn draw<R: Renderer, F: Fn(&mut Frame)>(
        &self,
        renderer: &R,
        bounds: Size,
        draw_fn: F,
    ) -> Geometry {
        renderer.draw_cache(&self.cache, bounds, draw_fn)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        use plotters::prelude::*;

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
            .bold_line_style(plotters::style::colors::BLUE.mix(0.1))
            .light_line_style(plotters::style::colors::BLUE.mix(0.05))
            .axis_style(ShapeStyle::from(plotters::style::colors::BLUE.mix(0.45)).stroke_width(1))
            .y_labels(10)
            .y_label_style(
                ("Noto Sans", 15)
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
                    PLOT_LINE_COLOR.mix(0.175),
                )
                .border_style(ShapeStyle::from(PLOT_LINE_COLOR).stroke_width(2)),
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
