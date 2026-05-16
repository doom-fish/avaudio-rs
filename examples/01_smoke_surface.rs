#[path = "support/mod.rs"]
mod support;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use avaudio::prelude::*;
use support::{artifacts_dir, make_test_audio, print_skip, short_sleep};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = artifacts_dir()?.join("01-smoke-surface.aiff");
    if let Err(error) = make_test_audio(&audio_path) {
        print_skip(&format!("could not generate test audio: {error}"));
        return Ok(());
    }

    let file = AudioFile::open_for_reading(&audio_path)?;
    println!("file length frames: {}", file.length_frames()?);
    println!("file info: {:?}", file.info()?);

    let format = file.processing_format()?;
    println!("processing format: {:?}", format.info()?);

    let buffer = file.read_pcm_buffer(2048)?;
    println!("buffer info: {:?}", buffer.info()?);

    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    engine.attach_player_node(&player);
    engine.connect_player_node_to_main_mixer(&player, Some(&format));
    engine.prepare();
    if let Err(error) = engine.start() {
        print_skip(&format!("engine.start() unavailable (headless): {error}"));
        return Ok(());
    }

    let completed = Arc::new(AtomicBool::new(false));
    let completed_flag = Arc::clone(&completed);
    player.schedule_buffer_with_completion(&buffer, move || {
        completed_flag.store(true, Ordering::SeqCst);
    })?;
    player.play();

    short_sleep();
    println!("engine running: {}", engine.is_running()?);
    println!("player playing: {}", player.is_playing()?);
    println!("completion fired: {}", completed.load(Ordering::SeqCst));

    player.stop();
    engine.stop();

    println!("✅ avaudio surface OK");
    Ok(())
}
