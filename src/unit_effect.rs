//! Shared audio-unit traits.

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
use crate::node::AudioNodeHandle;
use crate::util::parse_json_and_free;

/// Common state shared by bypass-capable audio-unit nodes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioUnitInfo {
    /// Whether the underlying effect/time-effect is bypassed.
    pub bypass: bool,
}

/// Implemented by AVAudioUnit-backed nodes that can be attached to an audio graph.
pub trait AudioUnitHandle: AudioNodeHandle {
    /// Returns a borrowed, non-owning pointer to the underlying `AVAudioUnit`.
    #[doc(hidden)]
    fn as_audio_unit_ptr(&self) -> *mut c_void;

    /// Returns common state for bypass-capable audio units.
    fn unit_info(&self) -> Result<AudioUnitInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_unit_info_json(self.as_audio_unit_ptr(), &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns whether the underlying audio unit is bypassed.
    fn bypass(&self) -> Result<bool, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_unit_info_json(self.as_audio_unit_ptr(), &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(parse_json_and_free::<AudioUnitInfo>(json_ptr)?.bypass)
    }

    /// Sets the bypass state on the underlying audio unit.
    fn set_bypass(&self, bypass: bool) {
        unsafe { ffi::av_audio_unit_set_bypass(self.as_audio_unit_ptr(), bypass) };
    }
}
