mod common;

use avaudio::prelude::*;

#[test]
fn time_pitch_properties() -> Result<(), Box<dyn std::error::Error>> {
    let unit = AudioUnitTimePitch::new()?;
    unit.set_pitch(200.0);
    unit.set_rate(1.1);
    unit.set_overlap(12.0);

    assert!((unit.pitch() - 200.0).abs() < 0.001);
    assert!((unit.rate() - 1.1).abs() < 0.001);
    assert!((unit.overlap() - 12.0).abs() < 0.001);
    Ok(())
}
