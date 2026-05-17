//! [`AVAudioRoutingArbiter`] wrappers.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CStr;

use serde::Deserialize;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;

/// Mirrors `AVAudioRoutingArbitrationCategory`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioRoutingArbitrationCategory {
    Playback,
    PlayAndRecord,
    PlayAndRecordVoice,
    Other(i64),
}

impl AudioRoutingArbitrationCategory {
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Playback,
            1 => Self::PlayAndRecord,
            2 => Self::PlayAndRecordVoice,
            other => Self::Other(other),
        }
    }

    const fn as_raw(self) -> i64 {
        match self {
            Self::Playback => 0,
            Self::PlayAndRecord => 1,
            Self::PlayAndRecordVoice => 2,
            Self::Other(other) => other,
        }
    }
}

struct RoutingArbitrationState {
    callback: Box<dyn FnMut(bool, Option<String>) + Send + 'static>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RoutingArbitrationResult {
    default_device_changed: bool,
    error: Option<String>,
}

/// Access to the process-wide routing-arbitration singleton.
pub struct AudioRoutingArbiter;

impl AudioRoutingArbiter {
    /// Returns the shared routing arbiter.
    pub const fn shared() -> Self {
        Self
    }

    /// Begins routing arbitration.
    pub fn begin<F>(
        &self,
        category: AudioRoutingArbitrationCategory,
        callback: F,
    ) -> Result<(), AVAudioError>
    where
        F: FnMut(bool, Option<String>) + Send + 'static,
    {
        let state = Box::new(RoutingArbitrationState {
            callback: Box::new(callback),
        });
        let userdata = Box::into_raw(state).cast::<c_void>();
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_routing_arbiter_begin(
                category.as_raw(),
                Some(routing_begin_result_trampoline),
                userdata,
                Some(routing_begin_drop),
                &mut err,
            )
        };
        if status != ffi::status::OK {
            unsafe { routing_begin_drop(userdata) };
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Leaves routing arbitration.
    pub fn leave(&self) {
        unsafe { ffi::av_audio_routing_arbiter_leave() };
    }
}

unsafe extern "C" fn routing_begin_result_trampoline(userdata: *mut c_void, message: *mut c_char) {
    let Some(state) = userdata.cast::<RoutingArbitrationState>().as_mut() else {
        return;
    };
    if message.is_null() {
        (state.callback)(false, None);
        return;
    }
    let json = CStr::from_ptr(message).to_string_lossy().into_owned();
    unsafe { ffi::ava_string_free(message) };
    match serde_json::from_str::<RoutingArbitrationResult>(&json) {
        Ok(result) => (state.callback)(result.default_device_changed, result.error),
        Err(_) => (state.callback)(false, Some(json)),
    }
}

unsafe extern "C" fn routing_begin_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    drop(Box::from_raw(userdata.cast::<RoutingArbitrationState>()));
}
