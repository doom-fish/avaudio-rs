//! [`AudioSequencer`] — transport, track, and user-event sequencing APIs.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::type_complexity
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::collections::BTreeMap;
use std::ffi::CString;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

use crate::engine::AudioEngine;
use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::music_track::MusicTrack;
use crate::util::parse_json_and_free;

fn path_to_cstring(path: impl AsRef<Path>) -> Result<CString, AVAudioError> {
    let path = path
        .as_ref()
        .to_str()
        .ok_or_else(|| AVAudioError::InvalidArgument("path is not valid UTF-8".into()))?;
    CString::new(path)
        .map_err(|error| AVAudioError::InvalidArgument(format!("path contains NUL byte: {error}")))
}

/// `AVMusicSequenceLoadOptions` bitflags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MusicSequenceLoadOptions(usize);

impl MusicSequenceLoadOptions {
    /// Preserve the source sequence's track structure.
    pub const NONE: Self = Self(0);
    /// Alias for the default preserve-tracks behavior.
    pub const PRESERVE_TRACKS: Self = Self(0);
    /// Split SMF MIDI channels into separate tracks while loading.
    pub const CHANNELS_TO_TRACKS: Self = Self(1 << 0);

    /// Returns the raw option bits.
    pub const fn bits(self) -> usize {
        self.0
    }

    /// Constructs flags from raw bits.
    pub const fn from_bits(bits: usize) -> Self {
        Self(bits)
    }

    /// Returns whether `other` is contained in these flags.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl Default for MusicSequenceLoadOptions {
    fn default() -> Self {
        Self::NONE
    }
}

impl BitOr for MusicSequenceLoadOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for MusicSequenceLoadOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for MusicSequenceLoadOptions {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for MusicSequenceLoadOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// Summary info reported by the sequencer bridge.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioSequencerInfo {
    /// Number of non-tempo tracks in the sequence.
    pub track_count: usize,
    /// Current playback position in seconds.
    pub current_position_in_seconds: f64,
    /// Current playback position in beats.
    pub current_position_in_beats: f64,
    /// Whether the sequencer transport is currently playing.
    pub is_playing: bool,
    /// Current playback rate.
    pub rate: f32,
    /// Whether the sequence currently exposes a tempo track.
    pub has_tempo_track: bool,
}

/// The bridged `AVAudioSequencerInfoDictionaryKey` string constants.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioSequencerInfoDictionaryKeys {
    pub album: String,
    pub approximate_duration_in_seconds: String,
    pub artist: String,
    pub channel_layout: String,
    pub comments: String,
    pub composer: String,
    pub copyright: String,
    pub encoding_application: String,
    pub genre: String,
    pub isrc: String,
    pub key_signature: String,
    pub lyricist: String,
    pub nominal_bit_rate: String,
    pub recorded_date: String,
    pub source_bit_depth: String,
    pub source_encoder: String,
    pub sub_title: String,
    pub tempo: String,
    pub time_signature: String,
    pub title: String,
    pub track_number: String,
    pub year: String,
}

/// Payload delivered for `AVMusicUserEvent` callbacks.
#[derive(Debug, Clone, PartialEq)]
pub struct AudioSequencerUserEvent {
    /// Raw, non-owning pointer to the `AVMusicTrack` that produced the event.
    pub track_ptr: *mut c_void,
    /// User-event bytes copied out of the Objective-C `NSData` payload.
    pub bytes: Vec<u8>,
    /// Beat location at which the event was encountered.
    pub beat: f64,
}

struct SequencerUserCallbackState {
    callback: Box<dyn FnMut(AudioSequencerUserEvent) + Send + 'static>,
}

/// Wraps an `AVAudioSequencer`.
pub struct AudioSequencer {
    ptr: *mut c_void,
}

unsafe impl Send for AudioSequencer {}

impl Drop for AudioSequencer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_sequencer_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioSequencer {
    /// Creates a sequencer with no audio-engine association.
    pub fn new() -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_sequencer_create(&mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }

    /// Creates a sequencer bound to an `AudioEngine`.
    pub fn with_engine(engine: &AudioEngine) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_sequencer_create_with_engine(engine.as_engine_ptr(), &mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }

    /// Returns current transport and track metadata.
    pub fn info(&self) -> Result<AudioSequencerInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_sequencer_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns the bridged `AVAudioSequencerInfoDictionaryKey` constants.
    pub fn info_dictionary_keys() -> Result<AudioSequencerInfoDictionaryKeys, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_sequencer_info_dictionary_keys_json(&mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns the sequence metadata dictionary as JSON-like values.
    pub fn user_info(&self) -> Result<BTreeMap<String, Value>, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_sequencer_user_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Loads a MIDI/sequence file into the sequencer with default load options.
    pub fn load_from_path(&self, path: impl AsRef<Path>) -> Result<(), AVAudioError> {
        self.load_from_path_with_options(path, MusicSequenceLoadOptions::NONE)
    }

    /// Loads a MIDI/sequence file into the sequencer with explicit load options.
    pub fn load_from_path_with_options(
        &self,
        path: impl AsRef<Path>,
        options: MusicSequenceLoadOptions,
    ) -> Result<(), AVAudioError> {
        let path = path_to_cstring(path)?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_sequencer_load_from_url(self.ptr, path.as_ptr(), options.bits(), &mut err)
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Loads sequence data with default load options.
    pub fn load_from_data(&self, data: &[u8]) -> Result<(), AVAudioError> {
        self.load_from_data_with_options(data, MusicSequenceLoadOptions::NONE)
    }

    /// Loads sequence data with explicit load options.
    pub fn load_from_data_with_options(
        &self,
        data: &[u8],
        options: MusicSequenceLoadOptions,
    ) -> Result<(), AVAudioError> {
        let bytes = if data.is_empty() { ptr::null() } else { data.as_ptr() };
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_sequencer_load_from_data(
                self.ptr,
                bytes,
                data.len(),
                options.bits(),
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Writes the sequence to disk.
    pub fn write_to_path(
        &self,
        path: impl AsRef<Path>,
        smpte_resolution: isize,
        replace_existing: bool,
    ) -> Result<(), AVAudioError> {
        let path = path_to_cstring(path)?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_sequencer_write_to_url(
                self.ptr,
                path.as_ptr(),
                smpte_resolution,
                replace_existing,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Serializes the current sequence into an in-memory data blob.
    pub fn data_with_smpte_resolution(&self, smpte_resolution: isize) -> Result<Vec<u8>, AVAudioError> {
        let mut out_len = 0usize;
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_sequencer_copy_data(self.ptr, smpte_resolution, &mut out_len, &mut err)
        };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        let data = if out_len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(ptr, out_len).to_vec() }
        };
        unsafe { ffi::ava_buffer_free(ptr.cast::<c_void>()) };
        Ok(data)
    }

    /// Reverses events across all tracks when supported by the OS.
    pub fn reverse_events(&self) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_audio_sequencer_reverse_events(self.ptr, &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Returns the number of non-tempo tracks.
    pub fn track_count(&self) -> Result<usize, AVAudioError> {
        Ok(self.info()?.track_count)
    }

    /// Returns the track at `index`.
    pub fn track_at_index(&self, index: usize) -> Result<MusicTrack, AVAudioError> {
        let index = isize::try_from(index)
            .map_err(|_| AVAudioError::InvalidArgument("track index exceeds isize::MAX".into()))?;
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_sequencer_copy_track_at_index(self.ptr, index, &mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(MusicTrack { ptr })
    }

    /// Returns all non-tempo tracks.
    pub fn tracks(&self) -> Result<Vec<MusicTrack>, AVAudioError> {
        let count = self.track_count()?;
        (0..count).map(|index| self.track_at_index(index)).collect()
    }

    /// Returns the tempo track when one exists.
    pub fn tempo_track(&self) -> Result<Option<MusicTrack>, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_sequencer_copy_tempo_track(self.ptr, &mut err) };
        if ptr.is_null() {
            return if err.is_null() {
                Ok(None)
            } else {
                Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) })
            };
        }
        Ok(Some(MusicTrack { ptr }))
    }

    /// Appends a new track and returns it.
    pub fn create_and_append_track(&self) -> Result<MusicTrack, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_sequencer_create_and_append_track(self.ptr, &mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(MusicTrack { ptr })
    }

    /// Removes a track from the sequence.
    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_track(&self, track: MusicTrack) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_audio_sequencer_remove_track(self.ptr, track.as_track_ptr(), &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Returns the current playback position in seconds.
    pub fn current_position_in_seconds(&self) -> Result<f64, AVAudioError> {
        Ok(self.info()?.current_position_in_seconds)
    }

    /// Sets the current playback position in seconds.
    pub fn set_current_position_in_seconds(&self, seconds: f64) {
        unsafe { ffi::av_audio_sequencer_set_current_position_in_seconds(self.ptr, seconds) };
    }

    /// Returns the current playback position in beats.
    pub fn current_position_in_beats(&self) -> Result<f64, AVAudioError> {
        Ok(self.info()?.current_position_in_beats)
    }

    /// Sets the current playback position in beats.
    pub fn set_current_position_in_beats(&self, beats: f64) {
        unsafe { ffi::av_audio_sequencer_set_current_position_in_beats(self.ptr, beats) };
    }

    /// Returns whether the sequencer transport is playing.
    pub fn is_playing(&self) -> Result<bool, AVAudioError> {
        Ok(self.info()?.is_playing)
    }

    /// Returns the playback rate.
    pub fn rate(&self) -> Result<f32, AVAudioError> {
        Ok(self.info()?.rate)
    }

    /// Sets the playback rate.
    pub fn set_rate(&self, rate: f32) {
        unsafe { ffi::av_audio_sequencer_set_rate(self.ptr, rate) };
    }

    /// Converts a beat position into seconds using the current tempo map.
    pub fn seconds_for_beats(&self, beats: f64) -> f64 {
        unsafe { ffi::av_audio_sequencer_seconds_for_beats(self.ptr, beats) }
    }

    /// Converts a time in seconds into beats using the current tempo map.
    pub fn beats_for_seconds(&self, seconds: f64) -> f64 {
        unsafe { ffi::av_audio_sequencer_beats_for_seconds(self.ptr, seconds) }
    }

    /// Converts a beat position into a host time.
    pub fn host_time_for_beats(&self, beats: f64) -> Result<u64, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let host_time = unsafe { ffi::av_audio_sequencer_host_time_for_beats(self.ptr, beats, &mut err) };
        if !err.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(host_time)
    }

    /// Converts a host time into a beat position.
    pub fn beats_for_host_time(&self, host_time: u64) -> Result<f64, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let beats = unsafe { ffi::av_audio_sequencer_beats_for_host_time(self.ptr, host_time, &mut err) };
        if !err.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(beats)
    }

    /// Pre-rolls the sequencer in preparation for playback.
    pub fn prepare_to_play(&self) {
        unsafe { ffi::av_audio_sequencer_prepare_to_play(self.ptr) };
    }

    /// Starts playback.
    pub fn start(&self) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_audio_sequencer_start(self.ptr, &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Stops playback.
    pub fn stop(&self) {
        unsafe { ffi::av_audio_sequencer_stop(self.ptr) };
    }

    /// Installs a callback for `AVMusicUserEvent`s encountered during playback.
    pub fn set_user_callback<F>(&self, callback: F) -> Result<(), AVAudioError>
    where
        F: FnMut(AudioSequencerUserEvent) + Send + 'static,
    {
        let (callback_fn, userdata, drop_fn) = sequencer_callback_parts(callback);
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_sequencer_set_user_callback(
                self.ptr,
                callback_fn,
                userdata,
                drop_fn,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            if let Some(drop_fn) = drop_fn {
                unsafe { drop_fn(userdata) };
            }
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Removes any previously installed user callback.
    pub fn clear_user_callback(&self) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_sequencer_set_user_callback(
                self.ptr,
                None,
                ptr::null_mut(),
                None,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }
}

fn sequencer_callback_parts<F>(
    callback: F,
) -> (
    Option<ffi::SequencerUserCallback>,
    *mut c_void,
    Option<ffi::DropCallback>,
)
where
    F: FnMut(AudioSequencerUserEvent) + Send + 'static,
{
    let state = Box::new(SequencerUserCallbackState {
        callback: Box::new(callback),
    });
    (
        Some(sequencer_user_callback_trampoline),
        Box::into_raw(state).cast::<c_void>(),
        Some(sequencer_user_callback_drop),
    )
}

unsafe extern "C" fn sequencer_user_callback_trampoline(
    userdata: *mut c_void,
    track_ptr: *mut c_void,
    bytes_ptr: *const u8,
    bytes_len: usize,
    beat: f64,
) {
    let Some(state) = userdata.cast::<SequencerUserCallbackState>().as_mut() else {
        return;
    };
    let bytes = if bytes_ptr.is_null() || bytes_len == 0 {
        Vec::new()
    } else {
        std::slice::from_raw_parts(bytes_ptr, bytes_len).to_vec()
    };
    (state.callback)(AudioSequencerUserEvent {
        track_ptr,
        bytes,
        beat,
    });
}

unsafe extern "C" fn sequencer_user_callback_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    drop(Box::from_raw(userdata.cast::<SequencerUserCallbackState>()));
}
