use std::{fs::File, io::prelude::*};
use chrono::prelude::*;

use walkdir::WalkDir;
use whisperdiscordbot;

#[test]
fn test_whisper_summary() {
    let models_folder = ".\\models";
    let test_audio_folder = ".\\tests\\audio-files";

    let time = Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();

    let filename = format!(".\\tests\\results\\output-{}.txt", time);

    let mut file = File::create(filename)
        .expect("Unable to create text file");

    for model in WalkDir::new(models_folder)
            .into_iter()
            .filter(|e| e.as_ref().unwrap().path().is_file())
            .filter_map(|e| e.ok()) {
        let model_path = model.path();

        let ctx = whisperdiscordbot::init_context(model_path);

        writeln!(file, "{}", model_path.file_name().unwrap().to_str().unwrap())
            .expect("Unable to write to log file");


        for audio_file in WalkDir::new(test_audio_folder)
            .into_iter()
            .filter(|e| e.as_ref().unwrap().path().is_file())
            .filter_map(|e| e.ok()) {
                let audio_file_path = audio_file.path();

                let (st, et, _state) = whisperdiscordbot::run_on_one_file(audio_file_path, &ctx);

                writeln!(file, "\t{}\t{}ms", audio_file_path.file_name().unwrap().to_str().unwrap(), (et - st).as_millis())
                    .expect("Unable to write to log file");
            }
        writeln!(file, "").expect("Unable to write to log file");
    }

    
}
