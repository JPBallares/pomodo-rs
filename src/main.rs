use iced::time::{self, Duration};
use iced::widget::{button, column, container, row, text, text_input};
use iced::window::{gain_focus, get_latest};
use iced::{Element, Length::Fill, Result, Size, Subscription, Task, application};

#[derive(Debug, Clone)]
enum Message {
    SetMinutes(String),
    SetSeconds(String),
    Pause,
    Resume,
    Stop,
    Tick,
}

#[derive(Debug)]
struct Pomodoro {
    initial_minutes: u16,
    initial_seconds: u16,
    remaining_time: u16,
    paused: bool,
}

impl Default for Pomodoro {
    fn default() -> Self {
        Self::new(30, 0)
    }
}

impl Pomodoro {
    fn new(minutes: u16, seconds: u16) -> Self {
        return Self {
            initial_minutes: minutes,
            initial_seconds: seconds,
            remaining_time: minutes * 60 + seconds,
            paused: true,
        };
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Pause => (self.paused = true).into(),
            Message::Resume => (self.paused = false).into(),
            Message::Stop => {
                self.remaining_time = self.initial_minutes * 60 + self.initial_seconds;
                self.paused = true;

                Task::none()
            }
            Message::SetMinutes(str) => {
                self.initial_minutes = match str.parse::<u16>() {
                    Ok(num) => num,
                    _ => 0,
                };
                self.remaining_time = self.initial_minutes * 60 + self.initial_seconds;

                Task::none()
            }
            Message::SetSeconds(str) => {
                self.initial_seconds = match str.parse::<u16>() {
                    Ok(num) => num,
                    _ => 0,
                };
                self.remaining_time = self.initial_minutes * 60 + self.initial_seconds;

                Task::none()
            }
            Message::Tick => {
                self.remaining_time -= 1;
                if self.remaining_time == 0 {
                    Task::batch([
                        self.update(Message::Stop),
                        get_latest().and_then::<Message>(gain_focus),
                    ])
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let remaining_minutes = self.remaining_time / 60; //get the minutes
        let remaining_seconds = self.remaining_time % 60; // get the seconds

        let running = !(remaining_seconds == self.initial_seconds
            && remaining_minutes == self.initial_minutes);

        container(
            column![
                match self.paused && !running {
                    true => container(row![
                        text_input("00", &format!("{}", &self.initial_minutes))
                            .on_input(Message::SetMinutes),
                        text(":"),
                        text_input("00", &format!("{}", &self.initial_seconds))
                            .on_input(Message::SetSeconds),
                    ])
                    .center_x(Fill),
                    false => container(text(format!(
                        "{:02}: {:02}",
                        remaining_minutes, remaining_seconds
                    )))
                    .center_x(Fill),
                },
                container(
                    row![
                        match self.paused {
                            true => button(if running { "Resume" } else { "Start" })
                                .on_press(Message::Resume),
                            false => button("Pause").on_press(Message::Pause),
                        },
                        button(if self.paused && !running {
                            "Reset"
                        } else {
                            "Stop"
                        })
                        .on_press(Message::Stop),
                    ]
                    .spacing(10)
                )
                .center_x(Fill)
            ]
            .spacing(10),
        )
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.paused {
            Subscription::none()
        } else {
            time::every(Duration::from_secs(1)).map(|_| Message::Tick)
        }
    }
}

fn main() -> Result {
    application("Pomodo.rs", Pomodoro::update, Pomodoro::view)
        .subscription(Pomodoro::subscription)
        .window_size(Size::new(300.0, 200.0))
        .run()
}
