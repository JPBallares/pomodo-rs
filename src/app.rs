use crate::messages::Message;
use crate::widgets::timer;
use iced::time::{self, Duration};
use iced::widget::{Column, button, container, row, text, text_input};
use iced::window::{gain_focus, get_latest};
use iced::{Element, Length::Fill, Subscription, Task};

#[derive(Debug)]
pub struct Pomodoro {
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
        Self {
            initial_minutes: minutes,
            initial_seconds: seconds,
            remaining_time: minutes * 60 + seconds,
            paused: true,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Pause => {
                self.paused = true;
                Task::none()
            }
            Message::Resume => {
                self.paused = false;
                Task::none()
            }
            Message::Stop => {
                self.remaining_time = self.initial_minutes * 60 + self.initial_seconds;
                self.paused = true;

                Task::none()
            }
            Message::SetMinutes(str) => {
                self.initial_minutes = str.parse::<u16>().unwrap_or_default();
                self.remaining_time = self.initial_minutes * 60 + self.initial_seconds;

                Task::none()
            }
            Message::SetSeconds(str) => {
                self.initial_seconds = str.parse::<u16>().unwrap_or_default();
                self.remaining_time = self.initial_minutes * 60 + self.initial_seconds;

                Task::none()
            }
            Message::Tick => {
                if self.remaining_time > 0 {
                    self.remaining_time -= 1;
                    Task::none()
                } else {
                    Task::batch([
                        self.update(Message::Stop),
                        get_latest().and_then::<Message>(gain_focus),
                    ])
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let initial_time = self.initial_minutes * 60 + self.initial_seconds;
        let running = self.remaining_time != initial_time;
        let mut column_elements = Vec::<Element<Message>>::new();
        column_elements.push(
            container(timer::view(100.0, self.remaining_time, initial_time))
                .center_x(Fill)
                .into(),
        );
        if self.paused && !running {
            column_elements.push(
                container(row![
                    text_input("00", &format!("{}", &self.initial_minutes))
                        .on_input(Message::SetMinutes),
                    text(":"),
                    text_input("00", &format!("{}", &self.initial_seconds))
                        .on_input(Message::SetSeconds),
                ])
                .center_x(Fill)
                .into(),
            );
        }
        column_elements.push(
            container(
                row![
                    if self.paused {
                        button(if running { "Resume" } else { "Start" }).on_press(Message::Resume)
                    } else {
                        button("Pause").on_press(Message::Pause)
                    },
                    button(if self.paused && !running {
                        "Reset"
                    } else {
                        "Stop"
                    })
                    .on_press(Message::Stop),
                ]
                .spacing(10),
            )
            .center_x(Fill)
            .into(),
        );
        container(
            Column::from_vec(column_elements)
                .spacing(10)
                .height(Fill)
                .width(Fill),
        )
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.paused {
            Subscription::none()
        } else {
            time::every(Duration::from_secs(1)).map(|_| Message::Tick)
        }
    }
}
