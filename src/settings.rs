//! `AVAudioSettings.h` enums and string constants.

#![allow(
    clippy::doc_markdown,
    clippy::enum_variant_names,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unsafe_derive_deserialize
)]

use core::ffi::c_char;
use core::ptr;

use serde::Deserialize;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::util::parse_json_and_free;

/// Mirrors the `AVAudioBitRateStrategy_*` NSString constants.
pub type AudioBitRateStrategy = String;

/// Bridged `AVAudioSettings.h` NSString constants.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioSettingsConstants {
    pub audio_file_type_key: String,
    pub bit_rate_strategy_constant: AudioBitRateStrategy,
    pub bit_rate_strategy_long_term_average: AudioBitRateStrategy,
    pub bit_rate_strategy_variable: AudioBitRateStrategy,
    pub bit_rate_strategy_variable_constrained: AudioBitRateStrategy,
}

impl AudioSettingsConstants {
    /// Returns the current AVFoundation constant values.
    pub fn current() -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_settings_constants_json(&mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }
}

/// Mirrors `AVAudioQuality`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioQuality {
    Min = 0,
    Low = 0x20,
    Medium = 0x40,
    High = 0x60,
    Max = 0x7f,
}

/// Mirrors `AVAudioDynamicRangeControlConfiguration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioDynamicRangeControlConfiguration {
    None = 0,
    Music = 1,
    Speech = 2,
    Movie = 3,
    Capture = 4,
}

/// Mirrors `AVAudioContentSource`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioContentSource {
    Unspecified = -1,
    Reserved = 0,
    AppleCaptureTraditional = 1,
    AppleCaptureSpatial = 2,
    AppleCaptureSpatialEnhanced = 3,
    AppleMusicTraditional = 4,
    AppleMusicSpatial = 5,
    AppleAvTraditionalOffline = 6,
    AppleAvSpatialOffline = 7,
    AppleAvTraditionalLive = 8,
    AppleAvSpatialLive = 9,
    ApplePassthrough = 10,
    CaptureTraditional = 33,
    CaptureSpatial = 34,
    CaptureSpatialEnhanced = 35,
    MusicTraditional = 36,
    MusicSpatial = 37,
    AvTraditionalOffline = 38,
    AvSpatialOffline = 39,
    AvTraditionalLive = 40,
    AvSpatialLive = 41,
    Passthrough = 42,
}
