mod support;

use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let delay = AudioUnitDelay::new()?;
    delay.set_delay_time(0.25);
    delay.set_feedback(18.0);

    let distortion = AudioUnitDistortion::new()?;
    distortion.load_factory_preset(AudioUnitDistortionPreset::SpeechGoldenPi);
    distortion.set_wet_dry_mix(35.0);

    let sampler = AudioUnitSampler::new()?;
    let artifacts = support::artifacts_dir()?;
    let sample_path = artifacts.join("example-sampler.aiff");
    support::make_test_audio(&sample_path)?;
    sampler.load_instrument(&sample_path)?;
    sampler.set_overall_gain(2.0);

    let varispeed = AudioUnitVarispeed::new()?;
    varispeed.set_rate(1.1);

    println!(
        "delay={}, distortion mix={}, sampler gain={}, varispeed rate={}",
        delay.delay_time(),
        distortion.wet_dry_mix(),
        sampler.overall_gain(),
        varispeed.rate()
    );
    Ok(())
}
