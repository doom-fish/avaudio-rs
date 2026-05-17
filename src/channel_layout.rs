//! [`AVAudioChannelLayout`] wrappers.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ptr;

use serde::Deserialize;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::types::AudioChannelCount;
use crate::util::parse_json_and_free;

/// Mirrors `AudioChannelLayoutTag`.
pub type AudioChannelLayoutTag = u32;

/// Mirrors `kAudioChannelLayoutTag_Mono`.
pub const AUDIO_CHANNEL_LAYOUT_TAG_MONO: AudioChannelLayoutTag = (100_u32 << 16) | 1;
/// Mirrors `kAudioChannelLayoutTag_Stereo`.
pub const AUDIO_CHANNEL_LAYOUT_TAG_STEREO: AudioChannelLayoutTag = (101_u32 << 16) | 2;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioChannelLayoutInfo {
    pub layout_tag: AudioChannelLayoutTag,
    pub channel_count: AudioChannelCount,
}

/// Wraps an `AVAudioChannelLayout`.
pub struct AudioChannelLayout {
    ptr: *mut c_void,
}

unsafe impl Send for AudioChannelLayout {}

impl Drop for AudioChannelLayout {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_channel_layout_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioChannelLayout {
    /// Creates a channel layout from an `AudioChannelLayoutTag`.
    pub fn new_with_layout_tag(layout_tag: AudioChannelLayoutTag) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr =
            unsafe { ffi::av_audio_channel_layout_create_with_layout_tag(layout_tag, &mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }

    /// Creates a mono layout.
    pub fn mono() -> Result<Self, AVAudioError> {
        Self::new_with_layout_tag(AUDIO_CHANNEL_LAYOUT_TAG_MONO)
    }

    /// Creates a stereo layout.
    pub fn stereo() -> Result<Self, AVAudioError> {
        Self::new_with_layout_tag(AUDIO_CHANNEL_LAYOUT_TAG_STEREO)
    }

    /// Returns layout metadata.
    pub fn info(&self) -> Result<AudioChannelLayoutInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_channel_layout_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns the layout tag.
    pub fn layout_tag(&self) -> Result<AudioChannelLayoutTag, AVAudioError> {
        Ok(self.info()?.layout_tag)
    }

    /// Returns the channel count.
    pub fn channel_count(&self) -> Result<AudioChannelCount, AVAudioError> {
        Ok(self.info()?.channel_count)
    }

    /// Compares two channel layouts for equality.
    pub fn equals(&self, other: &Self) -> bool {
        unsafe { ffi::av_audio_channel_layout_is_equal(self.ptr, other.ptr) }
    }
}
