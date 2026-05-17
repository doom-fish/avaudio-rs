use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let sequencer = AudioSequencer::with_engine(&engine)?;
    sequencer.set_rate(1.0);
    sequencer.set_user_callback(|event| {
        println!(
            "user event at beat {} ({} bytes)",
            event.beat,
            event.bytes.len()
        );
    })?;

    let info_keys = AudioSequencer::info_dictionary_keys()?;
    println!("sequencer info key for title: {}", info_keys.title);

    let tempo_track = sequencer.tempo_track()?.expect("tempo track");
    tempo_track.add_event(
        &MusicEvent::ExtendedTempo(ExtendedTempoEvent::new(120.0)),
        0.0,
    )?;

    let track = sequencer.create_and_append_track()?;
    track.add_event(
        &MusicEvent::MidiNote(MidiNoteEvent::new(0, 60, 96, 1.0)),
        0.0,
    )?;
    track.add_event(
        &MusicEvent::MusicUser(MusicUserEvent::new([1_u8, 2, 3, 4])),
        2.0,
    )?;

    let info = sequencer.info()?;
    println!(
        "sequencer: tracks={}, has_tempo_track={}, seconds={}, beats={}, playing={}",
        info.track_count,
        info.has_tempo_track,
        info.current_position_in_seconds,
        info.current_position_in_beats,
        info.is_playing
    );

    for event in track.events_in_range(BeatRange::new(0.0, 8.0))? {
        println!("track event at beat {} => {:?}", event.beat, event.event);
    }

    let data = sequencer.data_with_smpte_resolution(480)?;
    println!("serialized sequence data: {} bytes", data.len());

    sequencer.clear_user_callback()?;
    Ok(())
}
