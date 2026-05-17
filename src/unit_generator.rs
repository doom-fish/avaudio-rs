//! Generic [`AVAudioUnitGenerator`] support.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ptr;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::unit::AudioComponentDescription;
use crate::unit_effect::AudioUnitHandle;

/// Wraps a generic `AVAudioUnitGenerator`.
pub struct AudioUnitGenerator {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnitGenerator {}

impl Drop for AudioUnitGenerator {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_generator_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnitGenerator {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnitGenerator {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitGenerator {
    /// Instantiates a generic `AVAudioUnitGenerator` for the provided component description.
    pub fn new_with_component_description(
        description: AudioComponentDescription,
    ) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_unit_generator_create_with_component_description(
                description.component_type,
                description.component_subtype,
                description.component_manufacturer,
                description.component_flags,
                description.component_flags_mask,
                &mut err,
            )
        };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }
}
