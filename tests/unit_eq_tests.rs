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

#[test]
fn eq_band_rejects_unknown_filter_types() -> Result<(), Box<dyn std::error::Error>> {
    let eq = AudioUnitEQ::new(1)?;
    let params = AudioEQBandParams {
        filter_type: 7,
        frequency: 440.0,
        bandwidth: 1.0,
        gain: 1.5,
        bypass: false,
    };
    eq.set_band_params(0, &params)?;
    for filter_type in [-1, 11, 99] {
        let invalid = AudioEQBandParams {
            filter_type,
            frequency: 1_000.0,
            ..params.clone()
        };
        assert!(matches!(
            eq.set_band_params(0, &invalid),
            Err(AVAudioError::InvalidArgument(_))
        ));
    }
    let info = eq.band_info(0)?;
    assert_eq!(info.filter_type, 7);
    assert!((info.frequency - 440.0).abs() < 0.001);
    Ok(())
}
