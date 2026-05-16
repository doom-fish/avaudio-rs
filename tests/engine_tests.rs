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

    engine.attach_node(&player);
    engine.attach_node(&time_pitch);
    engine.connect_nodes(&player, &time_pitch, None);
    engine.connect_node_to_main_mixer(&time_pitch, None);

    assert!(!engine.is_running()?);
    Ok(())
}

#[test]
#[ignore = "requires audio output hardware"]
fn engine_start_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    engine.prepare();
    engine.start()?;
    assert!(engine.is_running()?);
    engine.stop();
    Ok(())
}
