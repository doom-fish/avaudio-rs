//! [`PCMBuffer`] — `AVAudioPCMBuffer` wrappers.

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
use crate::format::{AudioFormat, AudioFormatInfo};
use crate::util::parse_json_and_free;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PCMBufferInfo {
    pub frame_capacity: u32,
    pub frame_length: u32,
    pub format: AudioFormatInfo,
}

pub struct PCMBuffer {
    pub(crate) ptr: *mut c_void,
}

impl Drop for PCMBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_pcm_buffer_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl PCMBuffer {
    pub fn new(format: &AudioFormat, frame_capacity: u32) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_pcm_buffer_create(format.ptr, frame_capacity, &mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FORMAT_ERROR, err) });
        }
        Ok(Self { ptr })
    }

    pub fn info(&self) -> Result<PCMBufferInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_pcm_buffer_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FILE_ERROR, err) });
        }
        parse_json_and_free(json_ptr)
    }

    pub fn frame_capacity(&self) -> Result<u32, AVAudioError> {
        Ok(self.info()?.frame_capacity)
    }

    pub fn frame_length(&self) -> Result<u32, AVAudioError> {
        Ok(self.info()?.frame_length)
    }

    pub fn set_frame_length(&mut self, frame_length: u32) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::av_audio_pcm_buffer_set_frame_length(self.ptr, frame_length, &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    pub fn format(&self) -> Result<AudioFormat, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_pcm_buffer_copy_format(self.ptr) };
        if ptr.is_null() {
            return Err(AVAudioError::FormatError(
                "PCM buffer did not provide a format".into(),
            ));
        }
        Ok(AudioFormat { ptr })
    }
}
