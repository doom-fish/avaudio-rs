mod common;

use avaudio::prelude::*;

#[test]
fn output_node_format() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let output = engine.output_node()?;

    assert!(output.output_format(0)?.sample_rate >= 0.0);
    Ok(())
}

#[test]
fn output_node_rejects_buses_it_does_not_have() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let output = engine.output_node()?;
    for bus in [1, usize::MAX] {
        assert!(matches!(
            output.output_format(bus),
            Err(AVAudioError::OperationFailed(message)) if message.contains("out of range")
        ));
    }
    Ok(())
}
