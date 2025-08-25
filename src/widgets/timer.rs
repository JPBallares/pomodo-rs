use std::f32::consts::PI;

use iced::widget::canvas;
use iced::widget::canvas::Stroke;
use iced::widget::canvas::path::{Arc, Builder};
use iced::{Color, Radians, Rectangle, Renderer, Theme};
use iced::{Element, mouse};

use crate::app::Pomodoro;

#[derive(Debug)]
struct Timer {
    radius: f32,
    percent: f32,
}

impl<Message> canvas::Program<Message> for Timer {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
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
            style: canvas::Style::Solid(Color::parse("#77F774").unwrap_or_default()),
            width: LINE_WIDTH,
            line_cap: canvas::LineCap::Round,
            line_join: canvas::LineJoin::Bevel,
            line_dash: canvas::LineDash::default(),
        };
        frame.stroke(&semi_circle, stroke);
        vec![frame.into_geometry()]
    }
}

pub fn view<'a, Message: 'a>(
    _state: &'a Pomodoro,
    radius: f32,
    percent: f32,
) -> Element<'a, Message> {
    canvas(Timer {
        radius: radius,
        percent: percent,
    })
    .into()
}
