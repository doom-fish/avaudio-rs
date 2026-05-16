//! [`AudioUnitTimePitch`] — pitch and rate processing.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use core::ffi::c_void;
use core::ptr;

use crate::error::AVAudioError;
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::unit_effect::AudioUnitHandle;

/// Wraps an `AVAudioUnitTimePitch` node.
pub struct AudioUnitTimePitch {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnitTimePitch {}

impl Drop for AudioUnitTimePitch {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_time_pitch_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnitTimePitch {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnitTimePitch {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitTimePitch {
    /// Creates a time-pitch unit.
    pub fn new() -> Result<Self, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_unit_time_pitch_create() };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "failed to create AVAudioUnitTimePitch".into(),
            ));
        }
        Ok(Self { ptr })
    }

    /// Returns the pitch offset in cents.
    pub fn pitch(&self) -> f32 {
        unsafe { ffi::av_audio_unit_time_pitch_get_pitch(self.ptr) }
    }

    /// Sets the pitch offset in cents.
    pub fn set_pitch(&self, pitch: f32) {
        unsafe { ffi::av_audio_unit_time_pitch_set_pitch(self.ptr, pitch) };
    }

    /// Returns the playback rate multiplier.
    pub fn rate(&self) -> f32 {
        unsafe { ffi::av_audio_unit_time_pitch_get_rate(self.ptr) }
    }

    /// Sets the playback rate multiplier.
    pub fn set_rate(&self, rate: f32) {
        unsafe { ffi::av_audio_unit_time_pitch_set_rate(self.ptr, rate) };
    }

    /// Returns the overlap parameter.
    pub fn overlap(&self) -> f32 {
        unsafe { ffi::av_audio_unit_time_pitch_get_overlap(self.ptr) }
    }

    /// Sets the overlap parameter.
    pub fn set_overlap(&self, overlap: f32) {
        unsafe { ffi::av_audio_unit_time_pitch_set_overlap(self.ptr, overlap) };
    }
}
