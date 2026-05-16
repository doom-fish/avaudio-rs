//! [`AudioSinkNode`] — custom sink-node receiver callbacks.

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
use crate::node::AudioNodeHandle;

/// Per-render context supplied to an `AudioSinkNode` callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioSinkRenderContext {
    timestamp_ptr: *const c_void,
    frame_count: u32,
    input_data_ptr: *const c_void,
}

impl AudioSinkRenderContext {
    /// Returns the raw `AudioTimeStamp *` pointer valid for the current callback.
    pub const fn timestamp_ptr(self) -> *const c_void {
        self.timestamp_ptr
    }

    /// Returns the number of input frames provided by the engine.
    pub const fn frame_count(self) -> u32 {
        self.frame_count
    }

    /// Returns the raw immutable `AudioBufferList *` pointer valid for the current callback.
    pub const fn input_data_ptr(self) -> *const c_void {
        self.input_data_ptr
    }
}

struct SinkRenderState {
    callback: Box<dyn FnMut(&AudioSinkRenderContext) -> i32 + Send + 'static>,
}

/// Wraps an `AVAudioSinkNode`.
pub struct AudioSinkNode {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioSinkNode {}

impl Drop for AudioSinkNode {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_sink_node_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioSinkNode {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioSinkNode {
    /// Creates a sink node that receives audio on the engine render thread.
    pub fn new<F>(callback: F) -> Result<Self, AVAudioError>
    where
        F: FnMut(&AudioSinkRenderContext) -> i32 + Send + 'static,
    {
        let (callback_fn, userdata, drop_fn) = sink_render_callback_parts(callback);
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe { ffi::av_audio_sink_node_create(callback_fn, userdata, drop_fn, &mut err) };
        if ptr.is_null() {
            if let Some(drop_fn) = drop_fn {
                unsafe { drop_fn(userdata) };
            }
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }
}

fn sink_render_callback_parts<F>(
    callback: F,
) -> (
    Option<ffi::SinkNodeReceiverCallback>,
    *mut c_void,
    Option<ffi::DropCallback>,
)
where
    F: FnMut(&AudioSinkRenderContext) -> i32 + Send + 'static,
{
    let state = Box::new(SinkRenderState {
        callback: Box::new(callback),
    });
    (
        Some(sink_render_trampoline),
        Box::into_raw(state).cast::<c_void>(),
        Some(sink_render_drop),
    )
}

unsafe extern "C" fn sink_render_trampoline(
    userdata: *mut c_void,
    timestamp: *const c_void,
    frame_count: u32,
    input_data: *const c_void,
) -> i32 {
    let Some(state) = userdata.cast::<SinkRenderState>().as_mut() else {
        return ffi::status::CALLBACK_ERROR;
    };
    let context = AudioSinkRenderContext {
        timestamp_ptr: timestamp,
        frame_count,
        input_data_ptr: input_data,
    };
    (state.callback)(&context)
}

unsafe extern "C" fn sink_render_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    drop(Box::from_raw(userdata.cast::<SinkRenderState>()));
}
