use iced::{
    Element,
    Length::Fill,
    Result, Size, application,
    widget::{button, column, container, row, text, text_input},
};

#[derive(Debug, Clone)]
enum Message {
    SetMinutes(String),
    SetSeconds(String),
    ResetTimer,
    Pause,
    Resume,
    Stop,
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

    fn update(&mut self, message: Message) {
        match message {
            Message::Pause => self.paused = true,
            Message::Resume => self.paused = false,
            Message::Stop => {
                self.remaining_time = self.initial_minutes * 60 + self.initial_seconds;
                self.paused = true;
            }
            Message::SetMinutes(str) => {
                self.initial_minutes = match str.parse::<u16>() {
                    Ok(num) => num,
                    _ => 0,
                };
                self.remaining_time = self.initial_minutes * 60 + self.initial_seconds;
            }
            Message::SetSeconds(str) => {
                self.initial_seconds = match str.parse::<u16>() {
                    Ok(num) => num,
                    _ => 0,
                };
                self.remaining_time = self.initial_minutes * 60 + self.initial_seconds;
            }
            Message::ResetTimer => {
                self.remaining_time = self.initial_minutes * 60 + self.initial_seconds
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let remaining_minutes = self.remaining_time / 60; //get the minutes
        let remaining_seconds = self.remaining_time % 60; // get the seconds

        container(
            column![
                match self.paused {
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
                            true => container(button("Resume").on_press(Message::Resume)),
                            false => container(
                                row![
                                    button("Pause").on_press(Message::Pause),
                                    button("Stop").on_press(Message::Stop),
                                ]
                                .spacing(10),
                            ),
                        },
                        button("Reset").on_press(Message::ResetTimer)
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
}

fn main() -> Result {
    application("Pomodo.rs", Pomodoro::update, Pomodoro::view)
        .window_size(Size::new(300.0, 200.0))
        .run()
}
