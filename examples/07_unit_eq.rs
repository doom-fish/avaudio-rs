#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let eq = AudioUnitEQ::new(2)?;
    engine.attach_node(&eq);
    engine.connect_node_to_main_mixer(&eq, None);

    eq.set_global_gain(1.5);
    let band = eq.band_info(0)?;
    let params = AudioEQBandParams {
        filter_type: band.filter_type,
        frequency: 1_000.0,
        bandwidth: 1.0,
        gain: 4.0,
        bypass: false,
    };
    eq.set_band_params(0, &params)?;

    println!("global gain: {:.2}", eq.global_gain());
    println!("band count: {}", eq.band_count());
    println!("band 0: {:?}", eq.band_info(0)?);
    Ok(())
}
