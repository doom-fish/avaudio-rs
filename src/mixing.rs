//! [`AVAudioMixing`] protocol abstractions.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ptr;

use serde::Deserialize;

use crate::connection_point::AudioConnectionPoint;
use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::types::{
    Audio3DMixingPointSourceInHeadMode, Audio3DMixingRenderingAlgorithm,
    Audio3DMixingSourceMode, Audio3DVector, AudioNodeBus,
};
use crate::util::parse_json_and_free;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
struct Audio3DVectorPayload {
    x: f32,
    y: f32,
    z: f32,
}

#[doc(hidden)]
pub trait AudioMixingHandle {
    fn as_mixing_ptr(&self) -> *mut c_void;
}

/// Wraps an `AVAudioMixingDestination` vended by a source node.
pub struct AudioMixingDestination {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioMixingDestination {}

impl Drop for AudioMixingDestination {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_mixing_destination_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioMixingHandle for AudioMixingDestination {
    fn as_mixing_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioMixingDestination {
    /// Returns the underlying mixer connection point.
    pub fn connection_point(&self) -> Result<AudioConnectionPoint, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_mixing_destination_copy_connection_point(self.ptr, &mut err) };
        if ptr.is_null() {
            if err.is_null() {
                return Err(AVAudioError::OperationFailed(
                    "mixing destination did not provide a connection point".into(),
                ));
            }
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(AudioConnectionPoint { ptr })
    }
}

/// Mirrors `AVAudioStereoMixing`.
pub trait AudioStereoMixing: AudioMixingHandle {
    /// Returns the stereo pan setting.
    fn pan(&self) -> Result<f32, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let value = unsafe { ffi::av_audio_stereo_mixing_get_pan(self.as_mixing_ptr(), &mut err) };
        if !err.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(value)
    }

    /// Sets the stereo pan.
    fn set_pan(&self, pan: f32) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_audio_stereo_mixing_set_pan(self.as_mixing_ptr(), pan, &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }
}

impl<T: AudioMixingHandle + ?Sized> AudioStereoMixing for T {}

/// Mirrors `AVAudio3DMixing`.
pub trait Audio3DMixing: AudioMixingHandle {
    /// Returns the rendering algorithm.
    fn rendering_algorithm(&self) -> Result<Audio3DMixingRenderingAlgorithm, AVAudioError> {
        let raw = read_3d_enum(
            self.as_mixing_ptr(),
            ffi::av_audio_3d_mixing_get_rendering_algorithm,
        )?;
        decode_rendering_algorithm(raw)
    }

    /// Sets the rendering algorithm.
    fn set_rendering_algorithm(
        &self,
        algorithm: Audio3DMixingRenderingAlgorithm,
    ) -> Result<(), AVAudioError> {
        write_3d_enum(
            self.as_mixing_ptr(),
            algorithm as i64,
            ffi::av_audio_3d_mixing_set_rendering_algorithm,
        )
    }

    /// Returns the source mode.
    fn source_mode(&self) -> Result<Audio3DMixingSourceMode, AVAudioError> {
        let raw = read_3d_enum(self.as_mixing_ptr(), ffi::av_audio_3d_mixing_get_source_mode)?;
        decode_source_mode(raw)
    }

    /// Sets the source mode.
    fn set_source_mode(&self, mode: Audio3DMixingSourceMode) -> Result<(), AVAudioError> {
        write_3d_enum(
            self.as_mixing_ptr(),
            mode as i64,
            ffi::av_audio_3d_mixing_set_source_mode,
        )
    }

    /// Returns the point-source in-head mode.
    fn point_source_in_head_mode(
        &self,
    ) -> Result<Audio3DMixingPointSourceInHeadMode, AVAudioError> {
        let raw = read_3d_enum(
            self.as_mixing_ptr(),
            ffi::av_audio_3d_mixing_get_point_source_in_head_mode,
        )?;
        decode_point_source_in_head_mode(raw)
    }

    /// Sets the point-source in-head mode.
    fn set_point_source_in_head_mode(
        &self,
        mode: Audio3DMixingPointSourceInHeadMode,
    ) -> Result<(), AVAudioError> {
        write_3d_enum(
            self.as_mixing_ptr(),
            mode as i64,
            ffi::av_audio_3d_mixing_set_point_source_in_head_mode,
        )
    }

    /// Returns the playback rate.
    fn rate(&self) -> Result<f32, AVAudioError> {
        read_float(self.as_mixing_ptr(), ffi::av_audio_3d_mixing_get_rate)
    }

    /// Sets the playback rate.
    fn set_rate(&self, rate: f32) -> Result<(), AVAudioError> {
        write_float(self.as_mixing_ptr(), rate, ffi::av_audio_3d_mixing_set_rate)
    }

    /// Returns the reverb blend.
    fn reverb_blend(&self) -> Result<f32, AVAudioError> {
        read_float(self.as_mixing_ptr(), ffi::av_audio_3d_mixing_get_reverb_blend)
    }

    /// Sets the reverb blend.
    fn set_reverb_blend(&self, reverb_blend: f32) -> Result<(), AVAudioError> {
        write_float(
            self.as_mixing_ptr(),
            reverb_blend,
            ffi::av_audio_3d_mixing_set_reverb_blend,
        )
    }

    /// Returns the obstruction value.
    fn obstruction(&self) -> Result<f32, AVAudioError> {
        read_float(self.as_mixing_ptr(), ffi::av_audio_3d_mixing_get_obstruction)
    }

    /// Sets the obstruction value.
    fn set_obstruction(&self, obstruction: f32) -> Result<(), AVAudioError> {
        write_float(
            self.as_mixing_ptr(),
            obstruction,
            ffi::av_audio_3d_mixing_set_obstruction,
        )
    }

    /// Returns the occlusion value.
    fn occlusion(&self) -> Result<f32, AVAudioError> {
        read_float(self.as_mixing_ptr(), ffi::av_audio_3d_mixing_get_occlusion)
    }

    /// Sets the occlusion value.
    fn set_occlusion(&self, occlusion: f32) -> Result<(), AVAudioError> {
        write_float(
            self.as_mixing_ptr(),
            occlusion,
            ffi::av_audio_3d_mixing_set_occlusion,
        )
    }

    /// Returns the 3D position.
    fn position(&self) -> Result<Audio3DVector, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_3d_mixing_get_position_json(self.as_mixing_ptr(), &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        let payload: Audio3DVectorPayload = parse_json_and_free(json_ptr)?;
        Ok(Audio3DVector::new(payload.x, payload.y, payload.z))
    }

    /// Sets the 3D position.
    fn set_position(&self, position: Audio3DVector) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_3d_mixing_set_position(
                self.as_mixing_ptr(),
                position.x,
                position.y,
                position.z,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }
}

impl<T: AudioMixingHandle + ?Sized> Audio3DMixing for T {}

/// Mirrors `AVAudioMixing`.
pub trait AudioMixing: AudioStereoMixing + Audio3DMixing {
    /// Returns the input volume.
    fn volume(&self) -> Result<f32, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let value = unsafe { ffi::av_audio_mixing_get_volume(self.as_mixing_ptr(), &mut err) };
        if !err.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(value)
    }

    /// Sets the input volume.
    fn set_volume(&self, volume: f32) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_audio_mixing_set_volume(self.as_mixing_ptr(), volume, &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Returns the per-mixer destination for a connected source node.
    fn destination_for_mixer(
        &self,
        mixer: &dyn AudioNodeHandle,
        bus: AudioNodeBus,
    ) -> Result<Option<AudioMixingDestination>, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_mixing_destination_for_mixer(
                self.as_mixing_ptr(),
                mixer.as_node_ptr(),
                bus,
                &mut err,
            )
        };
        if ptr.is_null() {
            if err.is_null() {
                return Ok(None);
            }
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Some(AudioMixingDestination { ptr }))
    }
}

impl<T: AudioMixingHandle + ?Sized> AudioMixing for T {}

fn read_float(
    mixing_ptr: *mut c_void,
    getter: unsafe extern "C" fn(*mut c_void, *mut *mut c_char) -> f32,
) -> Result<f32, AVAudioError> {
    let mut err: *mut c_char = ptr::null_mut();
    let value = unsafe { getter(mixing_ptr, &mut err) };
    if !err.is_null() {
        return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
    }
    Ok(value)
}

fn write_float(
    mixing_ptr: *mut c_void,
    value: f32,
    setter: unsafe extern "C" fn(*mut c_void, f32, *mut *mut c_char) -> i32,
) -> Result<(), AVAudioError> {
    let mut err: *mut c_char = ptr::null_mut();
    let status = unsafe { setter(mixing_ptr, value, &mut err) };
    if status != ffi::status::OK {
        return Err(unsafe { from_swift(status, err) });
    }
    Ok(())
}

fn read_3d_enum(
    mixing_ptr: *mut c_void,
    getter: unsafe extern "C" fn(*mut c_void, *mut *mut c_char) -> i64,
) -> Result<i64, AVAudioError> {
    let mut err: *mut c_char = ptr::null_mut();
    let value = unsafe { getter(mixing_ptr, &mut err) };
    if !err.is_null() {
        return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
    }
    Ok(value)
}

fn write_3d_enum(
    mixing_ptr: *mut c_void,
    value: i64,
    setter: unsafe extern "C" fn(*mut c_void, i64, *mut *mut c_char) -> i32,
) -> Result<(), AVAudioError> {
    let mut err: *mut c_char = ptr::null_mut();
    let status = unsafe { setter(mixing_ptr, value, &mut err) };
    if status != ffi::status::OK {
        return Err(unsafe { from_swift(status, err) });
    }
    Ok(())
}

fn decode_rendering_algorithm(
    raw: i64,
) -> Result<Audio3DMixingRenderingAlgorithm, AVAudioError> {
    match raw {
        0 => Ok(Audio3DMixingRenderingAlgorithm::EqualPowerPanning),
        1 => Ok(Audio3DMixingRenderingAlgorithm::SphericalHead),
        2 => Ok(Audio3DMixingRenderingAlgorithm::Hrtf),
        3 => Ok(Audio3DMixingRenderingAlgorithm::SoundField),
        5 => Ok(Audio3DMixingRenderingAlgorithm::StereoPassThrough),
        6 => Ok(Audio3DMixingRenderingAlgorithm::HrtfHighQuality),
        7 => Ok(Audio3DMixingRenderingAlgorithm::Auto),
        other => Err(AVAudioError::OperationFailed(format!(
            "unknown AVAudio3DMixingRenderingAlgorithm raw value: {other}"
        ))),
    }
}

fn decode_source_mode(raw: i64) -> Result<Audio3DMixingSourceMode, AVAudioError> {
    match raw {
        0 => Ok(Audio3DMixingSourceMode::SpatializeIfMono),
        1 => Ok(Audio3DMixingSourceMode::Bypass),
        2 => Ok(Audio3DMixingSourceMode::PointSource),
        3 => Ok(Audio3DMixingSourceMode::AmbienceBed),
        other => Err(AVAudioError::OperationFailed(format!(
            "unknown AVAudio3DMixingSourceMode raw value: {other}"
        ))),
    }
}

fn decode_point_source_in_head_mode(
    raw: i64,
) -> Result<Audio3DMixingPointSourceInHeadMode, AVAudioError> {
    match raw {
        0 => Ok(Audio3DMixingPointSourceInHeadMode::Mono),
        1 => Ok(Audio3DMixingPointSourceInHeadMode::Bypass),
        other => Err(AVAudioError::OperationFailed(format!(
            "unknown AVAudio3DMixingPointSourceInHeadMode raw value: {other}"
        ))),
    }
}
