#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;
use support::{artifacts_dir, make_test_audio, print_skip};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = artifacts_dir()?.join("20-audio-file.aiff");
    if let Err(error) = make_test_audio(&audio_path) {
        print_skip(&format!("could not generate test audio: {error}"));
        return Ok(());
    }

    let file = AudioFile::open_for_reading(&audio_path)?;
    println!("file length frames: {}", file.length_frames()?);
    println!("file info: {:?}", file.info()?);
    println!("processing format: {:?}", file.processing_format()?.info()?);
    println!("file format: {:?}", file.file_format()?.info()?);
    Ok(())
}
