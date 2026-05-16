use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reverb = AudioUnitReverb::new()?;
    reverb.set_bypass(true);
    println!("reverb bypass: {}", reverb.bypass()?);

    let time_pitch = AudioUnitTimePitch::new()?;
    time_pitch.set_bypass(true);
    println!("time-pitch bypass: {}", time_pitch.bypass()?);
    Ok(())
}
