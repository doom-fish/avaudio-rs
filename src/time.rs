//! [`AVAudioTime`] wrappers.

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
use crate::types::AudioFramePosition;
use crate::util::parse_json_and_free;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTimeInfo {
    pub host_time_valid: bool,
    pub host_time: u64,
    pub sample_time_valid: bool,
    pub sample_time: AudioFramePosition,
    pub sample_rate: f64,
}

/// Wraps an `AVAudioTime`.
pub struct AudioTime {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioTime {}

impl Drop for AudioTime {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_time_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioTime {
    /// Creates a host-time-only timestamp.
    pub fn new_host_time(host_time: u64) -> Self {
        Self {
            ptr: unsafe { ffi::av_audio_time_create_with_host_time(host_time) },
        }
    }

    /// Creates a sample-time-only timestamp.
    pub fn new_sample_time(sample_time: AudioFramePosition, sample_rate: f64) -> Self {
        Self {
            ptr: unsafe { ffi::av_audio_time_create_with_sample_time(sample_time, sample_rate) },
        }
    }

    /// Creates a timestamp with both host and sample time.
    pub fn new_host_and_sample_time(
        host_time: u64,
        sample_time: AudioFramePosition,
        sample_rate: f64,
    ) -> Self {
        Self {
            ptr: unsafe {
                ffi::av_audio_time_create_with_host_and_sample_time(
                    host_time,
                    sample_time,
                    sample_rate,
                )
            },
        }
    }

    /// Returns time metadata.
    pub fn info(&self) -> Result<AudioTimeInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_time_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns whether the host-time field is valid.
    pub fn host_time_valid(&self) -> Result<bool, AVAudioError> {
        Ok(self.info()?.host_time_valid)
    }

    /// Returns the host time.
    pub fn host_time(&self) -> Result<u64, AVAudioError> {
        Ok(self.info()?.host_time)
    }

    /// Returns whether the sample-time field is valid.
    pub fn sample_time_valid(&self) -> Result<bool, AVAudioError> {
        Ok(self.info()?.sample_time_valid)
    }

    /// Returns the sample time.
    pub fn sample_time(&self) -> Result<AudioFramePosition, AVAudioError> {
        Ok(self.info()?.sample_time)
    }

    /// Returns the sample rate associated with `sample_time`.
    pub fn sample_rate(&self) -> Result<f64, AVAudioError> {
        Ok(self.info()?.sample_rate)
    }

    /// Extrapolates missing host/sample-time fields from an anchor timestamp.
    pub fn extrapolate_from_anchor(&self, anchor: &Self) -> Option<Self> {
        let ptr = unsafe { ffi::av_audio_time_extrapolate_from_anchor(self.ptr, anchor.ptr) };
        (!ptr.is_null()).then_some(Self { ptr })
    }

    /// Converts seconds to host ticks.
    pub fn host_time_for_seconds(seconds: f64) -> u64 {
        unsafe { ffi::av_audio_time_host_time_for_seconds(seconds) }
    }

    /// Converts host ticks to seconds.
    pub fn seconds_for_host_time(host_time: u64) -> f64 {
        unsafe { ffi::av_audio_time_seconds_for_host_time(host_time) }
    }
}
