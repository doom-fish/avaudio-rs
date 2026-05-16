//! [`AudioSimplePlayer`] — direct `AVAudioPlayer` playback.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::path::Path;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;

/// Wraps an `AVAudioPlayer` instance.
pub struct AudioSimplePlayer {
    ptr: *mut c_void,
}

unsafe impl Send for AudioSimplePlayer {}

impl Drop for AudioSimplePlayer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_simple_player_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioSimplePlayer {
    /// Creates a player from a file path.
    pub fn create_from_path(path: impl AsRef<Path>) -> Result<Self, AVAudioError> {
        let path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| AVAudioError::InvalidArgument("path is not valid UTF-8".into()))?;
        let path = CString::new(path).map_err(|error| {
            AVAudioError::InvalidArgument(format!("path contains NUL byte: {error}"))
        })?;
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_simple_player_create_from_path(path.as_ptr(), &mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::PLAYER_ERROR, err) });
        }
        Ok(Self { ptr })
    }

    /// Starts playback.
    pub fn play(&self) -> bool {
        unsafe { ffi::av_audio_simple_player_play(self.ptr) }
    }

    /// Pauses playback.
    pub fn pause(&self) {
        unsafe { ffi::av_audio_simple_player_pause(self.ptr) };
    }

    /// Stops playback.
    pub fn stop(&self) {
        unsafe { ffi::av_audio_simple_player_stop(self.ptr) };
    }

    /// Returns the output volume.
    pub fn volume(&self) -> f32 {
        unsafe { ffi::av_audio_simple_player_get_volume(self.ptr) }
    }

    /// Sets the output volume.
    pub fn set_volume(&self, volume: f32) {
        unsafe { ffi::av_audio_simple_player_set_volume(self.ptr, volume) };
    }

    /// Returns the stereo pan.
    pub fn pan(&self) -> f32 {
        unsafe { ffi::av_audio_simple_player_get_pan(self.ptr) }
    }

    /// Sets the stereo pan.
    pub fn set_pan(&self, pan: f32) {
        unsafe { ffi::av_audio_simple_player_set_pan(self.ptr, pan) };
    }

    /// Returns the playback rate.
    pub fn rate(&self) -> f32 {
        unsafe { ffi::av_audio_simple_player_get_rate(self.ptr) }
    }

    /// Sets the playback rate.
    pub fn set_rate(&self, rate: f32) {
        unsafe { ffi::av_audio_simple_player_set_rate(self.ptr, rate) };
    }

    /// Returns the file duration in seconds.
    pub fn duration(&self) -> f64 {
        unsafe { ffi::av_audio_simple_player_get_duration(self.ptr) }
    }

    /// Returns the current playback time in seconds.
    pub fn current_time(&self) -> f64 {
        unsafe { ffi::av_audio_simple_player_get_current_time(self.ptr) }
    }

    /// Seeks to a playback time in seconds.
    pub fn set_current_time(&self, time: f64) {
        unsafe { ffi::av_audio_simple_player_set_current_time(self.ptr, time) };
    }

    /// Returns whether the player is actively playing.
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::av_audio_simple_player_is_playing(self.ptr) }
    }

    /// Returns the current loop count.
    pub fn number_of_loops(&self) -> i32 {
        unsafe { ffi::av_audio_simple_player_get_number_of_loops(self.ptr) }
    }

    /// Sets the loop count.
    pub fn set_number_of_loops(&self, loop_count: i32) {
        unsafe { ffi::av_audio_simple_player_set_number_of_loops(self.ptr, loop_count) };
    }

    /// Prepares the player for low-latency playback.
    pub fn prepare_to_play(&self) -> bool {
        unsafe { ffi::av_audio_simple_player_prepare_to_play(self.ptr) }
    }
}
