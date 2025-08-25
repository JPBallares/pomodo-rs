#[derive(Debug, Clone)]
pub enum Message {
    SetMinutes(String),
    SetSeconds(String),
    Pause,
    Resume,
    Stop,
    Tick,
}
