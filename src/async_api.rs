//! Async stream wrappers for `AVFAudio` notifications, delegates, and taps.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CStr;

use doom_fish_utils::panic_safe::catch_user_panic;
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};

use crate::audio_file::AudioFile;
use crate::engine::AudioEngine;
use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::format::AudioFormat;
use crate::node::AudioNodeHandle;
use crate::pcm_buffer::PCMBuffer;
use crate::player::{AudioPlayerNode, AudioPlayerNodeBufferOptions};
use crate::recorder::AudioRecorder;
use crate::simple_player::AudioSimplePlayer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigChangeEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlayerNodeCompletionEvent {
    DataConsumed,
    DataRendered,
    DataPlayedBack,
    Other(i64),
}

impl PlayerNodeCompletionEvent {
    fn from_kind(kind: i32) -> Self {
        match kind {
            0 => Self::DataConsumed,
            1 => Self::DataRendered,
            2 => Self::DataPlayedBack,
            other => Self::Other(i64::from(other)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RecorderEvent {
    DidFinishRecording { successfully: bool },
    EncodeError { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SimplePlayerEvent {
    DidFinishPlaying { successfully: bool },
    DecodeError { message: String },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TapBufferEvent {
    pub frame_length: u32,
    pub channel_count: u32,
    pub sample_rate: f64,
}

#[repr(C)]
struct TapEventPayloadRaw {
    frame_length: u32,
    channel_count: u32,
    sample_rate: f64,
}

fn drop_sender<T>(sender_raw: &mut *mut AsyncStreamSender<T>) {
    if !(*sender_raw).is_null() {
        // SAFETY: `*sender_raw` was produced by `Box::into_raw` inside the
        // corresponding `subscribe*` constructor and this path runs at most
        // once (the pointer is immediately zeroed below so it cannot be
        // reached a second time).
        unsafe { drop(Box::from_raw(*sender_raw)) };
        *sender_raw = ptr::null_mut();
    }
}

pub struct ConfigChangeStream {
    inner: BoundedAsyncStream<ConfigChangeEvent>,
    bridge_ptr: *mut c_void,
    sender_raw: *mut AsyncStreamSender<ConfigChangeEvent>,
}

// SAFETY: `bridge_ptr` is an AVFoundation opaque handle whose
// subscription/unsubscribe APIs are thread-safe per Apple documentation.
// `sender_raw` is a heap-allocated `Box` that is only touched from a
// single thread at a time (subscribe on construction, Drop on teardown).
// `BoundedAsyncStream` is itself `Send`.
unsafe impl Send for ConfigChangeStream {}

impl Drop for ConfigChangeStream {
    fn drop(&mut self) {
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_engine_config_change_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
        drop_sender(&mut self.sender_raw);
    }
}

unsafe extern "C" fn config_change_cb(_kind: i32, _payload: *const c_void, ctx: *mut c_void) {
    catch_user_panic("config_change_cb", || {
        let Some(sender) = ctx.cast::<AsyncStreamSender<ConfigChangeEvent>>().as_ref() else {
            return;
        };
        sender.push(ConfigChangeEvent);
    });
}

impl ConfigChangeStream {
    pub fn subscribe(engine: &AudioEngine, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_raw = Box::into_raw(Box::new(sender));
        let bridge_ptr = unsafe {
            ffi::ava_engine_config_change_subscribe(
                engine.as_engine_ptr(),
                config_change_cb,
                sender_raw.cast::<c_void>(),
            )
        };
        Self {
            inner: stream,
            bridge_ptr,
            sender_raw,
        }
    }

    pub const fn next(&self) -> NextItem<'_, ConfigChangeEvent> {
        self.inner.next()
    }

    pub fn try_next(&self) -> Option<ConfigChangeEvent> {
        self.inner.try_next()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

pub struct PlayerNodeCompletionStream {
    inner: BoundedAsyncStream<PlayerNodeCompletionEvent>,
    bridge_ptr: *mut c_void,
    sender_raw: *mut AsyncStreamSender<PlayerNodeCompletionEvent>,
}

// SAFETY: Same rationale as `ConfigChangeStream`. The player node handle
// is thread-safe per Apple documentation and `sender_raw` is single-owner.
unsafe impl Send for PlayerNodeCompletionStream {}

impl Drop for PlayerNodeCompletionStream {
    fn drop(&mut self) {
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_player_node_stream_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
        drop_sender(&mut self.sender_raw);
    }
}

unsafe extern "C" fn player_completion_cb(kind: i32, _payload: *const c_void, ctx: *mut c_void) {
    catch_user_panic("player_completion_cb", || {
        let Some(sender) = ctx
            .cast::<AsyncStreamSender<PlayerNodeCompletionEvent>>()
            .as_ref()
        else {
            return;
        };
        sender.push(PlayerNodeCompletionEvent::from_kind(kind));
    });
}

impl PlayerNodeCompletionStream {
    pub fn subscribe(player: &AudioPlayerNode, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_raw = Box::into_raw(Box::new(sender));
        let bridge_ptr = unsafe {
            ffi::ava_player_node_stream_subscribe(
                player.ptr,
                player_completion_cb,
                sender_raw.cast::<c_void>(),
            )
        };
        Self {
            inner: stream,
            bridge_ptr,
            sender_raw,
        }
    }

    pub fn schedule_buffer(
        &self,
        buffer: &PCMBuffer,
        options: AudioPlayerNodeBufferOptions,
    ) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::ava_player_node_stream_schedule_buffer(
                self.bridge_ptr,
                buffer.ptr,
                options.bits(),
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    pub fn schedule_file(&self, file: &AudioFile) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::ava_player_node_stream_schedule_file(self.bridge_ptr, file.ptr, &mut err)
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    pub const fn next(&self) -> NextItem<'_, PlayerNodeCompletionEvent> {
        self.inner.next()
    }

    pub fn try_next(&self) -> Option<PlayerNodeCompletionEvent> {
        self.inner.try_next()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

pub struct RecorderEventStream {
    inner: BoundedAsyncStream<RecorderEvent>,
    bridge_ptr: *mut c_void,
    sender_raw: *mut AsyncStreamSender<RecorderEvent>,
}

// SAFETY: Same rationale as `ConfigChangeStream`. The recorder handle is
// thread-safe per Apple documentation and `sender_raw` is single-owner.
unsafe impl Send for RecorderEventStream {}

impl Drop for RecorderEventStream {
    fn drop(&mut self) {
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_recorder_stream_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
        drop_sender(&mut self.sender_raw);
    }
}

unsafe extern "C" fn recorder_event_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    catch_user_panic("recorder_event_cb", || {
        let Some(sender) = ctx.cast::<AsyncStreamSender<RecorderEvent>>().as_ref() else {
            return;
        };
        let event = match kind {
            0 => RecorderEvent::DidFinishRecording {
                successfully: false,
            },
            1 => RecorderEvent::DidFinishRecording { successfully: true },
            2 => {
                let message = if payload.is_null() {
                    String::new()
                } else {
                    // SAFETY: `payload` is a non-null, NUL-terminated C string
                    // allocated by the Swift bridge; its lifetime covers this
                    // callback invocation.
                    unsafe { CStr::from_ptr(payload.cast::<c_char>()) }
                        .to_string_lossy()
                        .into_owned()
                };
                RecorderEvent::EncodeError { message }
            }
            _ => return,
        };
        sender.push(event);
    });
}

impl RecorderEventStream {
    pub fn subscribe(recorder: &AudioRecorder, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_raw = Box::into_raw(Box::new(sender));
        let bridge_ptr = unsafe {
            ffi::ava_recorder_stream_subscribe(
                recorder.ptr(),
                recorder_event_cb,
                sender_raw.cast::<c_void>(),
            )
        };
        Self {
            inner: stream,
            bridge_ptr,
            sender_raw,
        }
    }

    pub const fn next(&self) -> NextItem<'_, RecorderEvent> {
        self.inner.next()
    }

    pub fn try_next(&self) -> Option<RecorderEvent> {
        self.inner.try_next()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

pub struct SimplePlayerEventStream {
    inner: BoundedAsyncStream<SimplePlayerEvent>,
    bridge_ptr: *mut c_void,
    sender_raw: *mut AsyncStreamSender<SimplePlayerEvent>,
}

// SAFETY: Same rationale as `ConfigChangeStream`. The player handle is
// thread-safe per Apple documentation and `sender_raw` is single-owner.
unsafe impl Send for SimplePlayerEventStream {}

impl Drop for SimplePlayerEventStream {
    fn drop(&mut self) {
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_simple_player_stream_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
        drop_sender(&mut self.sender_raw);
    }
}

unsafe extern "C" fn simple_player_event_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    catch_user_panic("simple_player_event_cb", || {
        let Some(sender) = ctx.cast::<AsyncStreamSender<SimplePlayerEvent>>().as_ref() else {
            return;
        };
        let event = match kind {
            0 => SimplePlayerEvent::DidFinishPlaying {
                successfully: false,
            },
            1 => SimplePlayerEvent::DidFinishPlaying { successfully: true },
            2 => {
                let message = if payload.is_null() {
                    String::new()
                } else {
                    // SAFETY: `payload` is a non-null, NUL-terminated C string
                    // allocated by the Swift bridge; its lifetime covers this
                    // callback invocation.
                    unsafe { CStr::from_ptr(payload.cast::<c_char>()) }
                        .to_string_lossy()
                        .into_owned()
                };
                SimplePlayerEvent::DecodeError { message }
            }
            _ => return,
        };
        sender.push(event);
    });
}

impl SimplePlayerEventStream {
    pub fn subscribe(player: &AudioSimplePlayer, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_raw = Box::into_raw(Box::new(sender));
        let bridge_ptr = unsafe {
            ffi::ava_simple_player_stream_subscribe(
                player.ptr(),
                simple_player_event_cb,
                sender_raw.cast::<c_void>(),
            )
        };
        Self {
            inner: stream,
            bridge_ptr,
            sender_raw,
        }
    }

    pub const fn next(&self) -> NextItem<'_, SimplePlayerEvent> {
        self.inner.next()
    }

    pub fn try_next(&self) -> Option<SimplePlayerEvent> {
        self.inner.try_next()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

/// Async stream of [`TapBufferEvent`]s produced by an `AVAudioNode.installTap`
/// tap installed via [`TapBufferStream::subscribe_to_node`].
///
/// # Real-time thread safety — important caveat
///
/// Apple's `AVAudioNode.installTap(onBus:bufferSize:format:block:)` fires
/// its callback on the **`CoreAudio` high-priority I/O render thread**.  The
/// internal implementation calls [`AsyncStreamSender::push`], which acquires
/// a [`std::sync::Mutex`] on every invocation.  Under normal conditions
/// (capacity is not exhausted, consumer drains promptly) the lock is
/// uncontested and the critical section is extremely short (a single ring-buffer
/// push).  However, callers should be aware of the following:
///
/// * **Keep capacity generous.**  A capacity of ≥ 32 tap events reduces the
///   probability of the audio thread encountering a contended mutex cycle.
/// * **Drain promptly.**  Call [`Self::try_next`] or poll [`Self::next`] from
///   a low-latency async task so the consumer does not hold the lock when the
///   render thread fires.
/// * **Never call blocking operations** (I/O, `std::thread::sleep`, heavy
///   allocation, or any other lock) from a task that calls [`Self::try_next`]
///   inside a tight audio-sync loop.
///
/// > **Known limitation:** the current implementation uses a
/// > `std::sync::Mutex`-backed ring buffer rather than a lock-free SPSC queue.
/// > A future version of this crate will replace it to fully eliminate priority-
/// > inversion risk on the render thread.  Track progress in the crate
/// > repository.
pub struct TapBufferStream {
    inner: BoundedAsyncStream<TapBufferEvent>,
    bridge_ptr: *mut c_void,
    sender_raw: *mut AsyncStreamSender<TapBufferEvent>,
}

// SAFETY: `bridge_ptr` is an AVFoundation opaque tap handle whose install/
// remove APIs are thread-safe per Apple documentation.  `sender_raw` is a
// heap-allocated `Box` that is only accessed from a single thread at a time.
// `BoundedAsyncStream` is itself `Send`.
unsafe impl Send for TapBufferStream {}

impl Drop for TapBufferStream {
    fn drop(&mut self) {
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_node_tap_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
        drop_sender(&mut self.sender_raw);
    }
}

// Called on Apple's CoreAudio high-priority I/O render thread.
//
// SAFETY contract for this function:
//   • `payload`, when non-null, points to a `TapEventPayloadRaw` struct
//     laid out exactly as defined in the Swift bridge (matching `#[repr(C)]`
//     on the Rust side); the Swift bridge guarantees its validity for the
//     duration of this call.
//   • `ctx` is either null or points to a live `AsyncStreamSender<TapBufferEvent>`
//     produced by `Box::into_raw` in `subscribe_to_node`; it remains valid
//     until `drop_sender` is called from `TapBufferStream::drop`, which only
//     happens after `ava_node_tap_unsubscribe` has returned and the render
//     thread can no longer fire this callback.
//
// ⚠ Real-time note: `sender.push()` acquires a `std::sync::Mutex`.
// See the `TapBufferStream` struct-level docs for the implications.
unsafe extern "C" fn tap_event_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    catch_user_panic("tap_event_cb", || {
        if kind != 0 || payload.is_null() {
            return;
        }
        let Some(sender) = ctx.cast::<AsyncStreamSender<TapBufferEvent>>().as_ref() else {
            return;
        };
        // SAFETY: `payload` is non-null (checked above) and points to a
        // valid `TapEventPayloadRaw` as guaranteed by the Swift bridge
        // contract described in the function-level SAFETY comment.
        let raw = unsafe { &*payload.cast::<TapEventPayloadRaw>() };
        sender.push(TapBufferEvent {
            frame_length: raw.frame_length,
            channel_count: raw.channel_count,
            sample_rate: raw.sample_rate,
        });
    });
}

impl TapBufferStream {
    /// Install a tap on `bus` of `node` and return a stream of buffer
    /// snapshot events.
    ///
    /// Each event contains the frame length, channel count, and sample rate
    /// of the buffer delivered by `CoreAudio`.  When the ring buffer is full,
    /// the **oldest** event is silently dropped (lossy-by-design).
    ///
    /// # Real-time thread safety
    ///
    /// The tap callback fires on Apple's `CoreAudio` high-priority I/O render
    /// thread.  Internally it acquires a `std::sync::Mutex`; see the
    /// [`TapBufferStream`] struct-level documentation for implications and
    /// guidance on choosing an appropriate `capacity`.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0 (see [`BoundedAsyncStream::new`]).
    pub fn subscribe_to_node(
        node: &dyn AudioNodeHandle,
        bus: usize,
        buffer_size: u32,
        format: Option<&AudioFormat>,
        capacity: usize,
    ) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_raw = Box::into_raw(Box::new(sender));
        let bridge_ptr = unsafe {
            ffi::ava_node_tap_subscribe(
                node.as_node_ptr(),
                bus,
                buffer_size,
                format.map_or(ptr::null_mut(), |format| format.ptr),
                tap_event_cb,
                sender_raw.cast::<c_void>(),
            )
        };
        Self {
            inner: stream,
            bridge_ptr,
            sender_raw,
        }
    }

    pub const fn next(&self) -> NextItem<'_, TapBufferEvent> {
        self.inner.next()
    }

    pub fn try_next(&self) -> Option<TapBufferEvent> {
        self.inner.try_next()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}
