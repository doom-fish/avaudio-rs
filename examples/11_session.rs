#![allow(clippy::unnecessary_wraps)]

#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("session sample rate: {:.2}", AudioSession::sample_rate());
    println!(
        "session output volume: {:.2}",
        AudioSession::output_volume()
    );
    println!(
        "other audio playing: {}",
        AudioSession::is_other_audio_playing()
    );
    Ok(())
}
