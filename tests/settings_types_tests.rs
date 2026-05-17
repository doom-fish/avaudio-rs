use avaudio::prelude::*;

#[test]
fn settings_and_type_surfaces() -> Result<(), Box<dyn std::error::Error>> {
    let constants = AudioSettingsConstants::current()?;
    assert!(!constants.audio_file_type_key.is_empty());
    assert!(!constants.bit_rate_strategy_constant.is_empty());
    assert!(!constants.bit_rate_strategy_long_term_average.is_empty());
    assert!(!constants.bit_rate_strategy_variable.is_empty());
    assert!(!constants.bit_rate_strategy_variable_constrained.is_empty());

    assert_eq!(AudioQuality::High as i64, 0x60);
    assert_eq!(AudioDynamicRangeControlConfiguration::Movie as i64, 3);
    assert_eq!(AudioContentSource::Passthrough as i64, 42);

    let vector = Audio3DVector::new(1.0, 2.0, 3.0);
    let orientation = Audio3DVectorOrientation::new(vector, Audio3DVector::new(0.0, 1.0, 0.0));
    assert_eq!(orientation.forward, vector);
    assert_eq!(Audio3DMixingRenderingAlgorithm::Auto as i64, 7);
    assert_eq!(Audio3DMixingSourceMode::AmbienceBed as i64, 3);
    assert_eq!(Audio3DMixingPointSourceInHeadMode::Bypass as i64, 1);
    assert_eq!(AudioEnvironmentOutputType::ExternalSpeakers as i64, 3);

    assert_eq!(AudioSessionActivationOptions::NONE.bits(), 0);
    assert_eq!(AudioSessionInterruptionOptions::SHOULD_RESUME.bits(), 1);
    assert_eq!(
        AudioSessionSetActiveOptions::NOTIFY_OTHERS_ON_DEACTIVATION.bits(),
        1
    );
    assert_eq!(AudioSessionIOType::Aggregated as u64, 1);
    assert_eq!(AudioSessionInterruptionType::Began as u64, 1);
    assert_eq!(
        AudioSessionRouteChangeReason::RouteConfigurationChange as u64,
        8
    );
    assert_eq!(AudioStereoOrientation::LandscapeLeft as i64, 4);
    assert_eq!(AudioSessionRenderingMode::DolbyAtmos as i64, 5);
    assert_eq!(AudioSessionMicrophoneInjectionMode::SpokenAudio as i64, 1);
    assert_eq!(AudioSessionSoundStageSize::Large as i64, 3);
    assert_eq!(AudioSessionAnchoringStrategy::Front as i64, 2);
    assert_eq!(AudioSessionSpatialExperience::Bypassed as i64, 2);
    assert_eq!(AudioSessionSilenceSecondaryAudioHintType::Begin as u64, 1);
    assert_eq!(AudioSessionPromptStyle::Short as i64, 0x7368_7274);

    Ok(())
}
