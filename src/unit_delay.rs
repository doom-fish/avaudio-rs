//! [`AudioUnitDelay`] — feedback delay processing.

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

/// Wraps an `AVAudioUnitDelay` node.
pub struct AudioUnitDelay {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnitDelay {}

impl Drop for AudioUnitDelay {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_delay_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnitDelay {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnitDelay {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitDelay {
    /// Creates a delay unit.
    pub fn new() -> Result<Self, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_unit_delay_create() };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "failed to create AVAudioUnitDelay".into(),
            ));
        }
        Ok(Self { ptr })
    }

    /// Returns the delay time in seconds.
    pub fn delay_time(&self) -> f64 {
        unsafe { ffi::av_audio_unit_delay_get_delay_time(self.ptr) }
    }

    /// Sets the delay time in seconds.
    pub fn set_delay_time(&self, delay_time: f64) {
        unsafe { ffi::av_audio_unit_delay_set_delay_time(self.ptr, delay_time) };
    }

    /// Returns the feedback percentage.
    pub fn feedback(&self) -> f32 {
        unsafe { ffi::av_audio_unit_delay_get_feedback(self.ptr) }
    }

    /// Sets the feedback percentage.
    pub fn set_feedback(&self, feedback: f32) {
        unsafe { ffi::av_audio_unit_delay_set_feedback(self.ptr, feedback) };
    }

    /// Returns the low-pass cutoff frequency in hertz.
    pub fn low_pass_cutoff(&self) -> f32 {
        unsafe { ffi::av_audio_unit_delay_get_low_pass_cutoff(self.ptr) }
    }

    /// Sets the low-pass cutoff frequency in hertz.
    pub fn set_low_pass_cutoff(&self, low_pass_cutoff: f32) {
        unsafe { ffi::av_audio_unit_delay_set_low_pass_cutoff(self.ptr, low_pass_cutoff) };
    }

    /// Returns the wet/dry mix percentage.
    pub fn wet_dry_mix(&self) -> f32 {
        unsafe { ffi::av_audio_unit_delay_get_wet_dry_mix(self.ptr) }
    }

    /// Sets the wet/dry mix percentage.
    pub fn set_wet_dry_mix(&self, wet_dry_mix: f32) {
        unsafe { ffi::av_audio_unit_delay_set_wet_dry_mix(self.ptr, wet_dry_mix) };
    }
}
