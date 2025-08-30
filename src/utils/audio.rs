use rodio::source::SineWave;
use rodio::{OutputStreamBuilder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub fn play_audio(path: &str) {
    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = Sink::connect_new(&stream_handle.mixer());
    let file = BufReader::new(File::open(path).unwrap());
    let source = rodio::Decoder::new(file).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

pub fn play_beep() {
    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = Sink::connect_new(&stream_handle.mixer());
    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(0.5))
        .amplify(0.20);
    sink.append(source);
    sink.sleep_until_end();
}
