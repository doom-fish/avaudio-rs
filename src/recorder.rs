//! [`AudioRecorder`] — direct `AVAudioRecorder` capture.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::convert::TryFrom;
use std::ffi::CString;
use std::path::Path;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;

fn channel_to_i32(channel: usize) -> Result<i32, AVAudioError> {
    i32::try_from(channel)
        .map_err(|_| AVAudioError::InvalidArgument("channel index exceeds Int32 range".into()))
}

/// Wraps an `AVAudioRecorder` instance.
pub struct AudioRecorder {
    ptr: *mut c_void,
}

unsafe impl Send for AudioRecorder {}

impl Drop for AudioRecorder {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_recorder_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioRecorder {
    /// Creates a recorder writing linear PCM data to the supplied path.
    pub fn create(
        path: impl AsRef<Path>,
        sample_rate: f64,
        channels: u32,
        bit_depth: u32,
    ) -> Result<Self, AVAudioError> {
        let path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| AVAudioError::InvalidArgument("path is not valid UTF-8".into()))?;
        let path = CString::new(path).map_err(|error| {
            AVAudioError::InvalidArgument(format!("path contains NUL byte: {error}"))
        })?;
        let channels = i32::try_from(channels).map_err(|_| {
            AVAudioError::InvalidArgument("channel count exceeds Int32 range".into())
        })?;
        let bit_depth = i32::try_from(bit_depth)
            .map_err(|_| AVAudioError::InvalidArgument("bit depth exceeds Int32 range".into()))?;
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_recorder_create(path.as_ptr(), sample_rate, channels, bit_depth, &mut err)
        };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }

    /// Starts recording.
    pub fn record(&self) -> bool {
        unsafe { ffi::av_audio_recorder_record(self.ptr) }
    }

    /// Stops recording.
    pub fn stop(&self) {
        unsafe { ffi::av_audio_recorder_stop(self.ptr) };
    }

    /// Pauses recording.
    pub fn pause(&self) {
        unsafe { ffi::av_audio_recorder_pause(self.ptr) };
    }

    /// Returns whether recording is active.
    pub fn is_recording(&self) -> bool {
        unsafe { ffi::av_audio_recorder_is_recording(self.ptr) }
    }

    /// Returns the current recorded duration in seconds.
    pub fn current_time(&self) -> f64 {
        unsafe { ffi::av_audio_recorder_current_time(self.ptr) }
    }

    /// Enables or disables metering.
    pub fn set_metering_enabled(&self, enabled: bool) {
        unsafe { ffi::av_audio_recorder_set_metering_enabled(self.ptr, enabled) };
    }

    /// Refreshes meter values.
    pub fn update_meters(&self) {
        unsafe { ffi::av_audio_recorder_update_meters(self.ptr) };
    }

    /// Returns the average power for a channel.
    pub fn average_power(&self, channel: usize) -> Result<f32, AVAudioError> {
        let channel = channel_to_i32(channel)?;
        Ok(unsafe { ffi::av_audio_recorder_average_power(self.ptr, channel) })
    }

    /// Returns the peak power for a channel.
    pub fn peak_power(&self, channel: usize) -> Result<f32, AVAudioError> {
        let channel = channel_to_i32(channel)?;
        Ok(unsafe { ffi::av_audio_recorder_peak_power(self.ptr, channel) })
    }

    /// Deletes the underlying recording file.
    pub fn delete_recording(&self) -> bool {
        unsafe { ffi::av_audio_recorder_delete_recording(self.ptr) }
    }
}
