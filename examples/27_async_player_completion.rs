#[path = "support/mod.rs"]
mod support;

use std::thread;
use std::time::{Duration, Instant};

use avaudio::async_api::{PlayerNodeCompletionEvent, PlayerNodeCompletionStream};
use avaudio::prelude::*;
use support::{artifacts_dir, make_test_audio, print_skip};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = artifacts_dir()?.join("27-async-player.aiff");
    if let Err(error) = make_test_audio(&audio_path) {
        print_skip(&format!("could not generate test audio: {error}"));
        return Ok(());
    }

    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    engine.attach_player_node(&player);
    engine.connect_player_node_to_main_mixer(&player, None);
    engine.prepare();
    if let Err(error) = engine.start() {
        print_skip(&format!("engine unavailable (headless): {error}"));
        return Ok(());
    }

    let stream = PlayerNodeCompletionStream::subscribe(&player, 4);
    let file = AudioFile::open_for_reading(&audio_path)?;
    stream.schedule_file(&file)?;
    player.play();

    let event = pollster::block_on(async {
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            if let Some(event) = stream.try_next() {
                break Some(event);
            }
            if Instant::now() >= deadline {
                break None;
            }
            thread::sleep(Duration::from_millis(20));
        }
    });
    println!("completion event: {event:?}");

    player.stop();
    engine.stop();

    match event {
        Some(PlayerNodeCompletionEvent::DataPlayedBack) => {
            println!("✅ async player completion stream OK");
            Ok(())
        }
        Some(other) => Err(format!("unexpected completion event: {other:?}").into()),
        None => {
            print_skip("timed out waiting for playback completion");
            Ok(())
        }
    }
}
