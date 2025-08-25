use std::f32::consts::PI;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas;
use iced::widget::canvas::path::{Arc, Builder};
use iced::widget::canvas::{Stroke, Text};
use iced::{Color, Radians, Rectangle, Renderer, Theme};
use iced::{Element, mouse};

pub const GREEN: Color = Color {
    r: 0.46,
    g: 0.96,
    b: 0.45,
    a: 1.0,
};

#[derive(Debug, Default)]
struct Timer {
    radius: f32,
    percent: f32,
    time: String,
}

impl<Message> canvas::Program<Message> for Timer {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        const LINE_WIDTH: f32 = 8.0;
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let mut builder = Builder::new();
        builder.arc(Arc {
            center: frame.center(),
            radius: self.radius - LINE_WIDTH,
            start_angle: Radians(0.0 - PI * (1.0 - self.percent)),
            end_angle: Radians(-PI),
        });
        let semi_circle = builder.build();
        let stroke = Stroke {
            style: canvas::Style::Solid(GREEN),
            width: LINE_WIDTH,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Bevel,
            line_dash: canvas::LineDash::default(),
        };
        frame.stroke(&semi_circle, stroke);
        let time_display = Text {
            content: String::clone(&self.time),
            position: frame.center(),
            color: theme.palette().text,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Default::default()
        };
        frame.fill_text(time_display);
        vec![frame.into_geometry()]
    }
}

pub fn view<'a, Message: 'a>(
    radius: f32,
    remaining_time: u16,
    initial_time: u16,
) -> Element<'a, Message> {
    let remaining_minutes = remaining_time / 60; //get the minutes
    let remaining_seconds = remaining_time % 60; // get the seconds
    let percent = if initial_time != 0 {
        f32::from(remaining_time) / f32::from(initial_time)
    } else {
        0.0
    };
    canvas(Timer {
        radius,
        percent,
        time: format!("{remaining_minutes:02}:{remaining_seconds:02}"),
    })
    .width(radius * 2.0)
    .height(radius * 2.0)
    .into()
}
