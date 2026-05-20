//! Errors produced by the `AVAudio` bridge.

use core::fmt;

use crate::ffi;

/// Top-level error type returned by fallible APIs in this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AVAudioError {
    /// Invalid caller input (UTF-8 / NUL / unsupported configuration).
    InvalidArgument(String),
    /// File or format creation failed.
    FormatError(String),
    /// Audio file I/O failed.
    FileError(String),
    /// Engine creation or graph configuration failed.
    EngineError(String),
    /// Player-node creation or scheduling failed.
    PlayerError(String),
    /// Callback installation or dispatch failed.
    CallbackError(String),
    /// A generic operation failed.
    OperationFailed(String),
}

impl fmt::Display for AVAudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::FormatError(message) => write!(f, "audio format error: {message}"),
            Self::FileError(message) => write!(f, "audio file error: {message}"),
            Self::EngineError(message) => write!(f, "audio engine error: {message}"),
            Self::PlayerError(message) => write!(f, "audio player error: {message}"),
            Self::CallbackError(message) => write!(f, "audio callback error: {message}"),
            Self::OperationFailed(message) => write!(f, "operation failed: {message}"),
        }
    }
}

impl std::error::Error for AVAudioError {}

pub unsafe fn from_swift(status: i32, error_str: *mut core::ffi::c_char) -> AVAudioError {
    let message = if error_str.is_null() {
        String::new()
    } else {
        let s = core::ffi::CStr::from_ptr(error_str)
            .to_string_lossy()
            .into_owned();
        ffi::ava_string_free(error_str);
        s
    };

    match status {
        ffi::status::INVALID_ARGUMENT => AVAudioError::InvalidArgument(message),
        ffi::status::FORMAT_ERROR => AVAudioError::FormatError(message),
        ffi::status::FILE_ERROR => AVAudioError::FileError(message),
        ffi::status::ENGINE_ERROR => AVAudioError::EngineError(message),
        ffi::status::PLAYER_ERROR => AVAudioError::PlayerError(message),
        ffi::status::CALLBACK_ERROR => AVAudioError::CallbackError(message),
        ffi::status::OPERATION_FAILED => AVAudioError::OperationFailed(message),
        _ => AVAudioError::OperationFailed(format!("unknown status {status}: {message}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    #[test]
    fn display_formats_format_errors() {
        assert_eq!(
            AVAudioError::FormatError("bad format".into()).to_string(),
            "audio format error: bad format",
        );
    }

    #[test]
    fn display_formats_callback_errors() {
        assert_eq!(
            AVAudioError::CallbackError("callback dropped".into()).to_string(),
            "audio callback error: callback dropped",
        );
    }

    #[test]
    fn from_swift_maps_known_status_codes() {
        unsafe {
            assert_eq!(
                from_swift(ffi::status::FILE_ERROR, ptr::null_mut()),
                AVAudioError::FileError(String::new()),
            );
            assert_eq!(
                from_swift(ffi::status::ENGINE_ERROR, ptr::null_mut()),
                AVAudioError::EngineError(String::new()),
            );
        }
    }

    #[test]
    fn from_swift_uses_operation_failed_for_unknown_statuses() {
        unsafe {
            assert_eq!(
                from_swift(77, ptr::null_mut()),
                AVAudioError::OperationFailed("unknown status 77: ".into()),
            );
        }
    }
}
