//! [`AudioUnitSampler`] — sampler loading and global playback parameters.

#![allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::similar_names
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::path::Path;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::unit_effect::AudioUnitHandle;

fn path_to_cstring(path: impl AsRef<Path>) -> Result<CString, AVAudioError> {
    let path = path
        .as_ref()
        .to_str()
        .ok_or_else(|| AVAudioError::InvalidArgument("path is not valid UTF-8".into()))?;
    CString::new(path)
        .map_err(|error| AVAudioError::InvalidArgument(format!("path contains NUL byte: {error}")))
}

/// Wraps an `AVAudioUnitSampler` node.
pub struct AudioUnitSampler {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnitSampler {}

impl Drop for AudioUnitSampler {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_sampler_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnitSampler {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnitSampler {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitSampler {
    /// Creates a sampler unit.
    pub fn new() -> Result<Self, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_unit_sampler_create() };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "failed to create AVAudioUnitSampler".into(),
            ));
        }
        Ok(Self { ptr })
    }

    /// Loads an instrument, preset, or audio file URL.
    pub fn load_instrument(&self, path: impl AsRef<Path>) -> Result<(), AVAudioError> {
        let path = path_to_cstring(path)?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_audio_unit_sampler_load_instrument(self.ptr, path.as_ptr(), &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Loads a specific instrument from a DLS or SoundFont sound bank.
    pub fn load_sound_bank_instrument(
        &self,
        path: impl AsRef<Path>,
        program: u8,
        bank_msb_value: u8,
        bank_lsb_value: u8,
    ) -> Result<(), AVAudioError> {
        let path = path_to_cstring(path)?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_unit_sampler_load_sound_bank_instrument(
                self.ptr,
                path.as_ptr(),
                i32::from(program),
                i32::from(bank_msb_value),
                i32::from(bank_lsb_value),
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Returns the global stereo pan.
    pub fn stereo_pan(&self) -> f32 {
        unsafe { ffi::av_audio_unit_sampler_get_stereo_pan(self.ptr) }
    }

    /// Sets the global stereo pan.
    pub fn set_stereo_pan(&self, stereo_pan: f32) {
        unsafe { ffi::av_audio_unit_sampler_set_stereo_pan(self.ptr, stereo_pan) };
    }

    /// Returns the overall gain in dB.
    pub fn overall_gain(&self) -> f32 {
        unsafe { ffi::av_audio_unit_sampler_get_overall_gain(self.ptr) }
    }

    /// Sets the overall gain in dB.
    pub fn set_overall_gain(&self, overall_gain: f32) {
        unsafe { ffi::av_audio_unit_sampler_set_overall_gain(self.ptr, overall_gain) };
    }

    /// Returns the global tuning in cents.
    pub fn global_tuning(&self) -> f32 {
        unsafe { ffi::av_audio_unit_sampler_get_global_tuning(self.ptr) }
    }

    /// Sets the global tuning in cents.
    pub fn set_global_tuning(&self, global_tuning: f32) {
        unsafe { ffi::av_audio_unit_sampler_set_global_tuning(self.ptr, global_tuning) };
    }
}
