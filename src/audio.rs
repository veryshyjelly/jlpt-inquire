use rodio::{Decoder, OutputStreamHandle, Source};
use std::fs::File;
use std::io::BufReader;

pub fn play_audio(id: String, output_stream: &OutputStreamHandle) {
    let file = BufReader::new(File::open(format!("./audios/{}.mpeg", id)).unwrap());
    let source = Decoder::new(file).unwrap();
    output_stream.play_raw(source.convert_samples()).unwrap();
}