//! [`AVAudioIONode`] protocol abstractions and voice-processing value types.

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
use crate::util::parse_json_and_free;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AudioVoiceProcessingOtherAudioDuckingConfigurationPayload {
    enable_advanced_ducking: bool,
    ducking_level_raw: i64,
}

#[doc(hidden)]
pub trait AudioIONodeHandle {
    fn as_io_node_ptr(&self) -> *mut c_void;
}

/// Mirrors `AVAudioVoiceProcessingOtherAudioDuckingConfiguration.Level`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioVoiceProcessingOtherAudioDuckingLevel {
    Default,
    Min,
    Mid,
    Max,
    Other(i64),
}

impl AudioVoiceProcessingOtherAudioDuckingLevel {
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Default,
            10 => Self::Min,
            20 => Self::Mid,
            30 => Self::Max,
            other => Self::Other(other),
        }
    }

    #[must_use]
    pub const fn as_raw(self) -> i64 {
        match self {
            Self::Default => 0,
            Self::Min => 10,
            Self::Mid => 20,
            Self::Max => 30,
            Self::Other(other) => other,
        }
    }
}

/// Mirrors `AVAudioVoiceProcessingOtherAudioDuckingConfiguration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioVoiceProcessingOtherAudioDuckingConfiguration {
    pub enable_advanced_ducking: bool,
    pub ducking_level: AudioVoiceProcessingOtherAudioDuckingLevel,
}

impl AudioVoiceProcessingOtherAudioDuckingConfiguration {
    /// Creates a ducking configuration value.
    pub const fn new(
        enable_advanced_ducking: bool,
        ducking_level: AudioVoiceProcessingOtherAudioDuckingLevel,
    ) -> Self {
        Self {
            enable_advanced_ducking,
            ducking_level,
        }
    }
}

/// Mirrors `AVAudioVoiceProcessingSpeechActivityEvent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioVoiceProcessingSpeechActivityEvent {
    Started,
    Ended,
    Other(i64),
}

impl AudioVoiceProcessingSpeechActivityEvent {
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Started,
            1 => Self::Ended,
            other => Self::Other(other),
        }
    }
}

/// Mirrors `AVAudioIONode`.
pub trait AudioIONode: AudioIONodeHandle {
    /// Returns the presentation or hardware latency.
    fn presentation_latency(&self) -> Result<f64, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let latency = unsafe { ffi::av_audio_io_node_get_presentation_latency(self.as_io_node_ptr(), &mut err) };
        if !err.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(latency)
    }

    /// Returns whether voice processing is enabled.
    fn is_voice_processing_enabled(&self) -> Result<bool, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let enabled = unsafe {
            ffi::av_audio_io_node_is_voice_processing_enabled(self.as_io_node_ptr(), &mut err)
        };
        if !err.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(enabled)
    }

    /// Enables or disables voice processing.
    fn set_voice_processing_enabled(&self, enabled: bool) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_io_node_set_voice_processing_enabled(
                self.as_io_node_ptr(),
                enabled,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }
}

impl<T: AudioIONodeHandle + ?Sized> AudioIONode for T {}

pub fn parse_ducking_configuration_json(
    json_ptr: *mut c_char,
) -> Result<AudioVoiceProcessingOtherAudioDuckingConfiguration, AVAudioError> {
    let payload: AudioVoiceProcessingOtherAudioDuckingConfigurationPayload =
        parse_json_and_free(json_ptr)?;
    Ok(AudioVoiceProcessingOtherAudioDuckingConfiguration::new(
        payload.enable_advanced_ducking,
        AudioVoiceProcessingOtherAudioDuckingLevel::from_raw(payload.ducking_level_raw),
    ))
}
