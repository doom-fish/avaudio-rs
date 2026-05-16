use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let sequencer = AudioSequencer::with_engine(&engine)?;
    sequencer.set_rate(1.0);
    sequencer.set_user_callback(|event| {
        println!("user event at beat {} ({} bytes)", event.beat, event.bytes.len());
    })?;

    let info = sequencer.info()?;
    println!(
        "sequencer: tracks={}, seconds={}, beats={}, playing={}",
        info.track_count, info.current_position_in_seconds, info.current_position_in_beats, info.is_playing
    );

    sequencer.clear_user_callback()?;
    Ok(())
}
