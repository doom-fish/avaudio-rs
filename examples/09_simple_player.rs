#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;
use support::{artifacts_dir, make_test_audio, print_skip, short_sleep};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = artifacts_dir()?.join("09-simple-player.aiff");
    if let Err(error) = make_test_audio(&audio_path) {
        print_skip(&format!("could not generate test audio: {error}"));
        return Ok(());
    }

    let player = AudioSimplePlayer::create_from_path(&audio_path)?;
    player.set_volume(0.5);
    player.set_pan(-0.25);
    player.set_rate(1.1);
    let prepared = player.prepare_to_play();
    let playing = player.play();

    println!("prepared: {prepared}");
    println!("playing: {playing}");
    println!("duration: {:.2}", player.duration());
    println!(
        "volume: {:.2}, pan: {:.2}, rate: {:.2}",
        player.volume(),
        player.pan(),
        player.rate()
    );
    short_sleep();
    player.pause();
    player.stop();
    if !playing {
        print_skip("AVAudioPlayer could not start playback on this host");
    }
    Ok(())
}
