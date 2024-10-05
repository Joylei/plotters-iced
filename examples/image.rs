extern crate iced;
extern crate plotters;
extern crate rand;
extern crate tokio;

use std::{fs::File, io::BufReader};

use iced::{
    widget::{
        canvas::{Cache, Frame, Geometry},
        Column, Container, Text,
    },
    Alignment, Element, Length, Size, Task,
};

use image::{DynamicImage, GenericImageView, ImageFormat};
use plotters::prelude::ChartBuilder;
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget, Renderer};

fn main() {
    iced::application("Image Example", State::update, State::view)
        .antialiasing(true)
        .run_with(State::new)
        .unwrap();
}

#[derive(Debug)]
enum Message {
    ImageLoaded(DynamicImage),
}

struct State {
    chart: Option<ExampleChart>,
}

impl State {
    fn new() -> (Self, Task<Message>) {
        (
            Self { chart: None },
            Task::batch([Task::perform(
                tokio::task::spawn_blocking(load_image),
                |data| Message::ImageLoaded(data.unwrap()),
            )]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ImageLoaded(data) => {
                self.chart = Some(ExampleChart::new(data));
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = Column::new()
            .spacing(20)
            .align_x(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
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
    image: DynamicImage,
}

impl ExampleChart {
    fn new(image: DynamicImage) -> Self {
        Self {
            cache: Cache::new(),
            image,
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
    fn draw<R: Renderer, F: Fn(&mut Frame)>(
        &self,
        renderer: &R,
        bounds: Size,
        draw_fn: F,
    ) -> Geometry {
        renderer.draw_cache(&self.cache, bounds, draw_fn)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        use plotters::prelude::*;

        let mut chart = builder
            .build_cartesian_2d(0.0..1.0, 0.0..1.0)
            .expect("failed to build chart");

        let (w, h) = self.image.dimensions();
        let bitmap = BitMapElement::with_ref((0.5, 0.5), (w, h), self.image.as_rgba8().unwrap());

        chart
            .draw_series(bitmap)
            .expect("failed to draw chart data");
    }
}

fn load_image() -> DynamicImage {
    let file_name = format!("./examples/images/rustacean-orig-noshadow.png");

    let image = image::load(
        BufReader::new(File::open(file_name).expect("couldn't open file")),
        ImageFormat::Png,
    )
    .expect("loading image failed");

    return image;
}
