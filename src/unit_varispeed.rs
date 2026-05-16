//! [`AudioUnitVarispeed`] — rate-changing playback with pitch shift.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::c_void;
use core::ptr;

use crate::error::AVAudioError;
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::unit_effect::AudioUnitHandle;

/// Wraps an `AVAudioUnitVarispeed` node.
pub struct AudioUnitVarispeed {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnitVarispeed {}

impl Drop for AudioUnitVarispeed {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_varispeed_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnitVarispeed {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnitVarispeed {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitVarispeed {
    /// Creates a varispeed unit.
    pub fn new() -> Result<Self, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_unit_varispeed_create() };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "failed to create AVAudioUnitVarispeed".into(),
            ));
        }
        Ok(Self { ptr })
    }

    /// Returns the playback rate multiplier.
    pub fn rate(&self) -> f32 {
        unsafe { ffi::av_audio_unit_varispeed_get_rate(self.ptr) }
    }

    /// Sets the playback rate multiplier.
    pub fn set_rate(&self, rate: f32) {
        unsafe { ffi::av_audio_unit_varispeed_set_rate(self.ptr, rate) };
    }
}
