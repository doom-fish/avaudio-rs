#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CStr;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use std::ops::{BitOr, BitOrAssign};

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::file::{AudioFile, PCMBuffer};
use crate::mixing::AudioMixingHandle;
use crate::node::AudioNodeHandle;
use crate::time::AudioTime;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioPlayerNodeInfo {
    pub is_playing: bool,
}

struct CompletionState {
    callback: Box<dyn FnMut() + Send + 'static>,
}

struct TypedCompletionState {
    callback: Box<dyn FnMut(AudioPlayerNodeCompletionCallbackType) + Send + 'static>,
}

/// Mirrors `AVAudioPlayerNodeBufferOptions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AudioPlayerNodeBufferOptions {
    raw: u64,
}

impl AudioPlayerNodeBufferOptions {
    pub const NONE: Self = Self { raw: 0 };
    pub const LOOPS: Self = Self { raw: 1 << 0 };
    pub const INTERRUPTS: Self = Self { raw: 1 << 1 };
    pub const INTERRUPTS_AT_LOOP: Self = Self { raw: 1 << 2 };

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.raw
    }
}

impl BitOr for AudioPlayerNodeBufferOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw | rhs.raw }
    }
}

impl BitOrAssign for AudioPlayerNodeBufferOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.raw |= rhs.raw;
    }
}

/// Mirrors `AVAudioPlayerNodeCompletionCallbackType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioPlayerNodeCompletionCallbackType {
    DataConsumed,
    DataRendered,
    DataPlayedBack,
    Other(i64),
}

impl AudioPlayerNodeCompletionCallbackType {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::DataConsumed,
            1 => Self::DataRendered,
            2 => Self::DataPlayedBack,
            other => Self::Other(other),
        }
    }

    const fn as_raw(self) -> i64 {
        match self {
            Self::DataConsumed => 0,
            Self::DataRendered => 1,
            Self::DataPlayedBack => 2,
            Self::Other(other) => other,
        }
    }
}

pub struct AudioPlayerNode {
    pub(crate) ptr: *mut c_void,
}

impl Drop for AudioPlayerNode {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_player_node_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioPlayerNode {
    fn as_node_ptr(&self) -> *mut c_void {
        unsafe { ffi::av_audio_player_node_get_node_unretained(self.ptr) }
    }
}

impl AudioMixingHandle for AudioPlayerNode {
    fn as_mixing_ptr(&self) -> *mut c_void {
        self.as_node_ptr()
    }
}

impl AudioPlayerNode {
    pub fn new() -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_player_node_create(&mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::PLAYER_ERROR, err) });
        }
        Ok(Self { ptr })
    }

    pub fn info(&self) -> Result<AudioPlayerNodeInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_player_node_info_json(self.ptr, &mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::PLAYER_ERROR, err) });
        }
        parse_json_and_free(json_ptr)
    }

    pub fn is_playing(&self) -> Result<bool, AVAudioError> {
        Ok(self.info()?.is_playing)
    }

    pub fn play(&self) {
        unsafe { ffi::av_audio_player_node_play(self.ptr) };
    }

    pub fn pause(&self) {
        unsafe { ffi::av_audio_player_node_pause(self.ptr) };
    }

    pub fn stop(&self) {
        unsafe { ffi::av_audio_player_node_stop(self.ptr) };
    }

    pub fn schedule_buffer(&self, buffer: &PCMBuffer) -> Result<(), AVAudioError> {
        self.schedule_buffer_with_optional_completion(buffer, None::<fn()>)
    }

    pub fn schedule_buffer_with_completion<F>(
        &self,
        buffer: &PCMBuffer,
        callback: F,
    ) -> Result<(), AVAudioError>
    where
        F: FnMut() + Send + 'static,
    {
        self.schedule_buffer_with_optional_completion(buffer, Some(callback))
    }

    pub fn schedule_file(&self, file: &AudioFile) -> Result<(), AVAudioError> {
        self.schedule_file_with_optional_completion(file, None::<fn()>)
    }

    pub fn schedule_file_with_completion<F>(
        &self,
        file: &AudioFile,
        callback: F,
    ) -> Result<(), AVAudioError>
    where
        F: FnMut() + Send + 'static,
    {
        self.schedule_file_with_optional_completion(file, Some(callback))
    }

    /// Schedules a buffer with explicit timing/options but without a completion callback.
    pub fn schedule_buffer_with_options(
        &self,
        buffer: &PCMBuffer,
        when: Option<&AudioTime>,
        options: AudioPlayerNodeBufferOptions,
    ) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_player_node_schedule_buffer_with_options(
                self.ptr,
                buffer.ptr,
                when.map_or(ptr::null_mut(), |when| when.ptr),
                options.bits(),
                None,
                ptr::null_mut(),
                None,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Schedules a buffer with a typed completion callback.
    pub fn schedule_buffer_with_callback_type<F>(
        &self,
        buffer: &PCMBuffer,
        when: Option<&AudioTime>,
        options: AudioPlayerNodeBufferOptions,
        callback_type: AudioPlayerNodeCompletionCallbackType,
        callback: F,
    ) -> Result<(), AVAudioError>
    where
        F: FnMut(AudioPlayerNodeCompletionCallbackType) + Send + 'static,
    {
        let state = Box::new(TypedCompletionState {
            callback: Box::new(callback),
        });
        let userdata = Box::into_raw(state).cast::<c_void>();
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_player_node_schedule_buffer_with_callback_type(
                self.ptr,
                buffer.ptr,
                when.map_or(ptr::null_mut(), |when| when.ptr),
                options.bits(),
                callback_type.as_raw(),
                Some(typed_completion_trampoline),
                userdata,
                Some(typed_completion_drop),
                &mut err,
            )
        };
        if status != ffi::status::OK {
            unsafe { typed_completion_drop(userdata) };
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    /// Schedules a file with a typed completion callback.
    pub fn schedule_file_with_callback_type<F>(
        &self,
        file: &AudioFile,
        when: Option<&AudioTime>,
        callback_type: AudioPlayerNodeCompletionCallbackType,
        callback: F,
    ) -> Result<(), AVAudioError>
    where
        F: FnMut(AudioPlayerNodeCompletionCallbackType) + Send + 'static,
    {
        let state = Box::new(TypedCompletionState {
            callback: Box::new(callback),
        });
        let userdata = Box::into_raw(state).cast::<c_void>();
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_player_node_schedule_file_with_callback_type(
                self.ptr,
                file.ptr,
                when.map_or(ptr::null_mut(), |when| when.ptr),
                callback_type.as_raw(),
                Some(typed_completion_trampoline),
                userdata,
                Some(typed_completion_drop),
                &mut err,
            )
        };
        if status != ffi::status::OK {
            unsafe { typed_completion_drop(userdata) };
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    fn schedule_buffer_with_optional_completion<F>(
        &self,
        buffer: &PCMBuffer,
        callback: Option<F>,
    ) -> Result<(), AVAudioError>
    where
        F: FnMut() + Send + 'static,
    {
        let (callback_fn, userdata, drop_fn) = completion_parts(callback);
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_player_node_schedule_buffer(
                self.ptr,
                buffer.ptr,
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

    fn schedule_file_with_optional_completion<F>(
        &self,
        file: &AudioFile,
        callback: Option<F>,
    ) -> Result<(), AVAudioError>
    where
        F: FnMut() + Send + 'static,
    {
        let (callback_fn, userdata, drop_fn) = completion_parts(callback);
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_player_node_schedule_file(
                self.ptr,
                file.ptr,
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

fn completion_parts<F>(
    callback: Option<F>,
) -> (
    Option<ffi::SimpleCallback>,
    *mut c_void,
    Option<ffi::DropCallback>,
)
where
    F: FnMut() + Send + 'static,
{
    callback.map_or((None, ptr::null_mut(), None), |callback| {
        let state = Box::new(CompletionState {
            callback: Box::new(callback),
        });
        (
            Some(completion_trampoline),
            Box::into_raw(state).cast::<c_void>(),
            Some(completion_drop),
        )
    })
}

unsafe extern "C" fn completion_trampoline(userdata: *mut c_void) {
    let Some(state) = userdata.cast::<CompletionState>().as_mut() else {
        return;
    };
    (state.callback)();
}

unsafe extern "C" fn completion_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    drop(Box::from_raw(userdata.cast::<CompletionState>()));
}

unsafe extern "C" fn typed_completion_trampoline(userdata: *mut c_void, value: i64) {
    let Some(state) = userdata.cast::<TypedCompletionState>().as_mut() else {
        return;
    };
    (state.callback)(AudioPlayerNodeCompletionCallbackType::from_raw(value));
}

unsafe extern "C" fn typed_completion_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    drop(Box::from_raw(userdata.cast::<TypedCompletionState>()));
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
