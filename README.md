# whisper-summary

A project for me to play around with creating an AI speech summariser

- Uses [tazz4843/whisper-rs](https://github.com/tazz4843/whisper-rs) for speech to text
- Download whisper ggml models from <https://huggingface.co/ggerganov/whisper.cpp/tree/main> and place in the `models` folder
- Use <https://cloudconvert.com/m4a-to-wav> to convert m4a to wav
- Will use <https://crates.io/crates/nnnoiseless> to remove silent bits of incoming noise