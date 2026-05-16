mod common;

use avaudio::prelude::*;

#[test]
fn audio_unit_variant_properties() -> Result<(), Box<dyn std::error::Error>> {
    let delay = AudioUnitDelay::new()?;
    delay.set_delay_time(0.5);
    delay.set_feedback(25.0);
    delay.set_low_pass_cutoff(8_000.0);
    delay.set_wet_dry_mix(40.0);
    assert!((delay.delay_time() - 0.5).abs() < 0.001);
    assert!((delay.feedback() - 25.0).abs() < 0.001);
    assert!((delay.low_pass_cutoff() - 8_000.0).abs() < 0.001);
    assert!((delay.wet_dry_mix() - 40.0).abs() < 0.001);

    let distortion = AudioUnitDistortion::new()?;
    distortion.load_factory_preset(AudioUnitDistortionPreset::SpeechWaves);
    distortion.set_pre_gain(-3.0);
    distortion.set_wet_dry_mix(60.0);
    assert!((distortion.pre_gain() + 3.0).abs() < 0.001);
    assert!((distortion.wet_dry_mix() - 60.0).abs() < 0.001);

    let sampler = AudioUnitSampler::new()?;
    sampler.set_stereo_pan(-12.0);
    sampler.set_overall_gain(3.0);
    sampler.set_global_tuning(12.0);
    assert!((sampler.stereo_pan() + 12.0).abs() < 0.001);
    assert!((sampler.overall_gain() - 3.0).abs() < 0.001);
    assert!((sampler.global_tuning() - 12.0).abs() < 0.001);

    let varispeed = AudioUnitVarispeed::new()?;
    varispeed.set_rate(1.5);
    assert!((varispeed.rate() - 1.5).abs() < 0.001);
    Ok(())
}
