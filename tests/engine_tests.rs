mod common;

use avaudio::prelude::*;

#[test]
fn engine_graph_extensions_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    let time_pitch = AudioUnitTimePitch::new()?;
    let _main_mixer = engine.main_mixer_node()?;
    let _input = engine.input_node()?;
    let _output = engine.output_node()?;

    engine.attach_node(&player)?;
    engine.attach_node(&time_pitch)?;
    engine.connect_nodes(&player, &time_pitch, None)?;
    engine.connect_node_to_main_mixer(&time_pitch, None)?;

    assert!(!engine.is_running()?);
    Ok(())
}

#[test]
#[ignore = "requires audio output hardware"]
fn engine_start_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    engine.prepare()?;
    engine.start()?;
    assert!(engine.is_running()?);
    engine.stop();
    Ok(())
}

#[test]
fn graph_misuse_returns_errors_instead_of_aborting() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    assert!(matches!(
        engine.prepare(),
        Err(AVAudioError::EngineError(_))
    ));
    assert!(matches!(engine.start(), Err(AVAudioError::EngineError(_))));

    let player = AudioPlayerNode::new()?;
    assert!(matches!(
        engine.connect_node_to_main_mixer(&player, None),
        Err(AVAudioError::EngineError(_))
    ));
    assert!(matches!(player.play(), Err(AVAudioError::PlayerError(_))));

    engine.attach_node(&player)?;
    engine.attach_node(&player)?;
    let other = AudioEngine::new()?;
    assert!(matches!(
        other.attach_node(&player),
        Err(AVAudioError::EngineError(_))
    ));

    let detached_mixer = AudioMixerNode::new()?;
    assert!(matches!(
        engine.connect_nodes(&player, &detached_mixer, None),
        Err(AVAudioError::EngineError(_))
    ));

    engine.connect_node_to_main_mixer(&player, None)?;
    engine.prepare()?;
    assert!(!engine.is_running()?);
    Ok(())
}
