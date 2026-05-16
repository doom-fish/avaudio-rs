//! [`AudioInputNode`] wrappers.

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
use crate::format::{AudioFormat, AudioFormatInfo};
use crate::node::AudioNodeHandle;
use crate::util::parse_json_and_free;

fn bus_to_i32(bus: usize) -> Result<i32, AVAudioError> {
    i32::try_from(bus)
        .map_err(|_| AVAudioError::InvalidArgument("bus index exceeds Int32 range".into()))
}

/// Wraps the engine-owned `AVAudioInputNode`.
pub struct AudioInputNode {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioInputNode {}

impl Drop for AudioInputNode {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_input_node_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioInputNode {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioInputNode {
    /// Returns the output format for a bus.
    pub fn output_format(&self, bus: usize) -> Result<AudioFormatInfo, AVAudioError> {
        let bus = bus_to_i32(bus)?;
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr =
            unsafe { ffi::av_audio_input_node_output_format_json(self.ptr, bus, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns the input format for a bus.
    pub fn input_format(&self, bus: usize) -> Result<AudioFormatInfo, AVAudioError> {
        let bus = bus_to_i32(bus)?;
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr =
            unsafe { ffi::av_audio_input_node_input_format_json(self.ptr, bus, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Installs a placeholder tap block that discards captured buffers.
    pub fn install_tap_scaffold(
        &self,
        bus: usize,
        buffer_size: u32,
        format: Option<&AudioFormat>,
    ) -> Result<(), AVAudioError> {
        let bus = bus_to_i32(bus)?;
        let status = unsafe {
            ffi::av_audio_input_node_install_tap_scaffold(
                self.ptr,
                bus,
                buffer_size,
                format.map_or(ptr::null_mut(), |format| format.ptr),
            )
        };
        if status != ffi::status::OK {
            return Err(AVAudioError::OperationFailed(
                "failed to install input-node tap scaffold".into(),
            ));
        }
        Ok(())
    }

    /// Removes a previously installed tap scaffold.
    pub fn remove_tap(&self, bus: usize) -> Result<(), AVAudioError> {
        let bus = bus_to_i32(bus)?;
        unsafe { ffi::av_audio_input_node_remove_tap(self.ptr, bus) };
        Ok(())
    }
}
