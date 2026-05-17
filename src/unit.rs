//! Generic [`AVAudioUnit`] support and component-description helpers.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ops::{BitOr, BitOrAssign};
use core::ptr;
use std::ffi::CString;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::unit_effect::AudioUnitHandle;
use crate::util::parse_json_and_free;

fn path_to_cstring(path: impl AsRef<Path>) -> Result<CString, AVAudioError> {
    let path = path
        .as_ref()
        .to_str()
        .ok_or_else(|| AVAudioError::InvalidArgument("path is not valid UTF-8".into()))?;
    CString::new(path)
        .map_err(|error| AVAudioError::InvalidArgument(format!("path contains NUL byte: {error}")))
}

/// Mirrors `AudioComponentDescription`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioComponentDescription {
    /// The audio-unit family, e.g. `kAudioUnitType_Effect`.
    pub component_type: u32,
    /// The specific subtype, e.g. `kAudioUnitSubType_Delay`.
    pub component_subtype: u32,
    /// The component manufacturer.
    pub component_manufacturer: u32,
    /// Component flags.
    pub component_flags: u32,
    /// Flags mask.
    pub component_flags_mask: u32,
}

impl AudioComponentDescription {
    /// Creates a new component description.
    pub const fn new(
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
        component_flags: u32,
        component_flags_mask: u32,
    ) -> Self {
        Self {
            component_type,
            component_subtype,
            component_manufacturer,
            component_flags,
            component_flags_mask,
        }
    }

    /// Encodes a four-character code into a `u32`.
    pub const fn fourcc(code: [u8; 4]) -> u32 {
        u32::from_be_bytes(code)
    }
}

/// Bitflags controlling async audio-unit instantiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AudioComponentInstantiationOptions(u32);

impl AudioComponentInstantiationOptions {
    /// No special options.
    pub const NONE: Self = Self(0);
    /// Request out-of-process loading for `AUv3` components.
    pub const LOAD_OUT_OF_PROCESS: Self = Self(1);
    /// Request in-process loading on macOS.
    pub const LOAD_IN_PROCESS: Self = Self(2);

    /// Returns the raw bitfield value.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl BitOr for AudioComponentInstantiationOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for AudioComponentInstantiationOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Metadata surfaced by `AVAudioUnit`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioUnitMetadata {
    /// `audioComponentDescription`.
    pub component_description: AudioComponentDescription,
    /// `name`.
    pub name: String,
    /// `manufacturerName`.
    pub manufacturer_name: String,
    /// `version`.
    pub version: u64,
    /// Raw `AudioUnit` pointer value.
    pub audio_unit_raw: u64,
    /// Whether `AUAudioUnit` is available.
    pub has_au_audio_unit: bool,
}

/// Opaque handle to the underlying `AUAudioUnit` object.
pub struct AUAudioUnitHandle {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AUAudioUnitHandle {}

impl Drop for AUAudioUnitHandle {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_au_audio_unit_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AUAudioUnitHandle {
    /// Returns the raw Objective-C pointer.
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

/// Generic `AVAudioUnit` wrapper returned by async component instantiation.
pub struct AudioUnit {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnit {}

impl Drop for AudioUnit {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnit {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnit {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnit {
    /// Instantiates an `AVAudioUnit` using Apple's asynchronous factory.
    pub fn instantiate(
        description: AudioComponentDescription,
        options: AudioComponentInstantiationOptions,
    ) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_unit_instantiate(
                description.component_type,
                description.component_subtype,
                description.component_manufacturer,
                description.component_flags,
                description.component_flags_mask,
                options.bits(),
                &mut err,
            )
        };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }

    /// Returns `AVAudioUnit` metadata.
    pub fn metadata(&self) -> Result<AudioUnitMetadata, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_unit_metadata_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Loads a `.aupreset` file into the audio unit.
    pub fn load_preset_at_path(&self, path: impl AsRef<Path>) -> Result<(), AVAudioError> {
        let path = path_to_cstring(path)?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_audio_unit_load_preset_at_url(self.ptr, path.as_ptr(), &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Copies the opaque `AUAudioUnit` wrapper when available.
    pub fn copy_au_audio_unit(&self) -> Option<AUAudioUnitHandle> {
        let ptr = unsafe { ffi::av_audio_unit_copy_au_audio_unit(self.ptr) };
        (!ptr.is_null()).then_some(AUAudioUnitHandle { ptr })
    }
}
