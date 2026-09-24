//! Integration tests for the `async` stream surfaces.

#![cfg(feature = "async")]

mod common;

use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;
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
fn muted_speech_activity_stream_subscribe_drop() {
    let Some(engine) = make_engine() else {
        return;
    };
    let Ok(input) = engine.input_node() else {
        return;
    };
    let Ok(stream) = MutedSpeechActivityStream::subscribe(&input, 4) else {
        return;
    };
    assert_eq!(stream.buffered_count(), 0);
    assert!(stream.try_next().is_none());
    drop(stream);
}

fn offline_player_engine(
    format: &AudioFormat,
) -> Result<(AudioEngine, AudioPlayerNode), AVAudioError> {
    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    engine.attach_player_node(&player)?;
    engine.connect_player_node_to_main_mixer(&player, Some(format))?;
    engine.enable_manual_rendering_mode(AudioEngineManualRenderingMode::Offline, format, 4096)?;
    engine.start()?;
    Ok((engine, player))
}

fn filled_buffer(format: &AudioFormat, frames: u32, value: f32) -> Result<PCMBuffer, AVAudioError> {
    let mut buffer = PCMBuffer::new(format, frames)?;
    buffer.set_frame_length(frames)?;
    if let Some(PCMChannelDataMut::Deinterleaved(channels)) = buffer.channel_data_mut::<f32>()? {
        for channel in channels {
            channel.fill(value);
        }
    }
    Ok(buffer)
}

fn render_blocks(engine: &AudioEngine, blocks: usize) -> Result<(), AVAudioError> {
    let mut output = PCMBuffer::new(&engine.manual_rendering_format()?, 4096)?;
    for _ in 0..blocks {
        engine.render_offline(4096, &mut output)?;
    }
    Ok(())
}

fn next_tap_event(stream: &TapBufferStream, timeout: Duration) -> Option<TapBufferEvent> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(event) = stream.try_next() {
            return Some(event);
        }
        if Instant::now() >= deadline {
            return None;
        }
        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn player_node_completion_stream_basic() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    engine.attach_player_node(&player)?;
    engine.connect_player_node_to_main_mixer(&player, None)?;
    if engine.prepare().and_then(|()| engine.start()).is_err() {
        return Ok(());
    }

    let format = engine.main_mixer_output_format(0)?;
    let buffer = filled_buffer(&format, 512, 0.0)?;
    let stream = PlayerNodeCompletionStream::subscribe(&player, 4);
    stream.schedule_buffer(
        &buffer,
        AudioPlayerNodeBufferOptions::NONE,
        AudioPlayerNodeCompletionCallbackType::DataPlayedBack,
    )?;
    player.play()?;

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
    Ok(())
}

#[test]
fn player_node_completion_stream_reports_the_requested_callback_type(
) -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 2, false)?;
    let (engine, player) = offline_player_engine(&format)?;
    let buffer = filled_buffer(&format, 512, 0.25)?;
    let stream = PlayerNodeCompletionStream::subscribe(&player, 4);

    assert!(matches!(
        stream.schedule_buffer(
            &buffer,
            AudioPlayerNodeBufferOptions::NONE,
            AudioPlayerNodeCompletionCallbackType::Other(9),
        ),
        Err(AVAudioError::InvalidArgument(_))
    ));
    assert!(!buffer.is_scheduled());

    stream.schedule_buffer(
        &buffer,
        AudioPlayerNodeBufferOptions::NONE,
        AudioPlayerNodeCompletionCallbackType::DataRendered,
    )?;
    player.play()?;
    render_blocks(&engine, 2)?;

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
    assert_eq!(event, Some(PlayerNodeCompletionEvent::DataRendered));
    Ok(())
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
fn recorder_event_streams_keep_the_registered_delegate() -> Result<(), Box<dyn std::error::Error>> {
    let recording_path = artifacts_dir()?.join("async-recorder-delegate.caf");
    let recorder = AudioRecorder::create(&recording_path, 44_100.0, 1, 16)?;
    let marker = Arc::new(());
    let held = Arc::clone(&marker);
    recorder.set_delegate(AudioRecorderDelegate::new().on_finish_recording(move |_| {
        let _ = &held;
    }))?;
    let first = RecorderEventStream::subscribe(&recorder, 4);
    let second = RecorderEventStream::subscribe(&recorder, 4);
    assert_eq!(Arc::strong_count(&marker), 2);
    drop(first);
    drop(second);
    assert_eq!(Arc::strong_count(&marker), 2);
    recorder.clear_delegate();
    assert_eq!(Arc::strong_count(&marker), 1);
    Ok(())
}

#[test]
fn simple_player_event_streams_keep_the_registered_delegate(
) -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = artifacts_dir()?.join("async-simple-player-delegate.aiff");
    common::make_test_audio(&audio_path)?;
    let player = AudioSimplePlayer::create_from_path(&audio_path)?;
    let marker = Arc::new(());
    let held = Arc::clone(&marker);
    player.set_delegate(
        AudioSimplePlayerDelegate::new().on_finish_playing(move |_| {
            let _ = &held;
        }),
    )?;
    let first = SimplePlayerEventStream::subscribe(&player, 4);
    let second = SimplePlayerEventStream::subscribe(&player, 4);
    assert_eq!(Arc::strong_count(&marker), 2);
    drop(second);
    drop(first);
    assert_eq!(Arc::strong_count(&marker), 2);
    player.clear_delegate();
    assert_eq!(Arc::strong_count(&marker), 1);
    Ok(())
}

#[test]
fn tap_buffer_stream_delivers_copied_samples() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 2, false)?;
    let (engine, player) = offline_player_engine(&format)?;
    let stream = TapBufferStream::subscribe_to_node(&player, 0, 4096, None, 16)?;
    let buffer = filled_buffer(&format, 48_000, 0.25)?;
    player.schedule_buffer(&buffer)?;
    player.play()?;
    render_blocks(&engine, 8)?;

    let event = next_tap_event(&stream, Duration::from_secs(3)).ok_or("no tap buffer arrived")?;
    assert_eq!(event.channel_count, 2);
    assert!((event.sample_rate - 48_000.0).abs() < f64::EPSILON);
    let Some(PCMSamples::Float32(channels)) = &event.samples else {
        panic!("tap event carried no float samples: {:?}", event.samples);
    };
    assert_eq!(channels.len(), 2);
    let frames = usize::try_from(event.frame_length)?;
    assert!(frames > 0);
    assert!(channels.iter().all(|channel| channel.len() == frames));
    assert!(channels
        .iter()
        .flatten()
        .any(|sample| (sample - 0.25).abs() < 1e-6));

    player.stop();
    drop(stream);
    engine.stop();
    Ok(())
}

#[test]
fn second_tap_on_a_bus_fails_without_removing_the_first() -> Result<(), Box<dyn std::error::Error>>
{
    let format = AudioFormat::standard(48_000.0, 2, false)?;
    let (engine, player) = offline_player_engine(&format)?;
    let first = TapBufferStream::subscribe_to_node(&player, 0, 4096, None, 16)?;

    let second = TapBufferStream::subscribe_to_node(&player, 0, 4096, None, 16);
    assert!(matches!(second, Err(AVAudioError::CallbackError(_))));

    let buffer = filled_buffer(&format, 48_000, 0.5)?;
    player.schedule_buffer(&buffer)?;
    player.play()?;
    render_blocks(&engine, 8)?;
    assert!(next_tap_event(&first, Duration::from_secs(3)).is_some());

    drop(first);
    let replacement = TapBufferStream::subscribe_to_node(&player, 0, 4096, None, 16);
    assert!(replacement.is_ok());

    player.stop();
    drop(replacement);
    engine.stop();
    Ok(())
}

#[test]
fn tap_buffer_stream_rejects_invalid_requests() -> Result<(), Box<dyn std::error::Error>> {
    let detached = AudioPlayerNode::new()?;
    assert!(matches!(
        TapBufferStream::subscribe_to_node(&detached, 0, 4096, None, 16),
        Err(AVAudioError::CallbackError(_))
    ));

    let format = AudioFormat::standard(48_000.0, 2, false)?;
    let (engine, player) = offline_player_engine(&format)?;
    assert!(matches!(
        TapBufferStream::subscribe_to_node(&player, 0, 4096, None, 0),
        Err(AVAudioError::InvalidArgument(_))
    ));
    assert!(matches!(
        TapBufferStream::subscribe_to_node(&player, usize::MAX, 4096, None, 16),
        Err(AVAudioError::InvalidArgument(_))
    ));
    engine.stop();
    Ok(())
}
