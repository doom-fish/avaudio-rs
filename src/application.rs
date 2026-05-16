//! [`AudioApplication`] — application-level input mute and record-permission access.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::type_complexity
)]

use core::ffi::{c_char, c_void};
use core::ptr;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;

/// Record-permission state reported by `AVAudioApplication`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioApplicationRecordPermission {
    /// The app has not yet asked for permission.
    Undetermined,
    /// The user denied permission.
    Denied,
    /// The user granted permission.
    Granted,
    /// An unknown future permission state.
    Other(i32),
}

impl AudioApplicationRecordPermission {
    const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Undetermined,
            1 => Self::Denied,
            2 => Self::Granted,
            other => Self::Other(other),
        }
    }
}

struct RecordPermissionCallbackState {
    callback: Box<dyn FnMut(bool) + Send + 'static>,
}

/// Access to the process-wide `AVAudioApplication.shared` singleton.
pub struct AudioApplication;

impl AudioApplication {
    /// Returns the shared application audio object.
    pub const fn shared() -> Self {
        Self
    }

    /// Returns whether the app's audio input is currently muted.
    pub fn input_muted(&self) -> Result<bool, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let value = unsafe { ffi::av_audio_application_get_input_muted(&mut err) };
        if !err.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(value)
    }

    /// Sets the app's audio-input muted state.
    ///
    /// On macOS, Apple requires the host app to install its mute-state-change handler before
    /// this call succeeds.
    pub fn set_input_muted(&self, muted: bool) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::av_audio_application_set_input_muted(muted, &mut err) };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Returns the current record-permission state.
    pub fn record_permission(&self) -> Result<AudioApplicationRecordPermission, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let raw = unsafe { ffi::av_audio_application_get_record_permission(&mut err) };
        if !err.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(AudioApplicationRecordPermission::from_raw(raw))
    }

    /// Requests microphone-record permission.
    ///
    /// The callback may be invoked immediately or later on a different thread.
    pub fn request_record_permission<F>(&self, callback: F) -> Result<(), AVAudioError>
    where
        F: FnMut(bool) + Send + 'static,
    {
        let (callback_fn, userdata, drop_fn) = record_permission_callback_parts(callback);
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_application_request_record_permission(
                callback_fn,
                userdata,
                drop_fn,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            if let Some(drop_fn) = drop_fn {
                unsafe { drop_fn(userdata) };
            }
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }
}

fn record_permission_callback_parts<F>(
    callback: F,
) -> (
    Option<ffi::BoolCallback>,
    *mut c_void,
    Option<ffi::DropCallback>,
)
where
    F: FnMut(bool) + Send + 'static,
{
    let state = Box::new(RecordPermissionCallbackState {
        callback: Box::new(callback),
    });
    (
        Some(record_permission_trampoline),
        Box::into_raw(state).cast::<c_void>(),
        Some(record_permission_drop),
    )
}

unsafe extern "C" fn record_permission_trampoline(userdata: *mut c_void, granted: bool) {
    let Some(state) = userdata.cast::<RecordPermissionCallbackState>().as_mut() else {
        return;
    };
    (state.callback)(granted);
}

unsafe extern "C" fn record_permission_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    drop(Box::from_raw(userdata.cast::<RecordPermissionCallbackState>()));
}
