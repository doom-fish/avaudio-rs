//! [`AudioSequencer`] — transport and user-event sequencing APIs.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::type_complexity
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::path::Path;

use serde::Deserialize;

use crate::engine::AudioEngine;
use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::util::parse_json_and_free;

fn path_to_cstring(path: impl AsRef<Path>) -> Result<CString, AVAudioError> {
    let path = path
        .as_ref()
        .to_str()
        .ok_or_else(|| AVAudioError::InvalidArgument("path is not valid UTF-8".into()))?;
    CString::new(path)
        .map_err(|error| AVAudioError::InvalidArgument(format!("path contains NUL byte: {error}")))
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

    /// Loads a MIDI/sequence file into the sequencer with default load options.
    pub fn load_from_path(&self, path: impl AsRef<Path>) -> Result<(), AVAudioError> {
        let path = path_to_cstring(path)?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_audio_sequencer_load_from_url(self.ptr, path.as_ptr(), &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
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
