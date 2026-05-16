use avaudio::prelude::*;

#[test]
fn audio_unit_bypass_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let reverb = AudioUnitReverb::new()?;
    reverb.set_bypass(true);
    assert!(reverb.bypass()?);
    reverb.set_bypass(false);
    assert!(!reverb.bypass()?);

    let time_pitch = AudioUnitTimePitch::new()?;
    time_pitch.set_bypass(true);
    assert!(time_pitch.bypass()?);
    time_pitch.set_bypass(false);
    assert!(!time_pitch.bypass()?);
    Ok(())
}
