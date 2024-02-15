// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

extern crate iced;
extern crate plotters;
extern crate sysinfo;

use chrono::{DateTime, Utc};
use iced::{
    alignment::{Horizontal, Vertical},
    executor, font,
    widget::{
        canvas::{Cache, Frame, Geometry},
        Column, Container, Row, Scrollable, Space, Text,
    },
    Alignment, Application, Command, Element, Font, Length, Settings, Size, Subscription, Theme,
};
use plotters::prelude::ChartBuilder;
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget, Renderer};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

const PLOT_SECONDS: usize = 60; //1 min
const TITLE_FONT_SIZE: u16 = 22;
const SAMPLE_EVERY: Duration = Duration::from_millis(1000);

const FONT_BOLD: Font = Font {
    family: font::Family::Name("Noto Sans"),
    weight: font::Weight::Bold,
    ..Font::DEFAULT
};

fn main() {
    State::run(Settings {
        antialiasing: true,
        default_font: Font::with_name("Noto Sans"),
        ..Settings::default()
    })
    .unwrap();
}

#[derive(Debug)]
enum Message {
    /// message that cause charts' data lazily updated
    Tick,
    FontLoaded(Result<(), font::Error>),
}

struct State {
    chart: SystemChart,
}

impl Application for State {
    type Message = self::Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                chart: Default::default(),
            },
            Command::batch([
                font::load(include_bytes!("./fonts/notosans-regular.ttf").as_slice())
                    .map(Message::FontLoaded),
                font::load(include_bytes!("./fonts/notosans-bold.ttf").as_slice())
                    .map(Message::FontLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        "CPU Monitor Example".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick => {
                self.chart.update();
            }
            _ => {}
        }
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
            //.style(style::Container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        const FPS: u64 = 50;
        iced::time::every(Duration::from_millis(1000 / FPS)).map(|_| Message::Tick)
    }
}

struct SystemChart {
    sys: System,
    last_sample_time: Instant,
    items_per_row: usize,
    processors: Vec<CpuUsageChart>,
    chart_height: f32,
}

impl Default for SystemChart {
    fn default() -> Self {
        Self {
            sys: System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::new().with_cpu_usage()),
            ),
            last_sample_time: Instant::now(),
            items_per_row: 3,
            processors: Default::default(),
            chart_height: 300.0,
        }
    }
}

impl SystemChart {
    #[inline]
    fn is_initialized(&self) -> bool {
        !self.processors.is_empty()
    }

    #[inline]
    fn should_update(&self) -> bool {
        !self.is_initialized() || self.last_sample_time.elapsed() > SAMPLE_EVERY
    }

    fn update(&mut self) {
        if !self.should_update() {
            return;
        }
        //eprintln!("refresh...");

        self.sys.refresh_cpu();
        self.last_sample_time = Instant::now();
        let now = Utc::now();
        let data = self.sys.cpus().iter().map(|v| v.cpu_usage() as i32);

        //check if initialized
        if !self.is_initialized() {
            eprintln!("init...");
            let mut processors: Vec<_> = data
                .map(|percent| CpuUsageChart::new(vec![(now, percent)].into_iter()))
                .collect();
            self.processors.append(&mut processors);
        } else {
            //eprintln!("update...");
            for (percent, p) in data.zip(self.processors.iter_mut()) {
                p.push_data(now, percent);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        if !self.is_initialized() {
            Text::new("Loading...")
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center)
                .into()
        } else {
            let mut col = Column::new().width(Length::Fill).height(Length::Shrink);

            let chart_height = self.chart_height;
            let mut idx = 0;
            for chunk in self.processors.chunks(self.items_per_row) {
                let mut row = Row::new()
                    .spacing(15)
                    .padding(20)
                    .width(Length::Fill)
                    .height(Length::Fixed(chart_height))
                    .align_items(Alignment::Center);
                for item in chunk {
                    row = row.push(item.view(idx));
                    idx += 1;
                }
                while idx % self.items_per_row != 0 {
                    row = row.push(Space::new(Length::Fill, Length::Fill));
                    idx += 1;
                }
                col = col.push(row);
            }

            Scrollable::new(col).height(Length::Fill).into()
        }
    }
}

struct CpuUsageChart {
    cache: Cache,
    data_points: VecDeque<(DateTime<Utc>, i32)>,
    limit: Duration,
}

impl CpuUsageChart {
    fn new(data: impl Iterator<Item = (DateTime<Utc>, i32)>) -> Self {
        let data_points: VecDeque<_> = data.collect();
        Self {
            cache: Cache::new(),
            data_points,
            limit: Duration::from_secs(PLOT_SECONDS as u64),
        }
    }

    fn push_data(&mut self, time: DateTime<Utc>, value: i32) {
        let cur_ms = time.timestamp_millis();
        self.data_points.push_front((time, value));
        loop {
            if let Some((time, _)) = self.data_points.back() {
                let diff = Duration::from_millis((cur_ms - time.timestamp_millis()) as u64);
                if diff > self.limit {
                    self.data_points.pop_back();
                    continue;
                }
            }
            break;
        }
        self.cache.clear();
    }

    fn view(&self, idx: usize) -> Element<Message> {
        Container::new(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(5)
                .push(Text::new(format!("Processor {}", idx)))
                .push(ChartWidget::new(self).height(Length::Fill)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }
}

impl Chart<Message> for CpuUsageChart {
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
            .front()
            .unwrap_or(&(
                chrono::DateTime::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
                    chrono::Utc,
                ),
                0,
            ))
            .0;
        let oldest_time = newest_time - chrono::Duration::seconds(PLOT_SECONDS as i64);
        let mut chart = chart
            .x_label_area_size(0)
            .y_label_area_size(28)
            .margin(20)
            .build_cartesian_2d(oldest_time..newest_time, 0..100)
            .expect("failed to build chart");

        chart
            .configure_mesh()
            .bold_line_style(plotters::style::colors::BLUE.mix(0.1))
            .light_line_style(plotters::style::colors::BLUE.mix(0.05))
            .axis_style(ShapeStyle::from(plotters::style::colors::BLUE.mix(0.45)).stroke_width(1))
            .y_labels(10)
            .y_label_style(
                ("sans-serif", 15)
                    .into_font()
                    .color(&plotters::style::colors::BLUE.mix(0.65))
                    .transform(FontTransform::Rotate90),
            )
            .y_label_formatter(&|y| format!("{}%", y))
            .draw()
            .expect("failed to draw chart mesh");

        chart
            .draw_series(
                AreaSeries::new(
                    self.data_points.iter().map(|x| (x.0, x.1)),
                    0,
                    PLOT_LINE_COLOR.mix(0.175),
                )
                .border_style(ShapeStyle::from(PLOT_LINE_COLOR).stroke_width(2)),
            )
            .expect("failed to draw chart data");
    }
}
