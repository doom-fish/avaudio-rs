mod common;

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
