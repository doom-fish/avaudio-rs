//! [`AudioInputNode`] wrappers.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::convert::TryFrom;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::format::{AudioFormat, AudioFormatInfo};
use crate::io_node::{
    parse_ducking_configuration_json, AudioIONodeHandle,
    AudioVoiceProcessingOtherAudioDuckingConfiguration, AudioVoiceProcessingSpeechActivityEvent,
};
use crate::mixing::AudioMixingHandle;
use crate::node::AudioNodeHandle;
use crate::pcm_buffer::PCMBuffer;
use crate::util::parse_json_and_free;

fn bus_to_i32(bus: usize) -> Result<i32, AVAudioError> {
    i32::try_from(bus)
        .map_err(|_| AVAudioError::InvalidArgument("bus index exceeds Int32 range".into()))
}

/// Borrowed manual-rendering input buffer returned to an `AVAudioIONodeInputBlock`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioManualRenderingInput {
    ptr: *mut c_void,
}

impl AudioManualRenderingInput {
    /// Creates a borrowed manual-rendering input from a PCM buffer.
    pub const fn from_buffer(buffer: &PCMBuffer) -> Self {
        Self { ptr: buffer.ptr }
    }
}

impl From<&PCMBuffer> for AudioManualRenderingInput {
    fn from(buffer: &PCMBuffer) -> Self {
        Self::from_buffer(buffer)
    }
}

struct ManualRenderingInputState {
    callback: Box<dyn FnMut(u32) -> Option<AudioManualRenderingInput> + Send + 'static>,
}

struct SpeechActivityListenerState {
    callback: Box<dyn FnMut(AudioVoiceProcessingSpeechActivityEvent) + Send + 'static>,
}

/// Wraps the engine-owned `AVAudioInputNode`.
pub struct AudioInputNode {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioInputNode {}

impl Drop for AudioInputNode {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_input_node_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioInputNode {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioIONodeHandle for AudioInputNode {
    fn as_io_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioMixingHandle for AudioInputNode {
    fn as_mixing_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioInputNode {
    /// Returns the output format for a bus.
    pub fn output_format(&self, bus: usize) -> Result<AudioFormatInfo, AVAudioError> {
        let bus = bus_to_i32(bus)?;
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr =
            unsafe { ffi::av_audio_input_node_output_format_json(self.ptr, bus, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns the input format for a bus.
    pub fn input_format(&self, bus: usize) -> Result<AudioFormatInfo, AVAudioError> {
        let bus = bus_to_i32(bus)?;
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr =
            unsafe { ffi::av_audio_input_node_input_format_json(self.ptr, bus, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Installs a placeholder tap block that discards captured buffers.
    pub fn install_tap_scaffold(
        &self,
        bus: usize,
        buffer_size: u32,
        format: Option<&AudioFormat>,
    ) -> Result<(), AVAudioError> {
        let bus = bus_to_i32(bus)?;
        let status = unsafe {
            ffi::av_audio_input_node_install_tap_scaffold(
                self.ptr,
                bus,
                buffer_size,
                format.map_or(ptr::null_mut(), |format| format.ptr),
            )
        };
        if status != ffi::status::OK {
            return Err(AVAudioError::OperationFailed(
                "failed to install input-node tap scaffold".into(),
            ));
        }
        Ok(())
    }

    /// Removes a previously installed tap scaffold.
    pub fn remove_tap(&self, bus: usize) -> Result<(), AVAudioError> {
        let bus = bus_to_i32(bus)?;
        unsafe { ffi::av_audio_input_node_remove_tap(self.ptr, bus) };
        Ok(())
    }

    /// Installs a no-op manual-rendering input block for the supplied format.
    pub fn set_manual_rendering_input_pcm_format_scaffold(&self, format: &AudioFormat) -> bool {
        unsafe {
            ffi::av_audio_input_node_set_manual_rendering_input_pcm_format(
                self.ptr,
                format.ptr,
                None,
                ptr::null_mut(),
                None,
            )
        }
    }

    /// Installs a manual-rendering input block backed by a Rust callback.
    pub fn set_manual_rendering_input_pcm_format_with_callback<F>(
        &self,
        format: &AudioFormat,
        callback: F,
    ) -> bool
    where
        F: FnMut(u32) -> Option<AudioManualRenderingInput> + Send + 'static,
    {
        let (callback_fn, userdata, drop_fn) = manual_rendering_input_callback_parts(callback);
        let ok = unsafe {
            ffi::av_audio_input_node_set_manual_rendering_input_pcm_format(
                self.ptr,
                format.ptr,
                callback_fn,
                userdata,
                drop_fn,
            )
        };
        if !ok {
            if let Some(drop_fn) = drop_fn {
                unsafe { drop_fn(userdata) };
            }
        }
        ok
    }

    /// Returns whether microphone voice processing is bypassed.
    pub fn is_voice_processing_bypassed(&self) -> bool {
        unsafe { ffi::av_audio_input_node_get_voice_processing_bypassed(self.ptr) }
    }

    /// Enables or disables the microphone bypass path.
    pub fn set_voice_processing_bypassed(&self, bypassed: bool) {
        unsafe { ffi::av_audio_input_node_set_voice_processing_bypassed(self.ptr, bypassed) };
    }

    /// Returns whether automatic gain control is enabled.
    pub fn is_voice_processing_agc_enabled(&self) -> bool {
        unsafe { ffi::av_audio_input_node_get_voice_processing_agc_enabled(self.ptr) }
    }

    /// Enables or disables automatic gain control.
    pub fn set_voice_processing_agc_enabled(&self, enabled: bool) {
        unsafe { ffi::av_audio_input_node_set_voice_processing_agc_enabled(self.ptr, enabled) };
    }

    /// Returns whether the processed input is muted.
    pub fn is_voice_processing_input_muted(&self) -> bool {
        unsafe { ffi::av_audio_input_node_get_voice_processing_input_muted(self.ptr) }
    }

    /// Mutes or unmutes the processed input.
    pub fn set_voice_processing_input_muted(&self, muted: bool) {
        unsafe { ffi::av_audio_input_node_set_voice_processing_input_muted(self.ptr, muted) };
    }

    /// Installs a muted-speech activity listener.
    pub fn set_muted_speech_activity_event_listener<F>(&self, callback: F) -> bool
    where
        F: FnMut(AudioVoiceProcessingSpeechActivityEvent) + Send + 'static,
    {
        let (callback_fn, userdata, drop_fn) = speech_activity_listener_callback_parts(callback);
        let ok = unsafe {
            ffi::av_audio_input_node_set_muted_speech_activity_event_listener(
                self.ptr,
                callback_fn,
                userdata,
                drop_fn,
            )
        };
        if !ok {
            if let Some(drop_fn) = drop_fn {
                unsafe { drop_fn(userdata) };
            }
        }
        ok
    }

    /// Clears any muted-speech activity listener.
    pub fn clear_muted_speech_activity_event_listener(&self) -> bool {
        unsafe {
            ffi::av_audio_input_node_set_muted_speech_activity_event_listener(
                self.ptr,
                None,
                ptr::null_mut(),
                None,
            )
        }
    }

    /// Returns the ducking configuration for other audio.
    pub fn voice_processing_other_audio_ducking_configuration(
        &self,
    ) -> Result<AudioVoiceProcessingOtherAudioDuckingConfiguration, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe {
            ffi::av_audio_input_node_get_other_audio_ducking_configuration_json(
                self.ptr,
                &mut err,
            )
        };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_ducking_configuration_json(json_ptr)
    }

    /// Updates the ducking configuration for other audio.
    pub fn set_voice_processing_other_audio_ducking_configuration(
        &self,
        configuration: AudioVoiceProcessingOtherAudioDuckingConfiguration,
    ) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_input_node_set_other_audio_ducking_configuration(
                self.ptr,
                configuration.enable_advanced_ducking,
                configuration.ducking_level.as_raw(),
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }
}

fn manual_rendering_input_callback_parts<F>(
    callback: F,
) -> (
    Option<ffi::InputNodeInputBlockCallback>,
    *mut c_void,
    Option<ffi::DropCallback>,
)
where
    F: FnMut(u32) -> Option<AudioManualRenderingInput> + Send + 'static,
{
    let state = Box::new(ManualRenderingInputState {
        callback: Box::new(callback),
    });
    (
        Some(manual_rendering_input_trampoline),
        Box::into_raw(state).cast::<c_void>(),
        Some(manual_rendering_input_drop),
    )
}

fn speech_activity_listener_callback_parts<F>(
    callback: F,
) -> (Option<ffi::IntCallback>, *mut c_void, Option<ffi::DropCallback>)
where
    F: FnMut(AudioVoiceProcessingSpeechActivityEvent) + Send + 'static,
{
    let state = Box::new(SpeechActivityListenerState {
        callback: Box::new(callback),
    });
    (
        Some(speech_activity_listener_trampoline),
        Box::into_raw(state).cast::<c_void>(),
        Some(speech_activity_listener_drop),
    )
}

unsafe extern "C" fn manual_rendering_input_trampoline(
    userdata: *mut c_void,
    frame_count: u32,
) -> *mut c_void {
    let Some(state) = userdata.cast::<ManualRenderingInputState>().as_mut() else {
        return ptr::null_mut();
    };
    (state.callback)(frame_count).map_or(ptr::null_mut(), |input| input.ptr)
}

unsafe extern "C" fn manual_rendering_input_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    drop(Box::from_raw(userdata.cast::<ManualRenderingInputState>()));
}

unsafe extern "C" fn speech_activity_listener_trampoline(userdata: *mut c_void, value: i64) {
    let Some(state) = userdata.cast::<SpeechActivityListenerState>().as_mut() else {
        return;
    };
    (state.callback)(AudioVoiceProcessingSpeechActivityEvent::from_raw(value));
}

unsafe extern "C" fn speech_activity_listener_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    drop(Box::from_raw(userdata.cast::<SpeechActivityListenerState>()));
}
