mod common;

use avaudio::prelude::*;

#[test]
fn environment_node_properties() -> Result<(), Box<dyn std::error::Error>> {
    let environment = AudioEnvironmentNode::new()?;
    environment.set_listener_position(1.0, 2.0, 3.0);
    environment.set_listener_orientation(10.0, 20.0, 30.0);
    environment.set_distance_attenuation(1, 1.0, 20.0, 0.75)?;
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

#[test]
fn distance_attenuation_rejects_unknown_models() -> Result<(), Box<dyn std::error::Error>> {
    let environment = AudioEnvironmentNode::new()?;
    environment.set_distance_attenuation(3, 1.0, 20.0, 0.75)?;
    for model in [0, 4, 99] {
        assert!(matches!(
            environment.set_distance_attenuation(model, 2.0, 30.0, 0.5),
            Err(AVAudioError::InvalidArgument(_))
        ));
    }
    let attenuation = environment.distance_attenuation()?;
    assert_eq!(attenuation.model, 3);
    assert!((attenuation.reference_distance - 1.0).abs() < f32::EPSILON);
    assert!((attenuation.maximum_distance - 20.0).abs() < f32::EPSILON);
    Ok(())
}
