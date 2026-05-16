//! [`AudioConverter`] — format conversion between PCM buffers.

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
use crate::format::AudioFormat;
use crate::format::AudioFormatInfo;
use crate::util::parse_json_and_free;

/// Converter metadata reported by the Swift bridge.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioConverterInfo {
    /// Input format info.
    pub input_format: AudioFormatInfo,
    /// Output format info.
    pub output_format: AudioFormatInfo,
}

/// Wraps an `AVAudioConverter`.
pub struct AudioConverter {
    ptr: *mut c_void,
}

unsafe impl Send for AudioConverter {}

impl Drop for AudioConverter {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_converter_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioConverter {
    /// Creates a converter for a format pair.
    pub fn new(
        input_format: &AudioFormat,
        output_format: &AudioFormat,
    ) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_converter_create(input_format.ptr, output_format.ptr, &mut err)
        };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }

    /// Returns converter metadata.
    pub fn info(&self) -> Result<AudioConverterInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_converter_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Converts an input buffer into an output buffer.
    pub fn convert_buffer(
        &self,
        input_buffer: &PCMBuffer,
        output_buffer: &mut PCMBuffer,
    ) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_converter_convert_buffer(
                self.ptr,
                input_buffer.ptr,
                output_buffer.ptr,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }
}
