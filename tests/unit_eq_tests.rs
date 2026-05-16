mod common;

use avaudio::prelude::*;

#[test]
fn eq_bands_and_global_gain() -> Result<(), Box<dyn std::error::Error>> {
    let eq = AudioUnitEQ::new(2)?;
    eq.set_global_gain(2.0);
    assert_eq!(eq.band_count(), 2);
    assert!((eq.global_gain() - 2.0).abs() < 0.001);

    let info = eq.band_info(0)?;
    let params = AudioEQBandParams {
        filter_type: info.filter_type,
        frequency: 880.0,
        bandwidth: 1.0,
        gain: 3.5,
        bypass: false,
    };
    eq.set_band_params(0, &params)?;
    let updated = eq.band_info(0)?;
    assert!((updated.frequency - 880.0).abs() < 0.001);
    Ok(())
}
