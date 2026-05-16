//! [`AudioOutputNode`] wrappers.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::convert::TryFrom;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::format::AudioFormatInfo;
use crate::node::AudioNodeHandle;
use crate::util::parse_json_and_free;

fn bus_to_i32(bus: usize) -> Result<i32, AVAudioError> {
    i32::try_from(bus)
        .map_err(|_| AVAudioError::InvalidArgument("bus index exceeds Int32 range".into()))
}

/// Wraps the engine-owned `AVAudioOutputNode`.
pub struct AudioOutputNode {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioOutputNode {}

impl Drop for AudioOutputNode {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_output_node_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioOutputNode {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioOutputNode {
    /// Returns the output format for a bus.
    pub fn output_format(&self, bus: usize) -> Result<AudioFormatInfo, AVAudioError> {
        let bus = bus_to_i32(bus)?;
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr =
            unsafe { ffi::av_audio_output_node_output_format_json(self.ptr, bus, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }
}
