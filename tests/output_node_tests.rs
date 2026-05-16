mod common;

use avaudio::prelude::*;

#[test]
fn output_node_format() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let output = engine.output_node()?;

    assert!(output.output_format(0)?.sample_rate >= 0.0);
    Ok(())
}
