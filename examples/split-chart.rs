// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

extern crate iced;
extern crate plotters;
extern crate sysinfo;

use iced::{
    executor, Align, Application, Clipboard, Column, Command, Container, Element, Font, Length,
    Settings,
};
use plotters::{coord::Shift, prelude::*};
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget, DrawingArea};

const TITLE_FONT_SIZE: u16 = 22;

const FONT_REGULAR: Font = Font::External {
    name: "sans-serif-regular",
    bytes: include_bytes!("./fonts/notosans-regular.ttf"),
};

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
enum Message {}

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
                chart: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "CPU Monitor Example".to_owned()
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Column::new()
            .spacing(20)
            .align_items(Align::Start)
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

    // fn subscription(&self) -> Subscription<Self::Message> {
    //     const FPS: u64 = 10;
    //     iced::time::every(Duration::from_millis(1000 / FPS)).map(|_| Message::Tick)
    // }
}

#[derive(Default)]
struct MyChart {}

impl MyChart {
    fn view(&mut self) -> Element<Message> {
        ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .resolve_font(|_, style| match style {
                plotters_backend::FontStyle::Bold => FONT_BOLD,
                _ => FONT_REGULAR,
            })
            .into()
    }
}

impl Chart<Message> for MyChart {
    // leave it empty
    fn build_chart<DB: DrawingBackend>(&self, _builder: ChartBuilder<DB>) {}

    fn draw_chart<DB: DrawingBackend>(&self, root: DrawingArea<DB, Shift>) {
        let children = root.split_evenly((2, 2));
        for (area, color) in children.into_iter().zip(0..) {
            area.fill(&Palette99::pick(color)).unwrap();
        }
    }
}
