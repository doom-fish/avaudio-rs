mod common;

use std::sync::Arc;

use avaudio::prelude::*;

#[test]
fn input_node_formats() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let input = engine.input_node()?;

    assert!(input.output_format(0)?.sample_rate >= 0.0);
    assert!(input.input_format(0)?.sample_rate >= 0.0);
    Ok(())
}

#[test]
fn input_node_tap_scaffold() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let input = engine.input_node()?;
    input.install_tap_scaffold(0, 256, None)?;
    input.remove_tap(0)?;
    Ok(())
}

#[test]
fn input_node_tap_scaffold_does_not_replace_an_existing_tap(
) -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let input = engine.input_node()?;
    input.install_tap_scaffold(0, 256, None)?;
    assert!(matches!(
        input.install_tap_scaffold(0, 256, None),
        Err(AVAudioError::CallbackError(_))
    ));
    input.remove_tap(0)?;
    input.install_tap_scaffold(0, 256, None)?;
    input.remove_tap(0)?;
    assert!(input.install_tap_scaffold(usize::MAX, 256, None).is_err());
    Ok(())
}

fn render_manual_input(
    buffer_channels: u32,
) -> Result<(AudioEngineManualRenderingStatus, PCMBuffer), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 2, false)?;
    let engine = AudioEngine::new()?;
    engine.enable_manual_rendering_mode(AudioEngineManualRenderingMode::Offline, &format, 1024)?;
    let input = engine.input_node()?;
    assert!(
        input.set_manual_rendering_input_pcm_format_with_callback(&format, move |frames| {
            let format = AudioFormat::standard(48_000.0, buffer_channels, false).ok()?;
            let mut buffer = PCMBuffer::new(&format, frames).ok()?;
            buffer.set_frame_length(frames).ok()?;
            if let Ok(Some(PCMChannelDataMut::Deinterleaved(channels))) =
                buffer.channel_data_mut::<f32>()
            {
                for channel in channels {
                    channel.fill(0.5);
                }
            }
            Some(AudioManualRenderingInput::from_buffer(buffer))
        })
    );
    let mixer = engine.main_mixer_node()?;
    engine.connect_nodes(&input, &mixer, Some(&format))?;
    engine.start()?;
    let mut output = PCMBuffer::new(&engine.manual_rendering_format()?, 1024)?;
    let status = engine.render_offline(512, &mut output)?;
    engine.stop();
    Ok((status, output))
}

#[test]
fn manual_rendering_input_owns_the_returned_buffer() -> Result<(), Box<dyn std::error::Error>> {
    let (status, output) = render_manual_input(2)?;
    assert_eq!(status, AudioEngineManualRenderingStatus::Success);
    let Some(PCMChannelData::Deinterleaved(channels)) = output.channel_data::<f32>() else {
        panic!("manual rendering output should be deinterleaved float");
    };
    assert!(channels.iter().all(|channel| channel.len() == 512));
    assert!(channels
        .iter()
        .flat_map(|channel| channel.iter())
        .all(|sample| (sample - 0.5).abs() < 1e-6));
    Ok(())
}

#[test]
fn manual_rendering_input_with_the_wrong_format_is_not_rendered(
) -> Result<(), Box<dyn std::error::Error>> {
    let (status, output) = render_manual_input(1)?;
    assert_eq!(
        status,
        AudioEngineManualRenderingStatus::InsufficientDataFromInputNode
    );
    assert_eq!(output.frame_length()?, 0);
    Ok(())
}

#[test]
fn rejected_input_callbacks_are_released_exactly_once() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let input = engine.input_node()?;
    let format = AudioFormat::standard(48_000.0, 2, false)?;
    let marker = Arc::new(());

    let held = Arc::clone(&marker);
    assert!(
        !input.set_manual_rendering_input_pcm_format_with_callback(&format, move |_| {
            let _ = &held;
            None
        })
    );
    assert_eq!(Arc::strong_count(&marker), 1);

    let held = Arc::clone(&marker);
    assert!(!input.set_muted_speech_activity_event_listener(move |_| {
        let _ = &held;
    }));
    assert_eq!(Arc::strong_count(&marker), 1);
    Ok(())
}
