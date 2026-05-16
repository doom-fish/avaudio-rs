mod common;

use avaudio::prelude::*;

#[test]
fn mixer_node_create_and_volume() -> Result<(), Box<dyn std::error::Error>> {
    let mixer = AudioMixerNode::new()?;
    mixer.set_output_volume(0.25);
    assert!((mixer.output_volume() - 0.25).abs() < 0.001);
    Ok(())
}
