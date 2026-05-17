//! [`AVAudioSessionCapability`] wrapper.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::c_void;
use core::ptr;

use crate::ffi;

/// Wraps an `AVAudioSessionCapability` instance.
pub struct AudioSessionCapability {
    ptr: *mut c_void,
}

unsafe impl Send for AudioSessionCapability {}

impl Drop for AudioSessionCapability {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_session_capability_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioSessionCapability {
    /// Creates a capability wrapper using the SDK-visible default initializer.
    pub fn new() -> Self {
        Self {
            ptr: unsafe { ffi::av_audio_session_capability_create() },
        }
    }

    /// Returns whether the capability is supported.
    pub fn is_supported(&self) -> bool {
        unsafe { ffi::av_audio_session_capability_is_supported(self.ptr) }
    }

    /// Returns whether the capability is enabled.
    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::av_audio_session_capability_is_enabled(self.ptr) }
    }
}

impl Default for AudioSessionCapability {
    fn default() -> Self {
        Self::new()
    }
}
