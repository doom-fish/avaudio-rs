//! `AVMusicTrack` wrappers and beat-range editing helpers.

#![allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools,
    clippy::type_complexity
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::cell::RefCell;
use std::ffi::CString;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::music_event::{event_from_json_ptr, event_to_json_cstring, MusicEvent};
use crate::unit::AudioUnit;
use crate::unit_effect::AudioUnitHandle;
use crate::util::parse_json_and_free;

/// `AVMusicTimeStampEndOfTrack`.
pub const MUSIC_TIME_STAMP_END_OF_TRACK: f64 = f64::MAX;
/// `AVMusicTrackLoopCountForever`.
pub const MUSIC_TRACK_LOOP_COUNT_FOREVER: i64 = -1;

/// Mirrors `AVBeatRange`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeatRange {
    pub start: f64,
    pub length: f64,
}

impl BeatRange {
    pub const fn new(start: f64, length: f64) -> Self {
        Self { start, length }
    }
}

/// Snapshot of `AVMusicTrack` state.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicTrackInfo {
    pub destination_midi_endpoint: u64,
    pub loop_range: BeatRange,
    pub looping_enabled: bool,
    pub number_of_loops: i64,
    pub offset_time: f64,
    pub muted: bool,
    pub soloed: bool,
    pub length_in_beats: f64,
    pub length_in_seconds: f64,
    pub time_resolution: usize,
    pub uses_automated_parameters: bool,
    pub has_destination_audio_unit: bool,
}

/// A concrete event at a specific beat within a track.
#[derive(Debug, Clone, PartialEq)]
pub struct TrackEvent {
    pub beat: f64,
    pub event: MusicEvent,
}

/// Mutation instructions for `enumerate_events_in_range`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TrackEventAction {
    pub new_beat: Option<f64>,
    pub remove: bool,
}

struct EnumerationState {
    callback: Box<dyn FnMut(TrackEvent) -> TrackEventAction + 'static>,
}

/// Wraps an `AVMusicTrack`.
pub struct MusicTrack {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for MusicTrack {}

impl Drop for MusicTrack {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_music_track_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl MusicTrack {
    pub(crate) const fn as_track_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Returns a snapshot of track state.
    pub fn info(&self) -> Result<MusicTrackInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_music_track_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns the destination audio unit when one is configured.
    pub fn destination_audio_unit(&self) -> Option<AudioUnit> {
        let ptr = unsafe { ffi::av_music_track_copy_destination_audio_unit(self.ptr) };
        (!ptr.is_null()).then_some(AudioUnit { ptr })
    }

    /// Sets the destination audio unit.
    pub fn set_destination_audio_unit<T: AudioUnitHandle>(&self, unit: Option<&T>) {
        unsafe {
            ffi::av_music_track_set_destination_audio_unit(
                self.ptr,
                unit.map_or(ptr::null_mut(), AudioUnitHandle::as_audio_unit_ptr),
            );
        };
    }

    /// Returns the destination MIDI endpoint.
    pub fn destination_midi_endpoint(&self) -> Result<u64, AVAudioError> {
        Ok(self.info()?.destination_midi_endpoint)
    }

    /// Sets the destination MIDI endpoint.
    pub fn set_destination_midi_endpoint(&self, endpoint: u64) {
        unsafe { ffi::av_music_track_set_destination_midi_endpoint(self.ptr, endpoint) };
    }

    /// Returns the loop range.
    pub fn loop_range(&self) -> Result<BeatRange, AVAudioError> {
        Ok(self.info()?.loop_range)
    }

    /// Sets the loop range.
    pub fn set_loop_range(&self, range: BeatRange) {
        unsafe { ffi::av_music_track_set_loop_range(self.ptr, range.start, range.length) };
    }

    /// Returns whether looping is enabled.
    pub fn is_looping_enabled(&self) -> Result<bool, AVAudioError> {
        Ok(self.info()?.looping_enabled)
    }

    /// Enables or disables looping.
    pub fn set_looping_enabled(&self, enabled: bool) {
        unsafe { ffi::av_music_track_set_looping_enabled(self.ptr, enabled) };
    }

    /// Returns the number of loops.
    pub fn number_of_loops(&self) -> Result<i64, AVAudioError> {
        Ok(self.info()?.number_of_loops)
    }

    /// Sets the number of loops.
    pub fn set_number_of_loops(&self, count: i64) {
        unsafe { ffi::av_music_track_set_number_of_loops(self.ptr, count) };
    }

    /// Returns the offset time.
    pub fn offset_time(&self) -> Result<f64, AVAudioError> {
        Ok(self.info()?.offset_time)
    }

    /// Sets the offset time.
    pub fn set_offset_time(&self, offset_time: f64) {
        unsafe { ffi::av_music_track_set_offset_time(self.ptr, offset_time) };
    }

    /// Returns whether the track is muted.
    pub fn is_muted(&self) -> Result<bool, AVAudioError> {
        Ok(self.info()?.muted)
    }

    /// Mutes or unmutes the track.
    pub fn set_muted(&self, muted: bool) {
        unsafe { ffi::av_music_track_set_muted(self.ptr, muted) };
    }

    /// Returns whether the track is soloed.
    pub fn is_soloed(&self) -> Result<bool, AVAudioError> {
        Ok(self.info()?.soloed)
    }

    /// Sets the solo state.
    pub fn set_soloed(&self, soloed: bool) {
        unsafe { ffi::av_music_track_set_soloed(self.ptr, soloed) };
    }

    /// Returns the length in beats.
    pub fn length_in_beats(&self) -> Result<f64, AVAudioError> {
        Ok(self.info()?.length_in_beats)
    }

    /// Sets the length in beats.
    pub fn set_length_in_beats(&self, length: f64) {
        unsafe { ffi::av_music_track_set_length_in_beats(self.ptr, length) };
    }

    /// Returns the length in seconds.
    pub fn length_in_seconds(&self) -> Result<f64, AVAudioError> {
        Ok(self.info()?.length_in_seconds)
    }

    /// Sets the length in seconds.
    pub fn set_length_in_seconds(&self, length: f64) {
        unsafe { ffi::av_music_track_set_length_in_seconds(self.ptr, length) };
    }

    /// Returns the MIDI PPQN resolution.
    pub fn time_resolution(&self) -> Result<usize, AVAudioError> {
        Ok(self.info()?.time_resolution)
    }

    /// Returns whether the track is an automation track.
    pub fn uses_automated_parameters(&self) -> Result<bool, AVAudioError> {
        Ok(self.info()?.uses_automated_parameters)
    }

    /// Sets the automation-track flag.
    pub fn set_uses_automated_parameters(&self, uses_automated_parameters: bool) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_music_track_set_uses_automated_parameters(
                self.ptr,
                uses_automated_parameters,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Adds an event at the given beat location.
    pub fn add_event(&self, event: &MusicEvent, beat: f64) -> Result<(), AVAudioError> {
        let json = event_to_json_cstring(event)?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_music_track_add_event_json(self.ptr, json.as_ptr(), beat, &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Moves all events in `range` by `beat_amount`.
    pub fn move_events_in_range(&self, range: BeatRange, beat_amount: f64) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_music_track_move_events_in_range(
                self.ptr,
                range.start,
                range.length,
                beat_amount,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Clears all events in `range` without splicing the track.
    pub fn clear_events_in_range(&self, range: BeatRange) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_music_track_clear_events_in_range(self.ptr, range.start, range.length, &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Cuts all events in `range`, splicing the track.
    pub fn cut_events_in_range(&self, range: BeatRange) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_music_track_cut_events_in_range(self.ptr, range.start, range.length, &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Copies events from `source_track` and splices them into this track.
    pub fn copy_events_in_range(
        &self,
        range: BeatRange,
        source_track: &Self,
        insert_at: f64,
    ) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_music_track_copy_events_in_range(
                self.ptr,
                range.start,
                range.length,
                source_track.ptr,
                insert_at,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Copies events from `source_track` and merges them into this track.
    pub fn copy_and_merge_events_in_range(
        &self,
        range: BeatRange,
        source_track: &Self,
        merge_at: f64,
    ) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_music_track_copy_and_merge_events_in_range(
                self.ptr,
                range.start,
                range.length,
                source_track.ptr,
                merge_at,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Enumerates events in `range`, optionally moving or removing them.
    pub fn enumerate_events_in_range<F>(&self, range: BeatRange, callback: F) -> Result<(), AVAudioError>
    where
        F: FnMut(TrackEvent) -> TrackEventAction + 'static,
    {
        let state = Box::new(EnumerationState {
            callback: Box::new(callback),
        });
        let state_ptr = Box::into_raw(state);
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_music_track_enumerate_events(
                self.ptr,
                range.start,
                range.length,
                Some(enumeration_trampoline),
                state_ptr.cast::<c_void>(),
                &mut err,
            )
        };
        unsafe {
            drop(Box::from_raw(state_ptr));
        }
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Returns a collected snapshot of all events in `range`.
    pub fn events_in_range(&self, range: BeatRange) -> Result<Vec<TrackEvent>, AVAudioError> {
        let events = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&events);
        self.enumerate_events_in_range(range, move |event| {
            sink.borrow_mut().push(event);
            TrackEventAction::default()
        })?;
        match Rc::try_unwrap(events) {
            Ok(events) => Ok(events.into_inner()),
            Err(events) => Ok(events.borrow().clone()),
        }
    }
}

unsafe extern "C" fn enumeration_trampoline(
    userdata: *mut c_void,
    event_json: *const c_char,
    beat: f64,
    new_beat_out: *mut f64,
    remove_out: *mut bool,
    error_out: *mut *mut c_char,
) -> i32 {
    let Some(state) = userdata.cast::<EnumerationState>().as_mut() else {
        return ffi::status::CALLBACK_ERROR;
    };
    let event = match event_from_json_ptr(event_json) {
        Ok(event) => event,
        Err(error) => {
            if !error_out.is_null() {
                if let Ok(message) = CString::new(error.to_string()) {
                    unsafe { *error_out = message.into_raw() };
                }
            }
            return ffi::status::CALLBACK_ERROR;
        }
    };
    let action = (state.callback)(TrackEvent { beat, event });
    if !new_beat_out.is_null() {
        unsafe { *new_beat_out = action.new_beat.unwrap_or(beat) };
    }
    if !remove_out.is_null() {
        unsafe { *remove_out = action.remove };
    }
    ffi::status::OK
}
