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
    assert!(info.has_tempo_track);
    assert!((sequencer.rate()? - 1.25).abs() < 0.001);
    assert!(sequencer.current_position_in_seconds()? >= 0.0);
    assert!(sequencer.current_position_in_beats()? >= 0.0);

    let seconds = sequencer.seconds_for_beats(1.0);
    assert!(seconds >= 0.0);
    let beats = sequencer.beats_for_seconds(seconds);
    assert!(beats >= 0.0);

    let info_keys = AudioSequencer::info_dictionary_keys()?;
    assert!(!info_keys.title.is_empty());
    assert!(sequencer
        .user_info()?
        .keys()
        .all(|key| !key.is_empty()));

    sequencer.set_user_callback(|event| {
        let _ = event.track_ptr;
        let _ = event.bytes.len();
        let _ = event.beat;
    })?;
    sequencer.clear_user_callback()?;
    Ok(())
}

#[test]
fn sequencer_track_event_surface() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let sequencer = AudioSequencer::with_engine(&engine)?;

    let tempo_track = sequencer.tempo_track()?.expect("tempo track");
    tempo_track.add_event(&MusicEvent::ExtendedTempo(ExtendedTempoEvent::new(96.0)), 0.0)?;
    assert_eq!(tempo_track.events_in_range(BeatRange::new(0.0, 1.0))?.len(), 1);

    let track = sequencer.create_and_append_track()?;
    track.set_loop_range(BeatRange::new(1.0, 4.0));
    track.set_looping_enabled(true);
    track.set_number_of_loops(2);
    track.set_offset_time(0.5);
    track.set_muted(true);
    track.set_soloed(true);
    track.set_length_in_beats(8.0);
    track.set_length_in_seconds(4.0);

    let sampler = AudioUnitSampler::new()?;
    track.set_destination_audio_unit(Some(&sampler));
    assert!(track.destination_audio_unit().is_some());
    track.set_uses_automated_parameters(false)?;

    track.add_event(&MusicEvent::MidiNote(MidiNoteEvent::new(0, 60, 100, 1.0)), 0.0)?;
    track.add_event(&MusicEvent::MusicUser(MusicUserEvent::new([1_u8, 2, 3])), 2.0)?;

    let saw_user_event = std::rc::Rc::new(std::cell::Cell::new(false));
    let saw_user_event_for_callback = std::rc::Rc::clone(&saw_user_event);
    track.enumerate_events_in_range(BeatRange::new(0.0, 4.0), move |event| {
        if matches!(event.event, MusicEvent::MusicUser(_)) {
            saw_user_event_for_callback.set(true);
            TrackEventAction {
                new_beat: Some(event.beat + 1.0),
                remove: false,
            }
        } else {
            TrackEventAction::default()
        }
    })?;
    assert!(saw_user_event.get());

    let events = track.events_in_range(BeatRange::new(0.0, 8.0))?;
    assert_eq!(events.len(), 2);
    assert!(events.iter().any(|event| matches!(event.event, MusicEvent::MusicUser(_)) && (event.beat - 3.0).abs() < 0.001));

    let copied_track = sequencer.create_and_append_track()?;
    copied_track.copy_events_in_range(BeatRange::new(0.0, 8.0), &track, 0.0)?;
    assert_eq!(copied_track.events_in_range(BeatRange::new(0.0, 8.0))?.len(), 2);
    copied_track.cut_events_in_range(BeatRange::new(0.0, 8.0))?;
    assert!(copied_track.events_in_range(BeatRange::new(0.0, 8.0))?.is_empty());

    let data = sequencer.data_with_smpte_resolution(480)?;
    assert!(!data.is_empty());

    let artifact_dir = std::path::PathBuf::from("target/test-artifacts");
    std::fs::create_dir_all(&artifact_dir)?;
    let midi_path = artifact_dir.join("sequencer_track_event_surface.mid");
    if midi_path.exists() {
        std::fs::remove_file(&midi_path)?;
    }
    sequencer.write_to_path(&midi_path, 480, true)?;
    assert!(midi_path.exists());

    let loaded = AudioSequencer::with_engine(&engine)?;
    loaded.load_from_data(&data)?;
    assert!(loaded.track_count()? >= 1);

    let loaded_from_path = AudioSequencer::with_engine(&engine)?;
    loaded_from_path.load_from_path(&midi_path)?;
    assert!(loaded_from_path.track_count()? >= 1);

    sequencer.remove_track(copied_track)?;
    assert_eq!(sequencer.track_count()?, 1);

    std::fs::remove_file(midi_path)?;
    Ok(())
}
