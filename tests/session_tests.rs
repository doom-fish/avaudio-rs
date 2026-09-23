mod common;

use avaudio::prelude::*;

#[test]
fn session_queries_report_that_avaudiosession_is_unavailable() {
    assert!(matches!(
        AudioSession::sample_rate(),
        Err(AVAudioError::Unsupported(_))
    ));
    assert!(matches!(
        AudioSession::output_volume(),
        Err(AVAudioError::Unsupported(_))
    ));
    assert!(matches!(
        AudioSession::is_other_audio_playing(),
        Err(AVAudioError::Unsupported(_))
    ));
}
