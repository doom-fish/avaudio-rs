mod common;

use std::sync::mpsc;
use std::time::Duration;

use avaudio::prelude::*;

#[test]
fn converter_prime_and_status_surfaces() -> Result<(), Box<dyn std::error::Error>> {
    let input_format = AudioFormat::standard(48_000.0, 1, false)?;
    let output_format = AudioFormat::standard(24_000.0, 1, false)?;
    let converter = AudioConverter::new(&input_format, &output_format)?;
    let mut input = PCMBuffer::new(&input_format, 480)?;
    input.set_frame_length(480)?;
    let mut output = PCMBuffer::new(&output_format, 512)?;

    converter.reset();
    converter.set_prime_method(AudioConverterPrimeMethod::Pre);
    assert_eq!(converter.prime_method(), AudioConverterPrimeMethod::Pre);

    let prime_info = AudioConverterPrimeInfo {
        leading_frames: 4,
        trailing_frames: 8,
    };
    converter.set_prime_info(prime_info);
    let observed_prime_info = converter.prime_info()?;
    assert!(observed_prime_info.leading_frames > 0);
    assert!(observed_prime_info.trailing_frames > 0);

    let status = converter.convert_buffer_status(&input, &mut output)?;
    assert!(matches!(
        status,
        AudioConverterOutputStatus::HaveData
            | AudioConverterOutputStatus::InputRanDry
            | AudioConverterOutputStatus::EndOfStream
    ));
    let _ = AudioConverterInputStatus::HaveData;
    Ok(())
}

#[test]
fn engine_manual_rendering_surfaces() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let format = AudioFormat::standard(44_100.0, 1, false)?;
    let mut buffer = PCMBuffer::new(&format, 256)?;

    engine.enable_manual_rendering_mode(AudioEngineManualRenderingMode::Realtime, &format, 256)?;
    assert!(engine.is_in_manual_rendering_mode()?);
    assert_eq!(
        engine.manual_rendering_mode()?,
        AudioEngineManualRenderingMode::Realtime
    );
    assert_eq!(engine.manual_rendering_maximum_frame_count()?, 256);
    assert!(
        (engine.manual_rendering_format()?.info()?.sample_rate - 44_100.0).abs() < f64::EPSILON
    );
    let status = engine.manual_rendering_block_render(128, &mut buffer)?;
    assert!(matches!(
        status,
        AudioEngineManualRenderingStatus::Error
            | AudioEngineManualRenderingStatus::Success
            | AudioEngineManualRenderingStatus::InsufficientDataFromInputNode
            | AudioEngineManualRenderingStatus::CannotDoInCurrentContext
    ));
    assert_eq!(
        AudioEngine::configuration_change_notification_name()?,
        "AVAudioEngineConfigurationChangeNotification"
    );
    assert!(matches!(
        AudioEngineManualRenderingError::from_raw(-80_802),
        AudioEngineManualRenderingError::NotRunning
    ));
    engine.disable_manual_rendering_mode();
    Ok(())
}

#[test]
fn mixing_and_io_surfaces() -> Result<(), Box<dyn std::error::Error>> {
    let player = AudioPlayerNode::new()?;
    player.set_volume(0.5)?;
    player.set_pan(0.25)?;
    player.set_rendering_algorithm(Audio3DMixingRenderingAlgorithm::Hrtf)?;
    player.set_source_mode(Audio3DMixingSourceMode::PointSource)?;
    player.set_point_source_in_head_mode(Audio3DMixingPointSourceInHeadMode::Bypass)?;
    player.set_rate(1.2)?;
    player.set_reverb_blend(0.3)?;
    player.set_obstruction(-1.0)?;
    player.set_occlusion(-2.0)?;
    player.set_position(Audio3DVector::new(1.0, 2.0, 3.0))?;

    assert!((player.volume()? - 0.5).abs() < 0.001);
    assert!((player.pan()? - 0.25).abs() < 0.001);
    assert_eq!(
        player.rendering_algorithm()?,
        Audio3DMixingRenderingAlgorithm::Hrtf
    );
    assert_eq!(player.source_mode()?, Audio3DMixingSourceMode::PointSource);
    assert_eq!(
        player.point_source_in_head_mode()?,
        Audio3DMixingPointSourceInHeadMode::Bypass
    );
    assert!((player.rate()? - 1.2).abs() < 0.001);
    assert!((player.reverb_blend()? - 0.3).abs() < 0.001);
    assert!((player.obstruction()? + 1.0).abs() < 0.001);
    assert!((player.occlusion()? + 2.0).abs() < 0.001);
    assert_eq!(player.position()?, Audio3DVector::new(1.0, 2.0, 3.0));

    let engine = AudioEngine::new()?;
    let main_mixer = engine.main_mixer_node()?;
    engine.attach_node(&player);
    engine.connect_node_to_main_mixer(&player, None);
    let destination = player
        .destination_for_mixer(&main_mixer, 0)?
        .expect("connected player should vend a mixing destination");
    destination.set_volume(0.4)?;
    destination.set_pan(-0.2)?;
    assert!((destination.volume()? - 0.4).abs() < 0.001);
    assert!((destination.pan()? + 0.2).abs() < 0.001);
    assert_eq!(destination.connection_point()?.bus()?, 0);

    let format = AudioFormat::standard(44_100.0, 1, false)?;
    let input = engine.input_node()?;
    let output = engine.output_node()?;
    assert!(input.presentation_latency()? >= 0.0);
    assert!(output.presentation_latency()? >= 0.0);

    engine.enable_manual_rendering_mode(AudioEngineManualRenderingMode::Realtime, &format, 256)?;
    assert!(input.set_manual_rendering_input_pcm_format_with_callback(&format, |_| None));
    engine.disable_manual_rendering_mode();

    assert!(!input.is_voice_processing_enabled()?);
    input.set_voice_processing_enabled(true)?;
    assert!(input.is_voice_processing_enabled()?);
    assert!(output.is_voice_processing_enabled()?);
    input.set_voice_processing_bypassed(false);
    input.set_voice_processing_agc_enabled(true);
    input.set_voice_processing_input_muted(false);
    assert!(input.set_muted_speech_activity_event_listener(|_| {}));
    assert!(input.clear_muted_speech_activity_event_listener());
    let configuration = AudioVoiceProcessingOtherAudioDuckingConfiguration::new(
        true,
        AudioVoiceProcessingOtherAudioDuckingLevel::Mid,
    );
    input.set_voice_processing_other_audio_ducking_configuration(configuration)?;
    assert_eq!(
        input.voice_processing_other_audio_ducking_configuration()?,
        configuration
    );
    let _ = AudioVoiceProcessingSpeechActivityEvent::Started;
    input.set_voice_processing_enabled(false)?;
    assert!(!output.is_voice_processing_enabled()?);
    Ok(())
}

#[test]
fn player_delegate_and_typed_completion_surfaces() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = common::artifacts_dir()?.join("gap-closure-player.aiff");
    common::make_test_audio(&audio_path)?;

    let simple_player = AudioSimplePlayer::create_from_path(&audio_path)?;
    simple_player.set_delegate(
        AudioSimplePlayerDelegate::new()
            .on_finish_playing(|_| {})
            .on_decode_error(|_| {}),
    )?;
    simple_player.clear_delegate();

    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    let format = AudioFormat::standard(48_000.0, 1, false)?;
    let mut buffer = PCMBuffer::new(&format, 256)?;
    buffer.set_frame_length(128)?;
    engine.attach_node(&player);
    engine.connect_node_to_main_mixer(&player, Some(&format));
    let options =
        AudioPlayerNodeBufferOptions::LOOPS | AudioPlayerNodeBufferOptions::INTERRUPTS_AT_LOOP;
    assert_eq!(options.bits(), 5);
    player.schedule_buffer_with_options(&buffer, None, options)?;
    let _ = AudioPlayerNodeCompletionCallbackType::DataConsumed;
    let _ = AudioPlayerNodeCompletionCallbackType::DataRendered;
    Ok(())
}

#[test]
fn recorder_delegate_and_routing_surfaces() -> Result<(), Box<dyn std::error::Error>> {
    let recording_path = common::artifacts_dir()?.join("gap-closure-recorder.caf");
    let recorder = AudioRecorder::create(&recording_path, 44_100.0, 1, 16)?;
    recorder.set_delegate(
        AudioRecorderDelegate::new()
            .on_finish_recording(|_| {})
            .on_encode_error(|_| {}),
    )?;
    recorder.clear_delegate();

    let (tx, rx) = mpsc::channel();
    AudioRoutingArbiter::shared().begin(
        AudioRoutingArbitrationCategory::Playback,
        move |changed, error| {
            let _ = tx.send((changed, error));
        },
    )?;
    let (_changed, error) = rx.recv_timeout(Duration::from_secs(5))?;
    assert!(error.is_none());
    AudioRoutingArbiter::shared().leave();
    Ok(())
}

#[test]
fn session_capability_smoke() {
    let capability = AudioSessionCapability::new();
    assert!(!capability.is_enabled() || capability.is_supported());
}
