use rodio::{Decoder, OutputStream, Source};
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

pub fn play_audio(id: String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(format!("./audios/{}.mpeg", id)).unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    sleep(Duration::from_secs(5));
}