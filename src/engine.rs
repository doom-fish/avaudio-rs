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
use crate::player::AudioPlayerNode;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioEngineInfo {
    pub is_running: bool,
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
