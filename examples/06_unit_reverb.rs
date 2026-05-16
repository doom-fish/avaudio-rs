#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let reverb = AudioUnitReverb::new()?;
    engine.attach_node(&reverb);
    engine.connect_node_to_main_mixer(&reverb, None);

    reverb.load_factory_preset(AudioUnitReverbPreset::Cathedral);
    reverb.set_wet_dry_mix(35.0);

    println!("wet/dry mix: {:.2}", reverb.wet_dry_mix());
    println!("preset: {:?}", AudioUnitReverbPreset::Cathedral);
    Ok(())
}
