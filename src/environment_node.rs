//! [`AudioEnvironmentNode`] — 3D spatial audio support.

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

/// Listener position in 3D space.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioListenerPosition {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
    /// Z coordinate.
    pub z: f32,
}

/// Listener orientation in angular degrees.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioListenerOrientation {
    /// Yaw angle.
    pub yaw: f32,
    /// Pitch angle.
    pub pitch: f32,
    /// Roll angle.
    pub roll: f32,
}

/// Distance attenuation parameters.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioDistanceAttenuation {
    /// Raw `AVAudioEnvironmentDistanceAttenuationModel` value.
    pub model: i32,
    /// Reference distance in meters.
    pub reference_distance: f32,
    /// Maximum distance in meters.
    pub maximum_distance: f32,
    /// Rolloff factor.
    pub rolloff_factor: f32,
}

/// Wraps an `AVAudioEnvironmentNode`.
pub struct AudioEnvironmentNode {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioEnvironmentNode {}

impl Drop for AudioEnvironmentNode {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_environment_node_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioEnvironmentNode {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioEnvironmentNode {
    /// Creates a new environment node.
    pub fn new() -> Result<Self, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_environment_node_create() };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "failed to create AVAudioEnvironmentNode".into(),
            ));
        }
        Ok(Self { ptr })
    }

    /// Sets the listener position.
    pub fn set_listener_position(&self, x: f32, y: f32, z: f32) {
        unsafe { ffi::av_audio_environment_node_set_listener_position(self.ptr, x, y, z) };
    }

    /// Returns the listener position.
    pub fn listener_position(&self) -> Result<AudioListenerPosition, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe {
            ffi::av_audio_environment_node_get_listener_position_json(self.ptr, &mut err)
        };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Sets the listener orientation.
    pub fn set_listener_orientation(&self, yaw: f32, pitch: f32, roll: f32) {
        unsafe {
            ffi::av_audio_environment_node_set_listener_orientation(self.ptr, yaw, pitch, roll);
        };
    }

    /// Returns the listener orientation.
    pub fn listener_orientation(&self) -> Result<AudioListenerOrientation, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe {
            ffi::av_audio_environment_node_get_listener_orientation_json(self.ptr, &mut err)
        };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Sets the distance attenuation parameters.
    pub fn set_distance_attenuation(
        &self,
        model: i32,
        reference_distance: f32,
        maximum_distance: f32,
        rolloff_factor: f32,
    ) {
        unsafe {
            ffi::av_audio_environment_node_set_distance_attenuation(
                self.ptr,
                model,
                reference_distance,
                maximum_distance,
                rolloff_factor,
            );
        };
    }

    /// Returns the distance attenuation parameters.
    pub fn distance_attenuation(&self) -> Result<AudioDistanceAttenuation, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe {
            ffi::av_audio_environment_node_get_distance_attenuation_json(self.ptr, &mut err)
        };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Sets the environment reverb blend level.
    pub fn set_reverb_blend(&self, blend: f32) {
        unsafe { ffi::av_audio_environment_node_set_reverb_blend(self.ptr, blend) };
    }

    /// Returns the environment reverb blend level.
    pub fn reverb_blend(&self) -> f32 {
        unsafe { ffi::av_audio_environment_node_get_reverb_blend(self.ptr) }
    }
}
