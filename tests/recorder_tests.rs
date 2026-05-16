mod common;

use avaudio::prelude::*;

#[test]
fn recorder_surface() -> Result<(), Box<dyn std::error::Error>> {
    let recording_path = common::artifacts_dir()?.join("recorder-surface.caf");
    let recorder = AudioRecorder::create(&recording_path, 44_100.0, 1, 16)?;
    recorder.set_metering_enabled(true);
    recorder.update_meters();
    assert!(recorder.current_time() >= 0.0);
    let _avg = recorder.average_power(0)?;
    let _peak = recorder.peak_power(0)?;
    Ok(())
}

#[test]
#[ignore = "requires microphone permission and input hardware"]
fn recorder_recording() -> Result<(), Box<dyn std::error::Error>> {
    let recording_path = common::artifacts_dir()?.join("recorder-recording.caf");
    let recorder = AudioRecorder::create(&recording_path, 44_100.0, 1, 16)?;
    assert!(recorder.record());
    recorder.stop();
    Ok(())
}
