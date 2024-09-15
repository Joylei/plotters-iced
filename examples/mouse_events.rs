// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Grey <grey@openrobotics.org>
// License: MIT

use iced::{
    event,
    mouse::Cursor,
    widget::{
        canvas::{self, Cache, Frame, Geometry},
        Column, Container, Text,
    },
    Alignment, Element, Length, Point, Size,
};
use plotters::{
    coord::{types::RangedCoordf32, ReverseCoordTranslate},
    prelude::*,
};
use plotters_iced::{Chart, ChartWidget, Renderer};
use std::cell::RefCell;

#[derive(Default)]
struct State {
    chart: ArtChart,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::MouseEvent(event, point) => {
                self.chart.set_current_position(point);
                match event {
                    iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left) => {
                        self.chart.set_down(true);
                    }
                    iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left) => {
                        self.chart.set_down(false);
                    }
                    _ => {
                        // Do nothing
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = Column::new()
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Text::new("Click below!").size(20))
            .push(self.chart.view())
            .align_x(Alignment::Center)
            .padding(15);

        Container::new(content)
            .padding(5)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

#[derive(Default)]
struct ArtChart {
    cache: Cache,
    points: Vec<(f32, f32)>,
    lines: Vec<((f32, f32), (f32, f32))>,
    is_down: bool,
    current_position: Option<(f32, f32)>,
    initial_down_position: Option<(f32, f32)>,
    spec: RefCell<Option<Cartesian2d<RangedCoordf32, RangedCoordf32>>>,
}

impl ArtChart {
    fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }

    fn set_current_position(&mut self, p: Point) {
        if let Some(spec) = self.spec.borrow().as_ref() {
            self.current_position = spec.reverse_translate((p.x as i32, p.y as i32));
            self.cache.clear();
        }
    }

    fn nearby(p0: (f32, f32), p1: (f32, f32)) -> bool {
        let delta = (p1.0 - p0.0, p1.1 - p0.1);
        (delta.0 * delta.0 + delta.1 * delta.1).sqrt() <= 1.0
    }

    fn set_down(&mut self, new_is_down: bool) {
        if !self.is_down && new_is_down {
            self.initial_down_position = self.current_position;
        }

        if self.is_down && !new_is_down {
            if let Some((initial_p, current_p)) =
                self.initial_down_position.zip(self.current_position)
            {
                if Self::nearby(initial_p, current_p) {
                    self.points.push(current_p);
                } else {
                    self.lines.push((initial_p, current_p));
                }
            }
        }

        self.is_down = new_is_down;
    }
}

impl Chart<Message> for ArtChart {
    type State = ();
    fn draw<R: Renderer, F: Fn(&mut Frame)>(
        &self,
        renderer: &R,
        bounds: Size,
        draw_fn: F,
    ) -> Geometry {
        renderer.draw_cache(&self.cache, bounds, draw_fn)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        use plotters::style::colors;

        const POINT_COLOR: RGBColor = colors::RED;
        const LINE_COLOR: RGBColor = colors::BLUE;
        const HOVER_COLOR: RGBColor = colors::YELLOW;
        const PREVIEW_COLOR: RGBColor = colors::GREEN;

        let mut chart = builder
            .x_label_area_size(28_i32)
            .y_label_area_size(28_i32)
            .margin(20_i32)
            .build_cartesian_2d(0_f32..100_f32, 0_f32..100_f32)
            .expect("Failed to build chart");

        chart
            .configure_mesh()
            .bold_line_style(colors::BLACK.mix(0.1))
            .light_line_style(colors::BLACK.mix(0.05))
            .axis_style(ShapeStyle::from(colors::BLACK.mix(0.45)).stroke_width(1))
            .y_labels(10)
            .y_label_style(
                ("sans-serif", 15)
                    .into_font()
                    .color(&colors::BLACK.mix(0.65))
                    .transform(FontTransform::Rotate90),
            )
            .y_label_formatter(&|y| format!("{}", y))
            .draw()
            .expect("Failed to draw chart mesh");

        chart
            .draw_series(
                self.points
                    .iter()
                    .map(|p| Circle::new(*p, 5_i32, POINT_COLOR.filled())),
            )
            .expect("Failed to draw points");

        for line in &self.lines {
            chart
                .draw_series(LineSeries::new(
                    vec![line.0, line.1].into_iter(),
                    LINE_COLOR.filled(),
                ))
                .expect("Failed to draw line");
        }

        if self.is_down {
            if let Some((initial_p, current_p)) =
                self.initial_down_position.zip(self.current_position)
            {
                if Self::nearby(initial_p, current_p) {
                    chart
                        .draw_series(std::iter::once(Circle::new(
                            current_p,
                            5_i32,
                            PREVIEW_COLOR.filled(),
                        )))
                        .expect("Failed to draw preview point");
                } else {
                    chart
                        .draw_series(LineSeries::new(
                            vec![initial_p, current_p].into_iter(),
                            PREVIEW_COLOR.filled(),
                        ))
                        .expect("Failed to draw preview line");
                }
            }
        } else if let Some(current_p) = self.current_position {
            chart
                .draw_series(std::iter::once(Circle::new(
                    current_p,
                    5_i32,
                    HOVER_COLOR.filled(),
                )))
                .expect("Failed to draw hover point");
        }

        *self.spec.borrow_mut() = Some(chart.as_coord_spec().clone());
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        if let Cursor::Available(point) = cursor {
            match event {
                canvas::Event::Mouse(evt) if bounds.contains(point) => {
                    let p_origin = bounds.position();
                    let p = point - p_origin;
                    return (
                        event::Status::Captured,
                        Some(Message::MouseEvent(evt, Point::new(p.x, p.y))),
                    );
                }
                _ => {}
            }
        }
        (event::Status::Ignored, None)
    }
}

#[derive(Debug)]
enum Message {
    MouseEvent(iced::mouse::Event, iced::Point),
}

fn main() -> iced::Result {
    iced::application("Art", State::update, State::view)
        .antialiasing(true)
        .run()
}
