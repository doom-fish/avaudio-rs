mod common;

use avaudio::prelude::*;

#[test]
fn reverb_properties() -> Result<(), Box<dyn std::error::Error>> {
    let reverb = AudioUnitReverb::new()?;
    reverb.load_factory_preset(AudioUnitReverbPreset::LargeHall);
    reverb.set_wet_dry_mix(40.0);

    assert!((reverb.wet_dry_mix() - 40.0).abs() < 0.001);
    Ok(())
}
