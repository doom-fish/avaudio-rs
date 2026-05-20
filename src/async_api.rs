//! Async futures and stream wrappers for `AVFAudio` permission, notification,
//! delegate, and tap surfaces.
//!
//! Enable with the `async` Cargo feature.
//!
//! ## Available types
//!
//! | Type | Apple API wrapped |
//! |------|-------------------|
//! | [`AsyncAudioApplication`] / [`RecordPermissionFuture`] | `AVAudioApplication.requestRecordPermission(completionHandler:)` |
//! | [`ConfigChangeStream`] | `AVAudioEngineConfigurationChangeNotification` |
//! | [`MutedSpeechActivityStream`] | `AVAudioInputNode.setMutedSpeechActivityEventListener(_:)` |
//! | [`PlayerNodeCompletionStream`] | `AVAudioPlayerNode` typed completion callbacks |
//! | [`RecorderEventStream`] | `AVAudioRecorderDelegate` finish / encode-error callbacks |
//! | [`SimplePlayerEventStream`] | `AVAudioPlayerDelegate` finish / decode-error callbacks |
//! | [`TapBufferStream`] | `AVAudioNode.installTap(onBus:bufferSize:format:block:)` |
//!
//! `TapBufferStream` is special-cased to use `doom-fish-utils::spsc::SpscRing`
//! on the `CoreAudio` render thread; every other stream uses
//! `doom-fish-utils::stream::BoundedAsyncStream`.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CStr;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use doom_fish_utils::completion::{AsyncCompletion, AsyncCompletionFuture};
use doom_fish_utils::panic_safe::catch_user_panic;
use doom_fish_utils::spsc::{PopFuture as SpscPopFuture, SpscConsumer, SpscProducer, SpscRing};
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};

use crate::audio_file::AudioFile;
use crate::engine::AudioEngine;
use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::format::AudioFormat;
use crate::input_node::AudioInputNode;
use crate::io_node::AudioVoiceProcessingSpeechActivityEvent;
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

const TAP_BUFFER_STREAM_MAX_CAPACITY: usize = 4096;

type TapBufferProducer = SpscProducer<TapBufferEvent, TAP_BUFFER_STREAM_MAX_CAPACITY>;
type TapBufferConsumer = SpscConsumer<TapBufferEvent, TAP_BUFFER_STREAM_MAX_CAPACITY>;
type TapBufferNext<'a> = SpscPopFuture<'a, TapBufferEvent, TAP_BUFFER_STREAM_MAX_CAPACITY>;

fn drop_boxed_ptr<T>(raw: &mut *mut T) {
    if !(*raw).is_null() {
        // SAFETY: `*raw` was produced by `Box::into_raw` inside the
        // corresponding `subscribe*` constructor and this path runs at most
        // once (the pointer is immediately zeroed below so it cannot be
        // reached a second time).
        unsafe { drop(Box::from_raw(*raw)) };
        *raw = ptr::null_mut();
    }
}

unsafe extern "C" fn record_permission_cb(userdata: *mut c_void, granted: bool) {
    catch_user_panic("record_permission_cb", || {
        // SAFETY: `userdata` is the `AsyncCompletion::create` context pointer
        // passed directly to `AVAudioApplication.requestRecordPermission`.
        unsafe {
            AsyncCompletion::<Result<bool, AVAudioError>>::complete_ok(userdata, Ok(granted));
        }
    });
}

/// Future returned by [`AsyncAudioApplication::request_record_permission`].
pub struct RecordPermissionFuture {
    inner: AsyncCompletionFuture<Result<bool, AVAudioError>>,
}

impl core::fmt::Debug for RecordPermissionFuture {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RecordPermissionFuture")
            .finish_non_exhaustive()
    }
}

impl Future for RecordPermissionFuture {
    type Output = Result<bool, AVAudioError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.inner).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(result)) => Poll::Ready(result),
            Poll::Ready(Err(message)) => Poll::Ready(Err(AVAudioError::OperationFailed(message))),
        }
    }
}

/// Async entry points for `AVAudioApplication`.
pub struct AsyncAudioApplication;

impl AsyncAudioApplication {
    /// Request microphone-record permission asynchronously.
    #[must_use]
    pub fn request_record_permission() -> RecordPermissionFuture {
        let (future, ctx) = AsyncCompletion::<Result<bool, AVAudioError>>::create();
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_application_request_record_permission(
                Some(record_permission_cb),
                ctx,
                None,
                &mut err,
            )
        };
        if status != ffi::status::OK {
            let error = unsafe { from_swift(status, err) };
            // SAFETY: `ctx` is the still-live `AsyncCompletion` context pointer
            // created above. No Swift callback will fire after a synchronous
            // registration failure.
            unsafe {
                AsyncCompletion::<Result<bool, AVAudioError>>::complete_ok(ctx, Err(error));
            }
        }
        RecordPermissionFuture { inner: future }
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
        drop_boxed_ptr(&mut self.sender_raw);
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

pub struct MutedSpeechActivityStream {
    inner: BoundedAsyncStream<AudioVoiceProcessingSpeechActivityEvent>,
    bridge_ptr: *mut c_void,
    sender_raw: *mut AsyncStreamSender<AudioVoiceProcessingSpeechActivityEvent>,
}

// SAFETY: `bridge_ptr` is an AVFoundation-owned opaque listener handle. The
// bridge serializes teardown before `sender_raw` is reclaimed, and
// `BoundedAsyncStream` is `Send`.
unsafe impl Send for MutedSpeechActivityStream {}

impl Drop for MutedSpeechActivityStream {
    fn drop(&mut self) {
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_input_node_speech_activity_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
        drop_boxed_ptr(&mut self.sender_raw);
    }
}

unsafe extern "C" fn muted_speech_activity_cb(
    kind: i32,
    _payload: *const c_void,
    ctx: *mut c_void,
) {
    catch_user_panic("muted_speech_activity_cb", || {
        let Some(sender) = ctx
            .cast::<AsyncStreamSender<AudioVoiceProcessingSpeechActivityEvent>>()
            .as_ref()
        else {
            return;
        };
        sender.push(AudioVoiceProcessingSpeechActivityEvent::from_raw(
            i64::from(kind),
        ));
    });
}

impl MutedSpeechActivityStream {
    /// Subscribe to muted-speech activity events for an input node.
    ///
    /// Only one muted-speech listener should be active per input node at a
    /// time. Avoid mixing this stream with the synchronous
    /// `set_muted_speech_activity_event_listener` API on the same node.
    pub fn subscribe(input: &AudioInputNode, capacity: usize) -> Result<Self, AVAudioError> {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let mut sender_raw = Box::into_raw(Box::new(sender));
        let mut err: *mut c_char = ptr::null_mut();
        let bridge_ptr = unsafe {
            ffi::ava_input_node_speech_activity_subscribe(
                input.ptr,
                muted_speech_activity_cb,
                sender_raw.cast::<c_void>(),
                &mut err,
            )
        };
        if bridge_ptr.is_null() {
            drop_boxed_ptr(&mut sender_raw);
            return Err(unsafe { from_swift(ffi::status::CALLBACK_ERROR, err) });
        }
        Ok(Self {
            inner: stream,
            bridge_ptr,
            sender_raw,
        })
    }

    pub const fn next(&self) -> NextItem<'_, AudioVoiceProcessingSpeechActivityEvent> {
        self.inner.next()
    }

    pub fn try_next(&self) -> Option<AudioVoiceProcessingSpeechActivityEvent> {
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
        drop_boxed_ptr(&mut self.sender_raw);
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
        drop_boxed_ptr(&mut self.sender_raw);
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
        drop_boxed_ptr(&mut self.sender_raw);
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
/// # Real-time safety
///
/// Apple's `AVAudioNode.installTap(onBus:bufferSize:format:block:)` fires its
/// callback on the **`CoreAudio` high-priority I/O render thread**. The tap
/// callback hands events off through a lock-free single-producer / single-
/// consumer ring, so the render thread does not take a mutex while publishing
/// tap snapshots to async Rust code.
///
/// The ring is intentionally **lossy**: if the consumer falls behind, the
/// oldest buffered tap event is overwritten so the render thread can keep
/// running without waiting. Drain the stream promptly if every tap snapshot is
/// important to your application.
///
/// Requested capacities above `TAP_BUFFER_STREAM_MAX_CAPACITY` are clamped to
/// that fixed pre-allocated maximum.
pub struct TapBufferStream {
    inner: TapBufferConsumer,
    bridge_ptr: *mut c_void,
    sender_raw: *mut TapBufferProducer,
}

// SAFETY: `bridge_ptr` is an AVFoundation opaque tap handle whose install/
// remove APIs are thread-safe per Apple documentation. `sender_raw` is a
// heap-allocated `Box` that is only touched from a single thread at a time.
// `SpscConsumer` is itself `Send`.
unsafe impl Send for TapBufferStream {}

impl Drop for TapBufferStream {
    fn drop(&mut self) {
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_node_tap_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
        drop_boxed_ptr(&mut self.sender_raw);
    }
}

// Called on Apple's `CoreAudio` high-priority I/O render thread.
//
// SAFETY contract for this function:
//   • `payload`, when non-null, points to a `TapEventPayloadRaw` struct
//     laid out exactly as defined in the Swift bridge (matching `#[repr(C)]`
//     on the Rust side); the Swift bridge guarantees its validity for the
//     duration of this call.
//   • `ctx` is either null or points to a live `TapBufferProducer` produced by
//     `Box::into_raw` in `subscribe_to_node`; it remains valid until
//     `drop_boxed_ptr` is called from `TapBufferStream::drop`, which only
//     happens after `ava_node_tap_unsubscribe` has returned and the render
//     thread can no longer fire this callback.
//
// The producer path is lock-free and never waits for the async consumer.
unsafe extern "C" fn tap_event_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    catch_user_panic("tap_event_cb", || {
        if kind != 0 || payload.is_null() {
            return;
        }
        let Some(sender) = ctx.cast::<TapBufferProducer>().as_ref() else {
            return;
        };
        // SAFETY: `payload` is non-null (checked above) and points to a
        // valid `TapEventPayloadRaw` as guaranteed by the Swift bridge
        // contract described in the function-level SAFETY comment.
        let raw = unsafe { &*payload.cast::<TapEventPayloadRaw>() };
        let _ = sender.push_overwrite(TapBufferEvent {
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
    /// of the buffer delivered by `CoreAudio`. When the requested capacity is
    /// exceeded, the **oldest** buffered event is overwritten so the render
    /// thread never waits for the async consumer.
    ///
    /// # Real-time safety
    ///
    /// The tap callback fires on Apple's `CoreAudio` high-priority I/O render
    /// thread and publishes into a lock-free SPSC ring. `capacity` must be
    /// greater than 0; values above `TAP_BUFFER_STREAM_MAX_CAPACITY` are
    /// clamped to that fixed pre-allocated maximum.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0.
    pub fn subscribe_to_node(
        node: &dyn AudioNodeHandle,
        bus: usize,
        buffer_size: u32,
        format: Option<&AudioFormat>,
        capacity: usize,
    ) -> Self {
        assert!(capacity > 0, "TapBufferStream capacity must be > 0");
        let ring_capacity = capacity.min(TAP_BUFFER_STREAM_MAX_CAPACITY);
        let (sender, stream) =
            SpscRing::<TapBufferEvent, TAP_BUFFER_STREAM_MAX_CAPACITY>::with_capacity(
                ring_capacity,
            );
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

    pub const fn next(&self) -> TapBufferNext<'_> {
        self.inner.pop_async()
    }

    pub fn try_next(&self) -> Option<TapBufferEvent> {
        self.inner.pop()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}
