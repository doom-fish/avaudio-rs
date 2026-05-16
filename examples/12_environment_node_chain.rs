#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;
use support::{artifacts_dir, make_test_audio, print_skip, short_sleep};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = artifacts_dir()?.join("12-environment-node-chain.aiff");
    if let Err(error) = make_test_audio(&audio_path) {
        print_skip(&format!("could not generate test audio: {error}"));
        return Ok(());
    }

    let file = AudioFile::open_for_reading(&audio_path)?;
    let format = file.processing_format()?;
    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    let environment = AudioEnvironmentNode::new()?;

    engine.attach_node(&player);
    engine.attach_node(&environment);
    engine.connect_nodes(&player, &environment, Some(&format));
    engine.connect_node_to_main_mixer(&environment, Some(&format));
    environment.set_listener_position(0.0, 0.0, 0.0);
    environment.set_listener_orientation(0.0, 0.0, 0.0);
    environment.set_reverb_blend(12.0);
    engine.prepare();
    if let Err(error) = engine.start() {
        print_skip(&format!("engine.start() unavailable (headless): {error}"));
        return Ok(());
    }

    player.schedule_file(&file)?;
    player.play();
    short_sleep();
    println!("player playing: {}", player.is_playing()?);
    println!(
        "environment position: {:?}",
        environment.listener_position()?
    );
    player.stop();
    engine.stop();
    Ok(())
}
