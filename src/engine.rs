#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CStr;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::format::AudioFormat;
use crate::input_output_node::{AudioInputNode, AudioOutputNode};
use crate::mixer::AudioMixerNode;
use crate::node::AudioNodeHandle;
use crate::pcm_buffer::PCMBuffer;
use crate::player::AudioPlayerNode;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioEngineInfo {
    pub is_running: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioEngineManualRenderingInfo {
    pub is_in_manual_rendering_mode: bool,
    pub manual_rendering_mode_raw: i64,
    pub manual_rendering_maximum_frame_count: u32,
    pub manual_rendering_sample_time: i64,
}

/// Mirrors `AVAudioEngineManualRenderingError`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioEngineManualRenderingError {
    InvalidMode,
    Initialized,
    NotRunning,
    Other(i32),
}

impl AudioEngineManualRenderingError {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            -80_800 => Self::InvalidMode,
            -80_801 => Self::Initialized,
            -80_802 => Self::NotRunning,
            other => Self::Other(other),
        }
    }
}

/// Mirrors `AVAudioEngineManualRenderingMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioEngineManualRenderingMode {
    Offline,
    Realtime,
    Other(i64),
}

impl AudioEngineManualRenderingMode {
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Offline,
            1 => Self::Realtime,
            other => Self::Other(other),
        }
    }

    const fn as_raw(self) -> i64 {
        match self {
            Self::Offline => 0,
            Self::Realtime => 1,
            Self::Other(other) => other,
        }
    }
}

/// Mirrors `AVAudioEngineManualRenderingStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioEngineManualRenderingStatus {
    Error,
    Success,
    InsufficientDataFromInputNode,
    CannotDoInCurrentContext,
    Other(i64),
}

impl AudioEngineManualRenderingStatus {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            -1 => Self::Error,
            0 => Self::Success,
            1 => Self::InsufficientDataFromInputNode,
            2 => Self::CannotDoInCurrentContext,
            other => Self::Other(other),
        }
    }
}

pub struct AudioEngine {
    ptr: *mut c_void,
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_engine_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioEngine {
    pub fn new() -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_engine_create(&mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::ENGINE_ERROR, err) });
        }
        Ok(Self { ptr })
    }

    pub(crate) const fn as_engine_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn info(&self) -> Result<AudioEngineInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_engine_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::ENGINE_ERROR, err) });
        }
        parse_json_and_free(json_ptr)
    }

    pub fn is_running(&self) -> Result<bool, AVAudioError> {
        Ok(self.info()?.is_running)
    }

    pub fn prepare(&self) {
        unsafe { ffi::av_audio_engine_prepare(self.ptr) };
    }

    pub fn start(&self) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_audio_engine_start(self.ptr, &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    pub fn stop(&self) {
        unsafe { ffi::av_audio_engine_stop(self.ptr) };
    }

    pub fn reset(&self) {
        unsafe { ffi::av_audio_engine_reset(self.ptr) };
    }

    /// Enables manual rendering for the engine.
    pub fn enable_manual_rendering_mode(
        &self,
        mode: AudioEngineManualRenderingMode,
        format: &AudioFormat,
        maximum_frame_count: u32,
    ) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_engine_enable_manual_rendering_mode(
                self.ptr,
                mode.as_raw(),
                format.ptr,
                maximum_frame_count,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Disables manual rendering and returns the engine to device-driven rendering.
    pub fn disable_manual_rendering_mode(&self) {
        unsafe { ffi::av_audio_engine_disable_manual_rendering_mode(self.ptr) };
    }

    /// Returns manual-rendering metadata.
    pub fn manual_rendering_info(&self) -> Result<AudioEngineManualRenderingInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr =
            unsafe { ffi::av_audio_engine_manual_rendering_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::ENGINE_ERROR, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns whether the engine is in manual rendering mode.
    pub fn is_in_manual_rendering_mode(&self) -> Result<bool, AVAudioError> {
        Ok(self.manual_rendering_info()?.is_in_manual_rendering_mode)
    }

    /// Returns the current manual-rendering mode.
    pub fn manual_rendering_mode(&self) -> Result<AudioEngineManualRenderingMode, AVAudioError> {
        Ok(AudioEngineManualRenderingMode::from_raw(
            self.manual_rendering_info()?.manual_rendering_mode_raw,
        ))
    }

    /// Returns the maximum manual-rendering frame count.
    pub fn manual_rendering_maximum_frame_count(&self) -> Result<u32, AVAudioError> {
        Ok(self
            .manual_rendering_info()?
            .manual_rendering_maximum_frame_count)
    }

    /// Returns the current manual-rendering sample time.
    pub fn manual_rendering_sample_time(&self) -> Result<i64, AVAudioError> {
        Ok(self.manual_rendering_info()?.manual_rendering_sample_time)
    }

    /// Returns the manual-rendering format.
    pub fn manual_rendering_format(&self) -> Result<AudioFormat, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_engine_copy_manual_rendering_format(self.ptr) };
        if ptr.is_null() {
            return Err(AVAudioError::FormatError(
                "audio engine did not provide a manual rendering format".into(),
            ));
        }
        Ok(AudioFormat { ptr })
    }

    /// Renders an offline block into the provided PCM buffer.
    pub fn render_offline(
        &self,
        number_of_frames: u32,
        buffer: &mut PCMBuffer,
    ) -> Result<AudioEngineManualRenderingStatus, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let raw_status = unsafe {
            ffi::av_audio_engine_render_offline(self.ptr, number_of_frames, buffer.ptr, &mut err)
        };
        if raw_status == i64::MIN {
            return Err(unsafe { from_swift(ffi::status::ENGINE_ERROR, err) });
        }
        Ok(AudioEngineManualRenderingStatus::from_raw(raw_status))
    }

    /// Renders a manual-rendering block into the provided PCM buffer.
    pub fn manual_rendering_block_render(
        &self,
        number_of_frames: u32,
        buffer: &mut PCMBuffer,
    ) -> Result<AudioEngineManualRenderingStatus, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let raw_status = unsafe {
            ffi::av_audio_engine_manual_rendering_block_render(
                self.ptr,
                number_of_frames,
                buffer.ptr,
                &mut err,
            )
        };
        if raw_status == i64::MIN {
            return Err(unsafe { from_swift(ffi::status::ENGINE_ERROR, err) });
        }
        Ok(AudioEngineManualRenderingStatus::from_raw(raw_status))
    }

    /// Returns the notification name posted when the engine configuration changes.
    pub fn configuration_change_notification_name() -> Result<String, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let string_ptr =
            unsafe { ffi::av_audio_engine_configuration_change_notification_name(&mut err) };
        if string_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::ENGINE_ERROR, err) });
        }
        let name = unsafe { CStr::from_ptr(string_ptr) }
            .to_string_lossy()
            .into_owned();
        unsafe { ffi::ava_string_free(string_ptr) };
        Ok(name)
    }

    pub fn attach_player_node(&self, player: &AudioPlayerNode) {
        self.attach_node(player);
    }

    pub fn attach_node(&self, node: &dyn AudioNodeHandle) {
        unsafe { ffi::av_audio_engine_attach_node(self.ptr, node.as_node_ptr()) };
    }

    pub fn connect_player_node_to_main_mixer(
        &self,
        player: &AudioPlayerNode,
        format: Option<&AudioFormat>,
    ) {
        self.connect_node_to_main_mixer(player, format);
    }

    pub fn connect_nodes(
        &self,
        from: &dyn AudioNodeHandle,
        to: &dyn AudioNodeHandle,
        format: Option<&AudioFormat>,
    ) {
        unsafe {
            ffi::av_audio_engine_connect_nodes(
                self.ptr,
                from.as_node_ptr(),
                to.as_node_ptr(),
                format.map_or(ptr::null_mut(), |format| format.ptr),
            );
        };
    }

    pub fn connect_node_to_main_mixer(
        &self,
        node: &dyn AudioNodeHandle,
        format: Option<&AudioFormat>,
    ) {
        unsafe {
            ffi::av_audio_engine_connect_node_to_main_mixer(
                self.ptr,
                node.as_node_ptr(),
                format.map_or(ptr::null_mut(), |format| format.ptr),
            );
        };
    }

    pub fn main_mixer_node(&self) -> Result<AudioMixerNode, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_engine_get_main_mixer_node(self.ptr) };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "audio engine did not provide a main mixer node".into(),
            ));
        }
        Ok(AudioMixerNode { ptr })
    }

    pub fn input_node(&self) -> Result<AudioInputNode, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_engine_get_input_node(self.ptr) };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "audio engine did not provide an input node".into(),
            ));
        }
        Ok(AudioInputNode { ptr })
    }

    pub fn output_node(&self) -> Result<AudioOutputNode, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_engine_get_output_node(self.ptr) };
        if ptr.is_null() {
            return Err(AVAudioError::OperationFailed(
                "audio engine did not provide an output node".into(),
            ));
        }
        Ok(AudioOutputNode { ptr })
    }

    pub fn main_mixer_output_format(&self, bus: usize) -> Result<AudioFormat, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_engine_copy_main_mixer_output_format(self.ptr, bus) };
        if ptr.is_null() {
            return Err(AVAudioError::FormatError(
                "audio engine did not provide a main mixer output format".into(),
            ));
        }
        Ok(AudioFormat { ptr })
    }
}

fn parse_json_and_free<T: DeserializeOwned>(json_ptr: *mut c_char) -> Result<T, AVAudioError> {
    let json = unsafe { CStr::from_ptr(json_ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::ava_string_free(json_ptr) };
    serde_json::from_str::<T>(&json).map_err(|error| {
        AVAudioError::OperationFailed(format!("failed to decode bridge JSON: {error}"))
    })
}
