//! Integration tests for the `async` stream surfaces.

#![cfg(feature = "async")]

mod common;

use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use avaudio::async_api::*;
use avaudio::prelude::*;

fn block<F: Future>(future: F) -> F::Output {
    pollster::block_on(future)
}

fn artifacts_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/example-artifacts");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn make_engine() -> Option<AudioEngine> {
    AudioEngine::new().ok()
}

#[test]
fn config_change_stream_subscribe_drop() {
    let Some(engine) = make_engine() else {
        return;
    };
    let stream = ConfigChangeStream::subscribe(&engine, 4);
    assert_eq!(stream.buffered_count(), 0);
    assert!(stream.try_next().is_none());
    drop(stream);
}

#[test]
fn player_node_completion_stream_basic() {
    let Some(engine) = make_engine() else {
        return;
    };
    let Ok(player) = AudioPlayerNode::new() else {
        return;
    };
    engine.attach_player_node(&player);
    engine.connect_player_node_to_main_mixer(&player, None);
    engine.prepare();
    if engine.start().is_err() {
        return;
    }

    let Ok(format) = engine.main_mixer_output_format(0) else {
        engine.stop();
        return;
    };
    let Ok(mut buffer) = PCMBuffer::new(&format, 512) else {
        engine.stop();
        return;
    };
    if buffer.set_frame_length(512).is_err() {
        engine.stop();
        return;
    }

    let stream = PlayerNodeCompletionStream::subscribe(&player, 4);
    if stream
        .schedule_buffer(&buffer, AudioPlayerNodeBufferOptions::NONE)
        .is_err()
    {
        engine.stop();
        return;
    }
    player.play();

    let event = block(async {
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            if let Some(event) = stream.try_next() {
                break Some(event);
            }
            if Instant::now() >= deadline {
                break None;
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    player.stop();
    engine.stop();
    assert!(matches!(
        event,
        Some(PlayerNodeCompletionEvent::DataPlayedBack)
    ));
}

#[test]
fn recorder_event_stream_subscribe_drop() -> Result<(), Box<dyn std::error::Error>> {
    let recording_path = artifacts_dir()?.join("async-recorder-stream.caf");
    let Ok(recorder) = AudioRecorder::create(&recording_path, 44_100.0, 1, 16) else {
        return Ok(());
    };
    let stream = RecorderEventStream::subscribe(&recorder, 4);
    assert_eq!(stream.buffered_count(), 0);
    assert!(stream.try_next().is_none());
    drop(stream);
    Ok(())
}

#[test]
fn simple_player_event_stream_subscribe_drop() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = artifacts_dir()?.join("async-simple-player.aiff");
    if common::make_test_audio(&audio_path).is_err() {
        return Ok(());
    }
    let Ok(player) = AudioSimplePlayer::create_from_path(&audio_path) else {
        return Ok(());
    };
    let stream = SimplePlayerEventStream::subscribe(&player, 4);
    assert_eq!(stream.buffered_count(), 0);
    assert!(stream.try_next().is_none());
    drop(stream);
    Ok(())
}

#[test]
fn tap_buffer_stream_basic() {
    let Some(engine) = make_engine() else {
        return;
    };
    let Ok(player) = AudioPlayerNode::new() else {
        return;
    };
    engine.attach_player_node(&player);
    engine.connect_player_node_to_main_mixer(&player, None);
    engine.prepare();
    if engine.start().is_err() {
        return;
    }

    let Ok(format) = engine.main_mixer_output_format(0) else {
        engine.stop();
        return;
    };
    let Ok(mut buffer) = PCMBuffer::new(&format, 512) else {
        engine.stop();
        return;
    };
    if buffer.set_frame_length(512).is_err() {
        engine.stop();
        return;
    }

    let Ok(mixer) = engine.main_mixer_node() else {
        engine.stop();
        return;
    };
    let tap_stream = TapBufferStream::subscribe_to_node(&mixer, 0, 4096, None, 16);
    if player.schedule_buffer(&buffer).is_err() {
        drop(tap_stream);
        engine.stop();
        return;
    }
    player.play();

    let maybe_event = block(async {
        let deadline = Instant::now() + Duration::from_millis(500);
        loop {
            if let Some(event) = tap_stream.try_next() {
                break Some(event);
            }
            if Instant::now() >= deadline {
                break None;
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    if let Some(event) = maybe_event {
        assert!(event.channel_count > 0);
        assert!(event.sample_rate > 0.0);
    }

    player.stop();
    drop(tap_stream);
    engine.stop();
}
