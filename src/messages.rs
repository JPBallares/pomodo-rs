#[derive(Debug, Clone)]
pub enum Message {
    SetSeconds(u16),
    Pause,
    Resume,
    Stop,
    Tick,
    ToggleBreak,
    PlayAudio,
}
