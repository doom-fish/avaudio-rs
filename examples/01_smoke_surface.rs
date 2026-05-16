use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let artifacts = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/example-artifacts");
    std::fs::create_dir_all(&artifacts)?;

    let audio_path = artifacts.join("test.aiff");
    if audio_path.exists() {
        std::fs::remove_file(&audio_path)?;
    }

    let say_status = Command::new("/usr/bin/say")
        .args([
            "-o",
            audio_path.to_str().ok_or("non-UTF-8 artifact path")?,
            "test",
        ])
        .status()?;
    if !say_status.success() {
        return Err(format!("`say` failed with status {say_status}").into());
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
    engine.start()?;

    let completed = Arc::new(AtomicBool::new(false));
    let completed_flag = Arc::clone(&completed);
    player.schedule_buffer_with_completion(&buffer, move || {
        completed_flag.store(true, Ordering::SeqCst);
    })?;
    player.play();

    thread::sleep(Duration::from_millis(250));
    println!("engine running: {}", engine.is_running()?);
    println!("player playing: {}", player.is_playing()?);
    println!("completion fired: {}", completed.load(Ordering::SeqCst));

    player.stop();
    engine.stop();

    println!("✅ avaudio surface OK");
    Ok(())
}
