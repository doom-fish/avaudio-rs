//! [`AudioUnitDistortion`] — multi-stage distortion processing.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::c_void;
use core::ptr;

use crate::error::AVAudioError;
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::unit_effect::AudioUnitHandle;

/// Factory preset identifiers for `AVAudioUnitDistortion`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AudioUnitDistortionPreset {
    DrumsBitBrush = 0,
    DrumsBufferBeats = 1,
    DrumsLoFi = 2,
    MultiBrokenSpeaker = 3,
    MultiCellphoneConcert = 4,
    MultiDecimated1 = 5,
    MultiDecimated2 = 6,
    MultiDecimated3 = 7,
    MultiDecimated4 = 8,
    MultiDistortedFunk = 9,
    MultiDistortedCubed = 10,
    MultiDistortedSquared = 11,
    MultiEcho1 = 12,
    MultiEcho2 = 13,
    MultiEchoTight1 = 14,
    MultiEchoTight2 = 15,
    MultiEverythingIsBroken = 16,
    SpeechAlienChatter = 17,
    SpeechCosmicInterference = 18,
    SpeechGoldenPi = 19,
    SpeechRadioTower = 20,
    SpeechWaves = 21,
}

impl AudioUnitDistortionPreset {
    const fn as_raw(self) -> i32 {
        self as i32
    }
}

/// Wraps an `AVAudioUnitDistortion` node.
pub struct AudioUnitDistortion {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnitDistortion {}

impl Drop for AudioUnitDistortion {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_distortion_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnitDistortion {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnitDistortion {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitDistortion {
    /// Creates a distortion unit.
    pub fn new() -> Result<Self, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_unit_distortion_create() };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "failed to create AVAudioUnitDistortion".into(),
            ));
        }
        Ok(Self { ptr })
    }

    /// Returns the pre-gain in dB.
    pub fn pre_gain(&self) -> f32 {
        unsafe { ffi::av_audio_unit_distortion_get_pre_gain(self.ptr) }
    }

    /// Sets the pre-gain in dB.
    pub fn set_pre_gain(&self, pre_gain: f32) {
        unsafe { ffi::av_audio_unit_distortion_set_pre_gain(self.ptr, pre_gain) };
    }

    /// Returns the wet/dry mix percentage.
    pub fn wet_dry_mix(&self) -> f32 {
        unsafe { ffi::av_audio_unit_distortion_get_wet_dry_mix(self.ptr) }
    }

    /// Sets the wet/dry mix percentage.
    pub fn set_wet_dry_mix(&self, wet_dry_mix: f32) {
        unsafe { ffi::av_audio_unit_distortion_set_wet_dry_mix(self.ptr, wet_dry_mix) };
    }

    /// Loads a factory preset.
    pub fn load_factory_preset(&self, preset: AudioUnitDistortionPreset) {
        unsafe { ffi::av_audio_unit_distortion_load_factory_preset(self.ptr, preset.as_raw()) };
    }
}
