#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let time_pitch = AudioUnitTimePitch::new()?;
    engine.attach_node(&time_pitch);
    engine.connect_node_to_main_mixer(&time_pitch, None);

    time_pitch.set_pitch(300.0);
    time_pitch.set_rate(1.2);
    time_pitch.set_overlap(12.0);

    println!("pitch: {:.2}", time_pitch.pitch());
    println!("rate: {:.2}", time_pitch.rate());
    println!("overlap: {:.2}", time_pitch.overlap());
    Ok(())
}
