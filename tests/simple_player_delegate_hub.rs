use std::ffi::c_void;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use avaudio::async_api::{SimplePlayerEvent, SimplePlayerEventStream};
use avaudio::prelude::*;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopDefaultMode: *const c_void;
    fn CFRunLoopRunInMode(
        mode: *const c_void,
        seconds: f64,
        return_after_source_handled: u8,
    ) -> i32;
}

fn write_silent_wav(path: &Path, frames: usize) -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![0_u8; frames * 2];
    let data_len = u32::try_from(data.len())?;
    let mut bytes = Vec::with_capacity(44 + data.len());
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_len).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&44_100_u32.to_le_bytes());
    bytes.extend_from_slice(&88_200_u32.to_le_bytes());
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&16_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_len.to_le_bytes());
    bytes.extend_from_slice(&data);
    fs::write(path, bytes)?;
    Ok(())
}

fn pump_main_run_loop_until(timeout: Duration, mut done: impl FnMut() -> bool) {
    let deadline = Instant::now() + timeout;
    while !done() && Instant::now() < deadline {
        unsafe { CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.05, 0) };
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-artifacts");
    fs::create_dir_all(&dir)?;
    let path = dir.join("simple-player-delegate-hub.wav");
    write_silent_wav(&path, 2_205)?;

    let player = AudioSimplePlayer::create_from_path(&path)?;
    player.set_volume(0.0);
    let (tx, rx) = mpsc::channel();
    player.set_delegate(AudioSimplePlayerDelegate::new().on_finish_playing(
        move |successfully| {
            let _ = tx.send(successfully);
        },
    ))?;
    let stream = SimplePlayerEventStream::subscribe(&player, 4);
    if !player.play() {
        println!("simple_player_delegate_hub: skipped, no audio output device");
        return Ok(());
    }

    let mut delegate_events = Vec::new();
    let mut stream_events = Vec::new();
    pump_main_run_loop_until(Duration::from_secs(5), || {
        delegate_events.extend(rx.try_iter());
        stream_events.extend(std::iter::from_fn(|| stream.try_next()));
        !delegate_events.is_empty() && !stream_events.is_empty()
    });
    assert_eq!(delegate_events, [true]);
    assert_eq!(
        stream_events,
        [SimplePlayerEvent::DidFinishPlaying { successfully: true }]
    );

    drop(stream);
    player.set_current_time(0.0);
    assert!(player.play());
    pump_main_run_loop_until(Duration::from_secs(5), || {
        delegate_events.extend(rx.try_iter());
        delegate_events.len() == 2
    });
    assert_eq!(delegate_events, [true, true]);
    println!("simple_player_delegate_hub: ok");
    Ok(())
}
