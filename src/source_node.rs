//! [`AudioSourceNode`] — custom source-node render callbacks.

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
use crate::format::AudioFormat;
use crate::node::AudioNodeHandle;

/// Per-render context supplied to an `AudioSourceNode` callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioSourceRenderContext {
    is_silence: bool,
    timestamp_ptr: *const c_void,
    frame_count: u32,
    output_data_ptr: *mut c_void,
}

impl AudioSourceRenderContext {
    /// Returns the silence hint supplied by the engine.
    pub const fn is_silence(self) -> bool {
        self.is_silence
    }

    /// Updates the silence hint that will be returned to the engine.
    pub fn set_is_silence(&mut self, is_silence: bool) {
        self.is_silence = is_silence;
    }

    /// Returns the raw `AudioTimeStamp *` pointer valid for the current callback.
    pub const fn timestamp_ptr(self) -> *const c_void {
        self.timestamp_ptr
    }

    /// Returns the requested frame count.
    pub const fn frame_count(self) -> u32 {
        self.frame_count
    }

    /// Returns the raw mutable `AudioBufferList *` pointer valid for the current callback.
    pub const fn output_data_ptr(self) -> *mut c_void {
        self.output_data_ptr
    }
}

struct SourceRenderState {
    callback: Box<dyn FnMut(&mut AudioSourceRenderContext) -> i32 + Send + 'static>,
}

/// Wraps an `AVAudioSourceNode`.
pub struct AudioSourceNode {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioSourceNode {}

impl Drop for AudioSourceNode {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_source_node_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioSourceNode {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioSourceNode {
    /// Creates a source node whose output format follows the graph connection format.
    pub fn new<F>(callback: F) -> Result<Self, AVAudioError>
    where
        F: FnMut(&mut AudioSourceRenderContext) -> i32 + Send + 'static,
    {
        let (callback_fn, userdata, drop_fn) = source_render_callback_parts(callback);
        let mut err: *mut c_char = ptr::null_mut();
        let ptr =
            unsafe { ffi::av_audio_source_node_create(callback_fn, userdata, drop_fn, &mut err) };
        if ptr.is_null() {
            if let Some(drop_fn) = drop_fn {
                unsafe { drop_fn(userdata) };
            }
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }

    /// Creates a source node with a fixed render-block format.
    pub fn new_with_format<F>(format: &AudioFormat, callback: F) -> Result<Self, AVAudioError>
    where
        F: FnMut(&mut AudioSourceRenderContext) -> i32 + Send + 'static,
    {
        let (callback_fn, userdata, drop_fn) = source_render_callback_parts(callback);
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_source_node_create_with_format(
                format.ptr,
                callback_fn,
                userdata,
                drop_fn,
                &mut err,
            )
        };
        if ptr.is_null() {
            if let Some(drop_fn) = drop_fn {
                unsafe { drop_fn(userdata) };
            }
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }
}

fn source_render_callback_parts<F>(
    callback: F,
) -> (
    Option<ffi::SourceNodeRenderCallback>,
    *mut c_void,
    Option<ffi::DropCallback>,
)
where
    F: FnMut(&mut AudioSourceRenderContext) -> i32 + Send + 'static,
{
    let state = Box::new(SourceRenderState {
        callback: Box::new(callback),
    });
    (
        Some(source_render_trampoline),
        Box::into_raw(state).cast::<c_void>(),
        Some(source_render_drop),
    )
}

unsafe extern "C" fn source_render_trampoline(
    userdata: *mut c_void,
    is_silence: *mut bool,
    timestamp: *const c_void,
    frame_count: u32,
    output_data: *mut c_void,
) -> i32 {
    let Some(state) = userdata.cast::<SourceRenderState>().as_mut() else {
        return ffi::status::CALLBACK_ERROR;
    };
    let mut context = AudioSourceRenderContext {
        is_silence: is_silence.as_ref().copied().unwrap_or(false),
        timestamp_ptr: timestamp,
        frame_count,
        output_data_ptr: output_data,
    };
    let status = (state.callback)(&mut context);
    if let Some(is_silence) = is_silence.as_mut() {
        *is_silence = context.is_silence;
    }
    status
}

unsafe extern "C" fn source_render_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    drop(Box::from_raw(userdata.cast::<SourceRenderState>()));
}
