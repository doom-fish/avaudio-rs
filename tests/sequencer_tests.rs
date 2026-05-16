mod common;

use avaudio::prelude::*;

#[test]
fn sequencer_transport_surface() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let sequencer = AudioSequencer::with_engine(&engine)?;

    sequencer.set_rate(1.25);
    sequencer.set_current_position_in_seconds(1.5);
    sequencer.set_current_position_in_beats(2.0);

    let info = sequencer.info()?;
    assert_eq!(info.track_count, 0);
    assert!((sequencer.rate()? - 1.25).abs() < 0.001);
    assert!(sequencer.current_position_in_seconds()? >= 0.0);
    assert!(sequencer.current_position_in_beats()? >= 0.0);

    let seconds = sequencer.seconds_for_beats(1.0);
    assert!(seconds >= 0.0);
    let beats = sequencer.beats_for_seconds(seconds);
    assert!(beats >= 0.0);

    sequencer.set_user_callback(|event| {
        let _ = event.track_ptr;
        let _ = event.bytes.len();
        let _ = event.beat;
    })?;
    sequencer.clear_user_callback()?;
    Ok(())
}
