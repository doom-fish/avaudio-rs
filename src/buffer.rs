//! [`AVAudioBuffer`] helpers shared by concrete buffer types.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use core::ffi::{c_char, c_void};
use core::ptr;

use serde::Deserialize;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::file::PCMBuffer;
use crate::format::AudioFormatInfo;
use crate::util::parse_json_and_free;

/// Structural information from an `AVAudioBuffer`'s `AudioBufferList`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioBufferInfo {
    /// Buffer format metadata.
    pub format: AudioFormatInfo,
    /// Number of buffers in the underlying `AudioBufferList`.
    pub buffer_count: u32,
    /// Byte size for each `AudioBuffer` entry.
    pub bytes_per_buffer: Vec<u32>,
    /// Channel count for each `AudioBuffer` entry.
    pub channel_counts: Vec<u32>,
}

/// Implemented by types backed by `AVAudioBuffer`.
pub trait AudioBufferHandle {
    /// Returns a borrowed, non-owning pointer to the underlying `AVAudioBuffer`.
    #[doc(hidden)]
    fn as_buffer_ptr(&self) -> *mut c_void;

    /// Returns structural information about the underlying `AudioBufferList`.
    fn buffer_info(&self) -> Result<AudioBufferInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_buffer_info_json(self.as_buffer_ptr(), &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }
}

impl AudioBufferHandle for PCMBuffer {
    fn as_buffer_ptr(&self) -> *mut c_void {
        self.ptr
    }
}
