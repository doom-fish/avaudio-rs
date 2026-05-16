mod common;

use avaudio::prelude::*;

#[test]
fn session_queries() {
    assert!(AudioSession::sample_rate() >= 0.0);
    assert!(AudioSession::output_volume() >= 0.0);
    let _ = AudioSession::is_other_audio_playing();
}
