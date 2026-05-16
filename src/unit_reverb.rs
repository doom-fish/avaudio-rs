//! [`AudioUnitReverb`] — wet/dry mix and factory presets.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use core::ffi::c_void;
use core::ptr;

use crate::error::AVAudioError;
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::unit_effect::AudioUnitHandle;

/// Factory preset identifiers for `AVAudioUnitReverb`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AudioUnitReverbPreset {
    /// Small room.
    SmallRoom = 0,
    /// Medium room.
    MediumRoom = 1,
    /// Large room.
    LargeRoom = 2,
    /// Medium hall.
    MediumHall = 3,
    /// Large hall.
    LargeHall = 4,
    /// Plate reverb.
    Plate = 5,
    /// Medium chamber.
    MediumChamber = 6,
    /// Large chamber.
    LargeChamber = 7,
    /// Cathedral.
    Cathedral = 8,
    /// Alternate large room.
    LargeRoom2 = 9,
    /// Alternate medium hall.
    MediumHall2 = 10,
    /// Third medium hall.
    MediumHall3 = 11,
    /// Alternate large hall.
    LargeHall2 = 12,
}

impl AudioUnitReverbPreset {
    const fn as_raw(self) -> i32 {
        self as i32
    }
}

/// Wraps an `AVAudioUnitReverb` node.
pub struct AudioUnitReverb {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnitReverb {}

impl Drop for AudioUnitReverb {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_reverb_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnitReverb {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnitReverb {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitReverb {
    /// Creates a reverb unit.
    pub fn new() -> Result<Self, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_unit_reverb_create() };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "failed to create AVAudioUnitReverb".into(),
            ));
        }
        Ok(Self { ptr })
    }

    /// Returns the wet/dry mix percentage.
    pub fn wet_dry_mix(&self) -> f32 {
        unsafe { ffi::av_audio_unit_reverb_get_wet_dry_mix(self.ptr) }
    }

    /// Sets the wet/dry mix percentage.
    pub fn set_wet_dry_mix(&self, mix: f32) {
        unsafe { ffi::av_audio_unit_reverb_set_wet_dry_mix(self.ptr, mix) };
    }

    /// Loads a factory preset.
    pub fn load_factory_preset(&self, preset: AudioUnitReverbPreset) {
        unsafe { ffi::av_audio_unit_reverb_load_factory_preset(self.ptr, preset.as_raw()) };
    }
}
