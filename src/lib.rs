#![allow(clippy::uninlined_format_args)]

use hound::{SampleFormat, WavReader};
use std::path::Path;
use std::time::Instant;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters, WhisperState};

fn parse_wav_file(path: &Path) -> Vec<i16> {
    let reader = WavReader::open(path).expect("failed to read file");

    if reader.spec().channels != 1 {
        panic!("expected mono audio file");
    }
    if reader.spec().sample_format != SampleFormat::Int {
        panic!("expected integer sample format");
    }
    if reader.spec().sample_rate != 16000 {
        panic!("expected 16KHz sample rate");
    }
    if reader.spec().bits_per_sample != 16 {
        panic!("expected 16 bits per sample");
    }

    reader
        .into_samples::<i16>()
        .map(|x| x.expect("sample"))
        .collect::<Vec<_>>()
}

pub fn print_result(st: Instant, et: Instant, state: &whisper_rs::WhisperState) {
    let num_segments = state
        .full_n_segments()
        .expect("failed to get number of segments");
    for i in 0..num_segments {
        let segment = state
            .full_get_segment_text(i)
            .expect("failed to get segment");
        let start_timestamp = state
            .full_get_segment_t0(i)
            .expect("failed to get start timestamp");
        let end_timestamp = state
            .full_get_segment_t1(i)
            .expect("failed to get end timestamp");
        println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
    }
    println!("took {}ms", (et - st).as_millis());
}

pub fn run_on_one_file<'a>(audio_path: &'a Path, ctx: &'a WhisperContext) -> (Instant, Instant, WhisperState<'a>) {
    let original_samples = parse_wav_file(audio_path);
    let mut samples = vec![0.0f32; original_samples.len()];
    whisper_rs::convert_integer_to_float_audio(&original_samples, &mut samples)
        .expect("failed to convert samples");

    
    let mut state = ctx.create_state().expect("failed to create key");
    let mut params = FullParams::new(SamplingStrategy::default());
    params.set_initial_prompt("experience");
    params.set_progress_callback_safe(|progress| println!("Progress callback: {}%", progress));

    let st = Instant::now();
    state
        .full(params, &samples)
        .expect("failed to convert samples");
    let et = Instant::now();

    return (st, et, state);
        
}

pub fn init_context(whisper_path: &Path) -> WhisperContext {
    let ctx = WhisperContext::new_with_params(
        &whisper_path.to_string_lossy(),
        WhisperContextParameters::default(),
    )
    .expect("failed to open model");
    return ctx;
}

