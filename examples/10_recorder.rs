#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;
use support::{artifacts_dir, print_skip, short_sleep};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recording_path = artifacts_dir()?.join("10-recorder.caf");
    let recorder = AudioRecorder::create(&recording_path, 44_100.0, 1, 16)?;
    recorder.set_metering_enabled(true);
    println!("current time before record: {:.2}", recorder.current_time());

    if !recorder.record() {
        print_skip("microphone access unavailable; skipping actual recording");
        return Ok(());
    }

    short_sleep();
    recorder.update_meters();
    println!("average power: {:.2}", recorder.average_power(0)?);
    println!("peak power: {:.2}", recorder.peak_power(0)?);
    recorder.pause();
    recorder.stop();
    let _deleted = recorder.delete_recording();
    Ok(())
}
