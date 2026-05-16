#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::{CStr, CString};
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::format::AudioFormatInfo;
use crate::format::AudioFormat;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFileInfo {
    pub length_frames: i64,
    pub processing_format: AudioFormatInfo,
    pub file_format: AudioFormatInfo,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PCMBufferInfo {
    pub frame_capacity: u32,
    pub frame_length: u32,
    pub format: AudioFormatInfo,
}

pub struct AudioFile {
    pub(crate) ptr: *mut c_void,
}

impl Drop for AudioFile {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_file_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioFile {
    pub fn open_for_reading(path: impl AsRef<Path>) -> Result<Self, AVAudioError> {
        let path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| AVAudioError::InvalidArgument("path is not valid UTF-8".into()))?;
        let path = CString::new(path)
            .map_err(|error| AVAudioError::InvalidArgument(format!("path contains NUL byte: {error}")))?;
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_file_open_for_reading(path.as_ptr(), &mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FILE_ERROR, err) });
        }
        Ok(Self { ptr })
    }

    pub fn info(&self) -> Result<AudioFileInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_file_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FILE_ERROR, err) });
        }
        parse_json_and_free(json_ptr)
    }

    pub fn length_frames(&self) -> Result<i64, AVAudioError> {
        Ok(self.info()?.length_frames)
    }

    pub fn processing_format(&self) -> Result<AudioFormat, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_file_copy_processing_format(self.ptr) };
        if ptr.is_null() {
            return Err(AVAudioError::FormatError(
                "audio file did not provide a processing format".into(),
            ));
        }
        Ok(AudioFormat { ptr })
    }

    pub fn file_format(&self) -> Result<AudioFormat, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_file_copy_file_format(self.ptr) };
        if ptr.is_null() {
            return Err(AVAudioError::FormatError(
                "audio file did not provide a file format".into(),
            ));
        }
        Ok(AudioFormat { ptr })
    }

    pub fn read_pcm_buffer(&self, frame_count: u32) -> Result<PCMBuffer, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_file_read_pcm_buffer(self.ptr, frame_count, &mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FILE_ERROR, err) });
        }
        Ok(PCMBuffer { ptr })
    }
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

fn parse_json_and_free<T: DeserializeOwned>(json_ptr: *mut c_char) -> Result<T, AVAudioError> {
    let json = unsafe { CStr::from_ptr(json_ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::ava_string_free(json_ptr) };
    serde_json::from_str::<T>(&json)
        .map_err(|error| AVAudioError::OperationFailed(format!("failed to decode bridge JSON: {error}")))
}
