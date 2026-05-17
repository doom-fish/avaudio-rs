//! [`AudioSimplePlayer`] — direct `AVAudioPlayer` playback.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::{CStr, CString};
use std::path::Path;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;

/// Callback configuration bridging `AVAudioPlayerDelegate`.
#[derive(Default)]
pub struct AudioSimplePlayerDelegate {
    did_finish_playing: Option<Box<dyn FnMut(bool) + Send + 'static>>,
    decode_error: Option<Box<dyn FnMut(Option<String>) + Send + 'static>>,
}

impl AudioSimplePlayerDelegate {
    /// Creates an empty delegate configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a finish-playing callback.
    #[must_use]
    pub fn on_finish_playing<F>(mut self, callback: F) -> Self
    where
        F: FnMut(bool) + Send + 'static,
    {
        self.did_finish_playing = Some(Box::new(callback));
        self
    }

    /// Registers a decode-error callback.
    #[must_use]
    pub fn on_decode_error<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<String>) + Send + 'static,
    {
        self.decode_error = Some(Box::new(callback));
        self
    }
}

struct AudioSimplePlayerDelegateState {
    did_finish_playing: Option<Box<dyn FnMut(bool) + Send + 'static>>,
    decode_error: Option<Box<dyn FnMut(Option<String>) + Send + 'static>>,
}

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

    pub(crate) const fn ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Installs delegate callbacks bridging `AVAudioPlayerDelegate`.
    pub fn set_delegate(&self, delegate: AudioSimplePlayerDelegate) -> Result<(), AVAudioError> {
        let state = Box::new(AudioSimplePlayerDelegateState {
            did_finish_playing: delegate.did_finish_playing,
            decode_error: delegate.decode_error,
        });
        let userdata = Box::into_raw(state).cast::<c_void>();
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_simple_player_set_delegate(
                self.ptr,
                Some(simple_player_finish_trampoline),
                Some(simple_player_decode_error_trampoline),
                userdata,
                Some(simple_player_delegate_drop),
                &mut err,
            )
        };
        if status != ffi::status::OK {
            unsafe { simple_player_delegate_drop(userdata) };
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Clears any installed delegate callbacks.
    pub fn clear_delegate(&self) {
        unsafe { ffi::av_audio_simple_player_clear_delegate(self.ptr) };
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

unsafe extern "C" fn simple_player_finish_trampoline(userdata: *mut c_void, success: bool) {
    let Some(state) = userdata.cast::<AudioSimplePlayerDelegateState>().as_mut() else {
        return;
    };
    if let Some(callback) = state.did_finish_playing.as_mut() {
        callback(success);
    }
}

unsafe extern "C" fn simple_player_decode_error_trampoline(
    userdata: *mut c_void,
    message: *mut c_char,
) {
    let Some(state) = userdata.cast::<AudioSimplePlayerDelegateState>().as_mut() else {
        return;
    };
    if let Some(callback) = state.decode_error.as_mut() {
        let value = if message.is_null() {
            None
        } else {
            let decoded = CStr::from_ptr(message).to_string_lossy().into_owned();
            unsafe { ffi::ava_string_free(message) };
            Some(decoded)
        };
        callback(value);
    } else if !message.is_null() {
        unsafe { ffi::ava_string_free(message) };
    }
}

unsafe extern "C" fn simple_player_delegate_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    drop(Box::from_raw(userdata.cast::<AudioSimplePlayerDelegateState>()));
}
