#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let environment = AudioEnvironmentNode::new()?;
    engine.attach_node(&environment);
    engine.connect_node_to_main_mixer(&environment, None);

    environment.set_listener_position(1.0, 2.0, -3.0);
    environment.set_listener_orientation(15.0, 5.0, 0.0);
    environment.set_distance_attenuation(1, 1.0, 25.0, 0.7);
    environment.set_reverb_blend(20.0);

    println!("listener position: {:?}", environment.listener_position()?);
    println!(
        "listener orientation: {:?}",
        environment.listener_orientation()?
    );
    println!(
        "distance attenuation: {:?}",
        environment.distance_attenuation()?
    );
    println!("reverb blend: {:.2}", environment.reverb_blend());
    Ok(())
}
