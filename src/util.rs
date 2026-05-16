use core::ffi::c_char;
use std::ffi::CStr;

use serde::de::DeserializeOwned;

use crate::error::AVAudioError;
use crate::ffi;

pub fn parse_json_and_free<T: DeserializeOwned>(json_ptr: *mut c_char) -> Result<T, AVAudioError> {
    let json = unsafe { CStr::from_ptr(json_ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::ava_string_free(json_ptr) };
    serde_json::from_str::<T>(&json).map_err(|error| {
        AVAudioError::OperationFailed(format!("failed to decode bridge JSON: {error}"))
    })
}
