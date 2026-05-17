//! [`AVAudioConnectionPoint`] wrappers.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ptr;

use serde::Deserialize;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::types::AudioNodeBus;
use crate::util::parse_json_and_free;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioConnectionPointInfo {
    pub bus: AudioNodeBus,
    pub node_raw: u64,
}

/// Wraps an `AVAudioConnectionPoint`.
pub struct AudioConnectionPoint {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioConnectionPoint {}

impl Drop for AudioConnectionPoint {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_connection_point_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioConnectionPoint {
    /// Creates a connection point for a node and bus.
    pub fn new(node: &dyn AudioNodeHandle, bus: AudioNodeBus) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_connection_point_create(node.as_node_ptr(), bus, &mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }

    /// Returns connection-point metadata.
    pub fn info(&self) -> Result<AudioConnectionPointInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_connection_point_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns the connection bus.
    pub fn bus(&self) -> Result<AudioNodeBus, AVAudioError> {
        Ok(self.info()?.bus)
    }

    /// Returns the raw pointer identity of the referenced node, if still available.
    pub fn node_raw(&self) -> Result<u64, AVAudioError> {
        Ok(self.info()?.node_raw)
    }

    /// Returns whether the connection point still references the supplied node.
    pub fn points_to(&self, node: &dyn AudioNodeHandle) -> Result<bool, AVAudioError> {
        Ok(self.node_raw()? == node.as_node_ptr() as usize as u64)
    }
}
