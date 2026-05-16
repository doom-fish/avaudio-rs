mod common;

use avaudio::prelude::*;

#[test]
fn environment_node_properties() -> Result<(), Box<dyn std::error::Error>> {
    let environment = AudioEnvironmentNode::new()?;
    environment.set_listener_position(1.0, 2.0, 3.0);
    environment.set_listener_orientation(10.0, 20.0, 30.0);
    environment.set_distance_attenuation(1, 1.0, 20.0, 0.75);
    environment.set_reverb_blend(15.0);

    let position = environment.listener_position()?;
    let orientation = environment.listener_orientation()?;
    let attenuation = environment.distance_attenuation()?;

    assert!((position.x - 1.0).abs() < f32::EPSILON);
    assert!((orientation.roll - 30.0).abs() < f32::EPSILON);
    assert_eq!(attenuation.model, 1);
    assert!((environment.reverb_blend() - 15.0).abs() < 0.001);
    Ok(())
}
