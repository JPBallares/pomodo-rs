use crate::messages::Message;
use crate::widgets::timer;
use iced::Padding;
use iced::time::{self, Duration};
use iced::widget::{Column, button, container, row};
use iced::window::{gain_focus, get_latest};
use iced::{Element, Length::Fill, Subscription, Task};

const DEFAULT_TIME: u16 = 60 * 25;
const MINI_BREAK_TIME: u16 = 60 * 5;
const LONG_BREAK_TIME: u16 = 60 * 30;

#[derive(Debug)]
pub struct Pomodoro {
    initial_time: u16,
    remaining_time: u16,
    paused: bool,
    iteration: u8,
    is_taking_break: bool,
}

impl Default for Pomodoro {
    fn default() -> Self {
        Self::new(DEFAULT_TIME)
    }
}

impl Pomodoro {
    fn new(seconds: u16) -> Self {
        Self {
            initial_time: seconds,
            remaining_time: seconds,
            paused: true,
            iteration: 0,
            is_taking_break: false,
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
                self.remaining_time = self.initial_time;
                self.paused = true;
                Task::none()
            }
            Message::SetSeconds(seconds) => {
                self.initial_time = seconds;
                self.remaining_time = seconds;
                Task::none()
            }
            Message::Tick => {
                if self.remaining_time > 0 {
                    self.remaining_time -= 1;
                    Task::none()
                } else {
                    Task::batch([
                        self.update(Message::ToggleBreak),
                        get_latest().and_then::<Message>(gain_focus),
                    ])
                }
            }
            Message::ToggleBreak => {
                self.is_taking_break = !self.is_taking_break;
                if !self.is_taking_break {
                    self.update(Message::SetSeconds(DEFAULT_TIME))
                } else {
                    self.iteration = (self.iteration + 1) % 4;

                    if self.iteration == 0 {
                        self.update(Message::SetSeconds(LONG_BREAK_TIME))
                    } else {
                        self.update(Message::SetSeconds(MINI_BREAK_TIME))
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let running = self.remaining_time != self.initial_time;
        let mut column_elements = Vec::<Element<Message>>::new();
        column_elements.push(
            container(timer::view(
                100.0,
                self.remaining_time,
                self.initial_time,
                self.is_taking_break,
            ))
            .center_x(Fill)
            .padding(Padding {
                top: 50.0,
                ..Default::default()
            })
            .into(),
        );
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
