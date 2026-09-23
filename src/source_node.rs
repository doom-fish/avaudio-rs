//! [`AudioSourceNode`] — custom source-node render callbacks.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::type_complexity
)]

use core::ffi::{c_char, c_void};
use core::ptr;

use doom_fish_utils::panic_safe::{catch_user_panic, catch_user_panic_result};

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
        let ptr = unsafe {
            ffi::av_audio_source_node_create(callback_fn, userdata, drop_fn, &raw mut err)
        };
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
                &raw mut err,
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
    let status = catch_user_panic_result("source_render_trampoline", || {
        (state.callback)(&mut context)
    });
    let Some(status) = status else {
        silence_audio_buffer_list(output_data);
        if let Some(is_silence) = is_silence.as_mut() {
            *is_silence = true;
        }
        return ffi::status::CALLBACK_ERROR;
    };
    if let Some(is_silence) = is_silence.as_mut() {
        *is_silence = context.is_silence;
    }
    status
}

#[repr(C)]
struct AudioBufferRaw {
    number_channels: u32,
    data_byte_size: u32,
    data: *mut c_void,
}

#[repr(C)]
struct AudioBufferListRaw {
    number_buffers: u32,
    buffers: [AudioBufferRaw; 1],
}

unsafe fn silence_audio_buffer_list(list: *mut c_void) {
    let list = list.cast::<AudioBufferListRaw>();
    if list.is_null() {
        return;
    }
    let count = (*list).number_buffers as usize;
    let buffers = ptr::addr_of_mut!((*list).buffers).cast::<AudioBufferRaw>();
    for index in 0..count {
        let buffer = buffers.add(index);
        let data = (*buffer).data;
        if !data.is_null() {
            ptr::write_bytes(data.cast::<u8>(), 0, (*buffer).data_byte_size as usize);
        }
    }
}

unsafe extern "C" fn source_render_drop(userdata: *mut c_void) {
    if userdata.is_null() {
        return;
    }
    let state = Box::from_raw(userdata.cast::<SourceRenderState>());
    catch_user_panic("source_render_drop", || drop(state));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    struct TwoBufferList {
        number_buffers: u32,
        buffers: [AudioBufferRaw; 2],
    }

    fn buffer_for(samples: &mut [f32]) -> AudioBufferRaw {
        AudioBufferRaw {
            number_channels: 1,
            data_byte_size: u32::try_from(core::mem::size_of_val(samples))
                .expect("test buffer fits in u32"),
            data: samples.as_mut_ptr().cast(),
        }
    }

    fn render(
        callback: impl FnMut(&mut AudioSourceRenderContext) -> i32 + Send + 'static,
        left: &mut [f32],
        right: &mut [f32],
    ) -> (i32, bool) {
        let mut list = TwoBufferList {
            number_buffers: 2,
            buffers: [buffer_for(left), buffer_for(right)],
        };
        let (trampoline, userdata, drop_fn) = source_render_callback_parts(callback);
        let trampoline = trampoline.expect("source trampoline");
        let drop_fn = drop_fn.expect("source drop callback");
        let mut is_silence = false;
        let status = unsafe {
            trampoline(
                userdata,
                &raw mut is_silence,
                ptr::null(),
                64,
                (&raw mut list).cast(),
            )
        };
        unsafe { drop_fn(userdata) };
        (status, is_silence)
    }

    #[test]
    fn panicking_callback_silences_every_output_buffer() {
        let mut left = vec![0.5_f32; 64];
        let mut right = vec![-0.25_f32; 64];
        let (status, is_silence) = render(|_| panic!("render failure"), &mut left, &mut right);
        assert_eq!(status, ffi::status::CALLBACK_ERROR);
        assert!(is_silence);
        assert!(left
            .iter()
            .chain(&right)
            .all(|sample| sample.to_bits() == 0));
    }

    #[test]
    fn returning_callback_keeps_output_and_reports_its_status() {
        let mut left = vec![0.5_f32; 64];
        let mut right = vec![-0.25_f32; 64];
        let (status, is_silence) = render(
            |context| {
                context.set_is_silence(true);
                7
            },
            &mut left,
            &mut right,
        );
        assert_eq!(status, 7);
        assert!(is_silence);
        assert!(left
            .iter()
            .all(|sample| sample.to_bits() == 0.5_f32.to_bits()));
        assert!(right
            .iter()
            .all(|sample| sample.to_bits() == (-0.25_f32).to_bits()));
    }

    #[test]
    fn silencing_skips_null_buffers_and_null_lists() {
        let mut samples = vec![1.0_f32; 16];
        let mut list = TwoBufferList {
            number_buffers: 2,
            buffers: [
                AudioBufferRaw {
                    number_channels: 1,
                    data_byte_size: 64,
                    data: ptr::null_mut(),
                },
                buffer_for(&mut samples),
            ],
        };
        unsafe {
            silence_audio_buffer_list(ptr::null_mut());
            silence_audio_buffer_list((&raw mut list).cast());
        }
        assert!(samples.iter().all(|sample| sample.to_bits() == 0));
    }

    struct PanicOnDrop(std::sync::Arc<std::sync::atomic::AtomicBool>);

    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            self.0.store(true, std::sync::atomic::Ordering::SeqCst);
            panic!("callback state destructor failure");
        }
    }

    #[test]
    fn panicking_callback_destructor_is_contained() {
        let dropped = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let guard = PanicOnDrop(std::sync::Arc::clone(&dropped));
        let (_, userdata, drop_fn) = source_render_callback_parts(move |_| {
            let _ = &guard;
            0
        });
        unsafe { drop_fn.expect("source drop callback")(userdata) };
        assert!(dropped.load(std::sync::atomic::Ordering::SeqCst));
    }
}
