//! [`AudioUnitEQ`] — equalizer bands and global gain.

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
use crate::node::AudioNodeHandle;
use crate::unit_effect::AudioUnitHandle;
use crate::util::parse_json_and_free;

fn band_to_i32(index: usize) -> Result<i32, AVAudioError> {
    i32::try_from(index)
        .map_err(|_| AVAudioError::InvalidArgument("band index exceeds Int32 range".into()))
}

/// Read-only band state reported by `AVAudioUnitEQ`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioEQBandInfo {
    /// Raw `AVAudioUnitEQFilterType` value.
    pub filter_type: i32,
    /// Center or cutoff frequency in Hz.
    pub frequency: f32,
    /// Bandwidth in octaves.
    pub bandwidth: f32,
    /// Gain in dB.
    pub gain: f32,
    /// Whether the band is bypassed.
    pub bypass: bool,
}

/// Writable band parameters for `AVAudioUnitEQ`.
#[derive(Debug, Clone, PartialEq)]
pub struct AudioEQBandParams {
    /// Raw `AVAudioUnitEQFilterType` value.
    pub filter_type: i32,
    /// Center or cutoff frequency in Hz.
    pub frequency: f32,
    /// Bandwidth in octaves.
    pub bandwidth: f32,
    /// Gain in dB.
    pub gain: f32,
    /// Whether the band is bypassed.
    pub bypass: bool,
}

impl From<AudioEQBandInfo> for AudioEQBandParams {
    fn from(info: AudioEQBandInfo) -> Self {
        Self {
            filter_type: info.filter_type,
            frequency: info.frequency,
            bandwidth: info.bandwidth,
            gain: info.gain,
            bypass: info.bypass,
        }
    }
}

/// Wraps an `AVAudioUnitEQ` node.
pub struct AudioUnitEQ {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnitEQ {}

impl Drop for AudioUnitEQ {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_eq_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnitEQ {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnitEQ {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitEQ {
    /// Creates an equalizer with the requested number of bands.
    pub fn new(number_of_bands: usize) -> Result<Self, AVAudioError> {
        let band_count = i32::try_from(number_of_bands)
            .map_err(|_| AVAudioError::InvalidArgument("band count exceeds Int32 range".into()))?;
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_unit_eq_create(band_count, &mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }

    /// Returns the global gain in dB.
    pub fn global_gain(&self) -> f32 {
        unsafe { ffi::av_audio_unit_eq_get_global_gain(self.ptr) }
    }

    /// Sets the global gain in dB.
    pub fn set_global_gain(&self, gain: f32) {
        unsafe { ffi::av_audio_unit_eq_set_global_gain(self.ptr, gain) };
    }

    /// Returns the number of EQ bands.
    pub fn band_count(&self) -> usize {
        usize::try_from(unsafe { ffi::av_audio_unit_eq_get_band_count(self.ptr) }).unwrap_or(0)
    }

    /// Returns the parameters for a single band.
    pub fn band_info(&self, band_index: usize) -> Result<AudioEQBandInfo, AVAudioError> {
        let band_index = band_to_i32(band_index)?;
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr =
            unsafe { ffi::av_audio_unit_eq_get_band_info_json(self.ptr, band_index, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Applies parameters to a band.
    pub fn set_band_params(
        &self,
        band_index: usize,
        params: &AudioEQBandParams,
    ) -> Result<(), AVAudioError> {
        let band_index = band_to_i32(band_index)?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_unit_eq_set_band_params(
                self.ptr,
                band_index,
                params.filter_type,
                params.frequency,
                params.bandwidth,
                params.gain,
                params.bypass,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }
}
