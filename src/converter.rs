//! [`AudioConverter`] — format conversion between PCM buffers.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::convert::TryFrom;

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

/// Mirrors `AVAudioConverterPrimeMethod`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioConverterPrimeMethod {
    Pre,
    Normal,
    None,
    Other(i64),
}

impl AudioConverterPrimeMethod {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Pre,
            1 => Self::Normal,
            2 => Self::None,
            other => Self::Other(other),
        }
    }

    const fn as_raw(self) -> i64 {
        match self {
            Self::Pre => 0,
            Self::Normal => 1,
            Self::None => 2,
            Self::Other(other) => other,
        }
    }
}

/// Mirrors `AVAudioConverterPrimeInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioConverterPrimeInfo {
    pub leading_frames: u32,
    pub trailing_frames: u32,
}

/// Mirrors `AVAudioConverterInputStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioConverterInputStatus {
    HaveData,
    NoDataNow,
    EndOfStream,
    Other(i64),
}

impl AudioConverterInputStatus {
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::HaveData,
            1 => Self::NoDataNow,
            2 => Self::EndOfStream,
            other => Self::Other(other),
        }
    }
}

/// Mirrors `AVAudioConverterOutputStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioConverterOutputStatus {
    HaveData,
    InputRanDry,
    EndOfStream,
    Error,
    Other(i64),
}

impl AudioConverterOutputStatus {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::HaveData,
            1 => Self::InputRanDry,
            2 => Self::EndOfStream,
            3 => Self::Error,
            other => Self::Other(other),
        }
    }
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

    /// Resets the converter so it can begin a fresh stream.
    pub fn reset(&self) {
        unsafe { ffi::av_audio_converter_reset(self.ptr) };
    }

    /// Returns the converter priming method.
    pub fn prime_method(&self) -> AudioConverterPrimeMethod {
        AudioConverterPrimeMethod::from_raw(unsafe {
            ffi::av_audio_converter_get_prime_method(self.ptr)
        })
    }

    /// Sets the converter priming method.
    pub fn set_prime_method(&self, prime_method: AudioConverterPrimeMethod) {
        unsafe { ffi::av_audio_converter_set_prime_method(self.ptr, prime_method.as_raw()) };
    }

    /// Returns priming-frame requirements.
    pub fn prime_info(&self) -> Result<AudioConverterPrimeInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_converter_prime_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Sets priming-frame requirements.
    pub fn set_prime_info(&self, info: AudioConverterPrimeInfo) {
        unsafe {
            ffi::av_audio_converter_set_prime_info(
                self.ptr,
                info.leading_frames,
                info.trailing_frames,
            );
        }
    }

    /// Converts an input buffer into an output buffer and reports the converter status.
    pub fn convert_buffer_status(
        &self,
        input_buffer: &PCMBuffer,
        output_buffer: &mut PCMBuffer,
    ) -> Result<AudioConverterOutputStatus, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let raw_status = unsafe {
            ffi::av_audio_converter_convert_buffer_with_status(
                self.ptr,
                input_buffer.ptr,
                output_buffer.ptr,
                &mut err,
            )
        };
        if raw_status < 0 {
            let status = i32::try_from(raw_status).unwrap_or(ffi::status::OPERATION_FAILED);
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(AudioConverterOutputStatus::from_raw(raw_status))
    }

    /// Converts an input buffer into an output buffer.
    pub fn convert_buffer(
        &self,
        input_buffer: &PCMBuffer,
        output_buffer: &mut PCMBuffer,
    ) -> Result<(), AVAudioError> {
        match self.convert_buffer_status(input_buffer, output_buffer)? {
            AudioConverterOutputStatus::HaveData
            | AudioConverterOutputStatus::InputRanDry
            | AudioConverterOutputStatus::EndOfStream => Ok(()),
            AudioConverterOutputStatus::Error | AudioConverterOutputStatus::Other(_) => {
                Err(AVAudioError::OperationFailed(
                    "converter returned an error status without an NSError".into(),
                ))
            }
        }
    }
}
