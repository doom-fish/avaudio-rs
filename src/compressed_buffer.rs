//! [`AVAudioCompressedBuffer`] wrappers.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ptr;

use serde::Deserialize;

use crate::buffer::AudioBufferHandle;
use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::format::AudioFormat;
use crate::types::AudioPacketCount;
use crate::util::parse_json_and_free;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioCompressedBufferInfo {
    pub packet_capacity: AudioPacketCount,
    pub packet_count: AudioPacketCount,
    pub maximum_packet_size: usize,
    pub byte_capacity: u32,
    pub byte_length: u32,
}

/// Wraps an `AVAudioCompressedBuffer`.
pub struct AudioCompressedBuffer {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioCompressedBuffer {}

impl Drop for AudioCompressedBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_compressed_buffer_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioBufferHandle for AudioCompressedBuffer {
    fn as_buffer_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioCompressedBuffer {
    /// Allocates a compressed-audio buffer.
    pub fn new(
        format: &AudioFormat,
        packet_capacity: AudioPacketCount,
        maximum_packet_size: usize,
    ) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_compressed_buffer_create(
                format.ptr,
                packet_capacity,
                maximum_packet_size,
                &mut err,
            )
        };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FORMAT_ERROR, err) });
        }
        Ok(Self { ptr })
    }

    /// Returns compressed-buffer metadata.
    pub fn info(&self) -> Result<AudioCompressedBufferInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_compressed_buffer_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns the packet capacity.
    pub fn packet_capacity(&self) -> Result<AudioPacketCount, AVAudioError> {
        Ok(self.info()?.packet_capacity)
    }

    /// Returns the current packet count.
    pub fn packet_count(&self) -> Result<AudioPacketCount, AVAudioError> {
        Ok(self.info()?.packet_count)
    }

    /// Sets the current packet count.
    pub fn set_packet_count(&self, packet_count: AudioPacketCount) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_compressed_buffer_set_packet_count(self.ptr, packet_count, &mut err)
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Returns the maximum packet size in bytes.
    pub fn maximum_packet_size(&self) -> Result<usize, AVAudioError> {
        Ok(self.info()?.maximum_packet_size)
    }

    /// Returns the byte capacity.
    pub fn byte_capacity(&self) -> Result<u32, AVAudioError> {
        Ok(self.info()?.byte_capacity)
    }

    /// Returns the byte length.
    pub fn byte_length(&self) -> Result<u32, AVAudioError> {
        Ok(self.info()?.byte_length)
    }

    /// Sets the byte length.
    pub fn set_byte_length(&self, byte_length: u32) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_compressed_buffer_set_byte_length(self.ptr, byte_length, &mut err)
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Copies the compressed format metadata object.
    pub fn format(&self) -> Result<AudioFormat, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_compressed_buffer_copy_format(self.ptr) };
        if ptr.is_null() {
            return Err(AVAudioError::FormatError(
                "compressed buffer did not provide a format".into(),
            ));
        }
        Ok(AudioFormat { ptr })
    }
}
