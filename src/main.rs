use std::path::Path;
use whisperdiscordbot::whisper as whisper;

fn main() {
    let arg1 = std::env::args()
        .nth(1)
        .expect("first argument should be path to WAV file");
    let audio_path = Path::new(&arg1);
    if !audio_path.exists() {
        panic!("audio file doesn't exist");
    }
    let arg2 = std::env::args()
        .nth(2)
        .expect("second argument should be path to Whisper model");
    let whisper_path = Path::new(&arg2);
    if !whisper_path.exists() {
        panic!("whisper file doesn't exist")
    }

    let ctx = whisper::init_context(&whisper_path);

    let (st, et, state) = whisper::run_on_one_file(audio_path, &ctx);

    whisper::print_result(st, et, &state);
}