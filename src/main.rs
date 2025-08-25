use iced::{Result, Size, application};
use pomodo_rs::app::Pomodoro;

fn main() -> Result {
    application("Pomodo.rs", Pomodoro::update, Pomodoro::view)
        .subscription(Pomodoro::subscription)
        .window_size(Size::new(300.0, 300.0))
        .run()
}
