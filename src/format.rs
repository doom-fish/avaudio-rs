#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use core::ffi::{c_char, c_void};
use core::ptr;
use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::util::parse_json_and_free;

#[derive(Debug, Clone, PartialEq)]
pub struct AudioFormatInfo {
    pub common_format: i32,
    pub sample_rate: f64,
    pub channel_count: u32,
    pub is_interleaved: bool,
}

crate::util::impl_from_json!(AudioFormatInfo {
    common_format: i32 = "commonFormat",
    sample_rate: f64 = "sampleRate",
    channel_count: u32 = "channelCount",
    is_interleaved: bool = "isInterleaved",
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioCommonFormat {
    Other,
    PcmFloat32,
    PcmFloat64,
    PcmInt16,
    PcmInt32,
}

impl AudioCommonFormat {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::PcmFloat32,
            2 => Self::PcmFloat64,
            3 => Self::PcmInt16,
            4 => Self::PcmInt32,
            _ => Self::Other,
        }
    }
}

pub struct AudioFormat {
    pub(crate) ptr: *mut c_void,
}

impl Drop for AudioFormat {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_format_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioFormat {
    pub fn standard(
        sample_rate: f64,
        channel_count: u32,
        interleaved: bool,
    ) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_format_create_standard(
                sample_rate,
                channel_count,
                interleaved,
                &raw mut err,
            )
        };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FORMAT_ERROR, err) });
        }
        Ok(Self { ptr })
    }

    pub fn with_common_format(
        common_format: AudioCommonFormat,
        sample_rate: f64,
        channel_count: u32,
        interleaved: bool,
    ) -> Result<Self, AVAudioError> {
        let raw = match common_format {
            AudioCommonFormat::PcmFloat32 => 1,
            AudioCommonFormat::PcmFloat64 => 2,
            AudioCommonFormat::PcmInt16 => 3,
            AudioCommonFormat::PcmInt32 => 4,
            AudioCommonFormat::Other => {
                return Err(AVAudioError::InvalidArgument(
                    "AudioCommonFormat::Other has no PCM sample layout".into(),
                ))
            }
        };
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_format_create_with_common_format(
                raw,
                sample_rate,
                channel_count,
                interleaved,
                &raw mut err,
            )
        };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FORMAT_ERROR, err) });
        }
        Ok(Self { ptr })
    }

    pub fn info(&self) -> Result<AudioFormatInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_format_info_json(self.ptr, &raw mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FORMAT_ERROR, err) });
        }
        parse_json_and_free(json_ptr)
    }

    pub fn common_format(&self) -> Result<AudioCommonFormat, AVAudioError> {
        Ok(AudioCommonFormat::from_raw(self.info()?.common_format))
    }

    pub fn sample_rate(&self) -> Result<f64, AVAudioError> {
        Ok(self.info()?.sample_rate)
    }

    pub fn channel_count(&self) -> Result<u32, AVAudioError> {
        Ok(self.info()?.channel_count)
    }

    pub fn is_interleaved(&self) -> Result<bool, AVAudioError> {
        Ok(self.info()?.is_interleaved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_format_maps_known_raw_values() {
        assert_eq!(AudioCommonFormat::from_raw(1), AudioCommonFormat::PcmFloat32);
        assert_eq!(AudioCommonFormat::from_raw(2), AudioCommonFormat::PcmFloat64);
        assert_eq!(AudioCommonFormat::from_raw(3), AudioCommonFormat::PcmInt16);
        assert_eq!(AudioCommonFormat::from_raw(4), AudioCommonFormat::PcmInt32);
    }

    #[test]
    fn common_format_maps_unknown_values_to_other() {
        assert_eq!(AudioCommonFormat::from_raw(-1), AudioCommonFormat::Other);
    }

    #[test]
    fn audio_format_info_deserializes_bridge_shape() {
        let info: AudioFormatInfo = crate::util::from_json(
            r#"{"commonFormat":3,"sampleRate":48000.0,"channelCount":2,"isInterleaved":true}"#,
        )
        .unwrap();

        assert_eq!(
            info,
            AudioFormatInfo {
                common_format: 3,
                sample_rate: 48_000.0,
                channel_count: 2,
                is_interleaved: true,
            },
        );
    }
}
