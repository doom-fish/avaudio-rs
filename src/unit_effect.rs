//! Shared audio-unit traits and generic [`AVAudioUnitEffect`] support.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::path::Path;

use serde::Deserialize;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::unit::{AUAudioUnitHandle, AudioComponentDescription, AudioUnitMetadata};
use crate::util::parse_json_and_free;

fn path_to_cstring(path: impl AsRef<Path>) -> Result<CString, AVAudioError> {
    let path = path
        .as_ref()
        .to_str()
        .ok_or_else(|| AVAudioError::InvalidArgument("path is not valid UTF-8".into()))?;
    CString::new(path)
        .map_err(|error| AVAudioError::InvalidArgument(format!("path contains NUL byte: {error}")))
}

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

    /// Returns shared `AVAudioUnit` metadata.
    fn metadata(&self) -> Result<AudioUnitMetadata, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr =
            unsafe { ffi::av_audio_unit_metadata_json(self.as_audio_unit_ptr(), &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns the underlying component description.
    fn component_description(&self) -> Result<AudioComponentDescription, AVAudioError> {
        Ok(self.metadata()?.component_description)
    }

    /// Returns the audio-unit display name.
    fn name(&self) -> Result<String, AVAudioError> {
        Ok(self.metadata()?.name)
    }

    /// Returns the manufacturer name.
    fn manufacturer_name(&self) -> Result<String, AVAudioError> {
        Ok(self.metadata()?.manufacturer_name)
    }

    /// Returns the version integer.
    fn version(&self) -> Result<u64, AVAudioError> {
        Ok(self.metadata()?.version)
    }

    /// Returns the raw `AudioUnit` pointer value.
    fn audio_unit_raw(&self) -> Result<u64, AVAudioError> {
        Ok(self.metadata()?.audio_unit_raw)
    }

    /// Returns whether an `AUAudioUnit` bridge object is available.
    fn has_au_audio_unit(&self) -> Result<bool, AVAudioError> {
        Ok(self.metadata()?.has_au_audio_unit)
    }

    /// Copies the opaque `AUAudioUnit` object when available.
    fn copy_au_audio_unit(&self) -> Option<AUAudioUnitHandle> {
        let ptr = unsafe { ffi::av_audio_unit_copy_au_audio_unit(self.as_audio_unit_ptr()) };
        (!ptr.is_null()).then_some(AUAudioUnitHandle { ptr })
    }

    /// Loads a `.aupreset` file into the audio unit.
    fn load_preset_at_path(&self, path: impl AsRef<Path>) -> Result<(), AVAudioError> {
        let path = path_to_cstring(path)?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_unit_load_preset_at_url(self.as_audio_unit_ptr(), path.as_ptr(), &mut err)
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }
}

/// Wraps a generic `AVAudioUnitEffect`.
pub struct AudioUnitEffect {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnitEffect {}

impl Drop for AudioUnitEffect {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_effect_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnitEffect {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnitEffect {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitEffect {
    /// Instantiates a generic `AVAudioUnitEffect` for the provided component description.
    pub fn new_with_component_description(
        description: AudioComponentDescription,
    ) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_unit_effect_create_with_component_description(
                description.component_type,
                description.component_subtype,
                description.component_manufacturer,
                description.component_flags,
                description.component_flags_mask,
                &mut err,
            )
        };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }
}
