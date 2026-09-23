use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use avaudio::prelude::*;

#[test]
fn player_node_schedule_buffer_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 1, false)?;
    let mut buffer = PCMBuffer::new(&format, 256)?;
    buffer.set_frame_length(128)?;

    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    engine.attach_node(&player)?;
    engine.connect_node_to_main_mixer(&player, Some(&format))?;
    player.schedule_buffer(&buffer)?;

    assert!(!player.is_playing()?);
    Ok(())
}

fn offline_player(format: &AudioFormat) -> Result<(AudioEngine, AudioPlayerNode), AVAudioError> {
    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    engine.attach_node(&player)?;
    engine.connect_node_to_main_mixer(&player, Some(format))?;
    engine.enable_manual_rendering_mode(AudioEngineManualRenderingMode::Offline, format, 4096)?;
    engine.start()?;
    Ok((engine, player))
}

fn wait_until(condition: impl Fn() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(3);
    while !condition() {
        if Instant::now() >= deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(10));
    }
    true
}

#[test]
fn mismatched_buffer_is_rejected_and_its_callback_released(
) -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    let stereo = AudioFormat::standard(48_000.0, 2, false)?;
    engine.attach_node(&player)?;
    engine.connect_node_to_main_mixer(&player, Some(&stereo))?;

    let mono = AudioFormat::standard(48_000.0, 1, false)?;
    let mut buffer = PCMBuffer::new(&mono, 64)?;
    buffer.set_frame_length(64)?;
    assert!(matches!(
        player.schedule_buffer(&buffer),
        Err(AVAudioError::PlayerError(_))
    ));
    assert!(!buffer.is_scheduled());

    let marker = Arc::new(());
    let held = Arc::clone(&marker);
    let result = player.schedule_buffer_with_completion(&buffer, move || {
        let _ = &held;
    });
    assert!(matches!(result, Err(AVAudioError::PlayerError(_))));
    assert_eq!(Arc::strong_count(&marker), 1);
    Ok(())
}

#[test]
fn scheduled_buffer_rejects_writes_until_the_player_releases_it(
) -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 2, false)?;
    let (engine, player) = offline_player(&format)?;
    let mut buffer = PCMBuffer::new(&format, 512)?;
    buffer.set_frame_length(512)?;

    player.schedule_buffer(&buffer)?;
    assert!(buffer.is_scheduled());
    assert!(matches!(
        buffer.channel_data_mut::<f32>(),
        Err(AVAudioError::InvalidArgument(_))
    ));
    assert!(buffer.set_frame_length(256).is_err());
    assert!(buffer.channel_data::<f32>().is_some());

    player.play()?;
    let mut output = PCMBuffer::new(&engine.manual_rendering_format()?, 4096)?;
    assert!(matches!(
        engine.render_offline(4096, &mut buffer),
        Err(AVAudioError::InvalidArgument(_))
    ));
    engine.render_offline(4096, &mut output)?;
    assert!(wait_until(|| !buffer.is_scheduled()));
    assert!(buffer.channel_data_mut::<f32>()?.is_some());
    buffer.set_frame_length(256)?;
    engine.stop();
    Ok(())
}

#[test]
fn completion_runs_after_the_player_handle_is_dropped() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 2, false)?;
    let (engine, player) = offline_player(&format)?;
    let mut buffer = PCMBuffer::new(&format, 512)?;
    buffer.set_frame_length(512)?;

    let fired = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&fired);
    player.schedule_buffer_with_completion(&buffer, move || {
        flag.store(true, Ordering::SeqCst);
    })?;
    player.play()?;
    drop(player);

    let mut output = PCMBuffer::new(&engine.manual_rendering_format()?, 4096)?;
    engine.render_offline(4096, &mut output)?;
    assert!(wait_until(|| fired.load(Ordering::SeqCst)));
    engine.stop();
    Ok(())
}
