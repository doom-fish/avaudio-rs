//! [`AudioMixerNode`] — standalone `AVAudioMixerNode` wrapper.

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

/// Wraps an `AVAudioMixerNode` for use in an audio graph.
pub struct AudioMixerNode {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioMixerNode {}

impl Drop for AudioMixerNode {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_mixer_node_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioMixerNode {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioMixerNode {
    /// Creates a standalone mixer node.
    pub fn new() -> Result<Self, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_mixer_node_create() };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "failed to create AVAudioMixerNode".into(),
            ));
        }
        Ok(Self { ptr })
    }

    /// Returns the node output volume.
    pub fn output_volume(&self) -> f32 {
        unsafe { ffi::av_audio_mixer_node_get_output_volume(self.ptr) }
    }

    /// Sets the node output volume.
    pub fn set_output_volume(&self, volume: f32) {
        unsafe { ffi::av_audio_mixer_node_set_output_volume(self.ptr, volume) };
    }
}
