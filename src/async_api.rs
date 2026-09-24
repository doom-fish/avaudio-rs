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
//! and delivers a copy of every tap buffer's samples; every other stream uses
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

use doom_fish_utils::callback_context::CallbackContext;
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
use crate::pcm_buffer::{PCMBuffer, PCMSamples};
use crate::player::{
    AudioPlayerNode, AudioPlayerNodeBufferOptions, AudioPlayerNodeCompletionCallbackType,
};
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

#[derive(Debug, Clone, PartialEq)]
pub struct TapBufferEvent {
    pub frame_length: u32,
    pub channel_count: u32,
    pub sample_rate: f64,
    pub samples: Option<PCMSamples>,
}

impl TapBufferEvent {
    fn capture(buffer: &PCMBuffer) -> Self {
        let layout = buffer.layout();
        Self {
            frame_length: layout.frame_length,
            channel_count: layout.channel_count,
            sample_rate: layout.sample_rate,
            samples: buffer.copy_samples(),
        }
    }
}

const TAP_BUFFER_STREAM_MAX_CAPACITY: usize = 4096;

type TapBufferProducer = SpscProducer<TapBufferEvent, TAP_BUFFER_STREAM_MAX_CAPACITY>;
type TapBufferConsumer = SpscConsumer<TapBufferEvent, TAP_BUFFER_STREAM_MAX_CAPACITY>;
type TapBufferNext<'a> = SpscPopFuture<'a, TapBufferEvent, TAP_BUFFER_STREAM_MAX_CAPACITY>;
type TapBufferContext = CallbackContext<TapBufferProducer>;
type ConfigChangeContext = CallbackContext<AsyncStreamSender<ConfigChangeEvent>>;
type PlayerCompletionContext = CallbackContext<AsyncStreamSender<PlayerNodeCompletionEvent>>;
type RecorderEventContext = CallbackContext<AsyncStreamSender<RecorderEvent>>;
type SimplePlayerEventContext = CallbackContext<AsyncStreamSender<SimplePlayerEvent>>;

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
                &raw mut err,
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
    context: ConfigChangeContext,
    bridge_ptr: *mut c_void,
}

// SAFETY: `bridge_ptr` is an AVFoundation opaque handle whose
// subscription/unsubscribe APIs are thread-safe per Apple documentation.
// The sender lives in a reference-counted `CallbackContext`, and
// `BoundedAsyncStream` is itself `Send`.
unsafe impl Send for ConfigChangeStream {}

impl Drop for ConfigChangeStream {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_engine_config_change_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
    }
}

unsafe extern "C" fn config_change_cb(_kind: i32, _payload: *const c_void, ctx: *mut c_void) {
    unsafe {
        ConfigChangeContext::with(ctx, "config_change_cb", |sender| {
            sender.push(ConfigChangeEvent);
        })
    };
}

impl ConfigChangeStream {
    pub fn subscribe(engine: &AudioEngine, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let context = CallbackContext::new(sender);
        let bridge_ptr = unsafe {
            ffi::ava_engine_config_change_subscribe(
                engine.as_engine_ptr(),
                config_change_cb,
                context.retained_ptr(),
                ConfigChangeContext::RELEASE,
            )
        };
        Self {
            inner: stream,
            context,
            bridge_ptr,
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
                &raw mut err,
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
    context: PlayerCompletionContext,
    bridge_ptr: *mut c_void,
}

// SAFETY: Same rationale as `ConfigChangeStream`. The player node handle
// is thread-safe per Apple documentation.
unsafe impl Send for PlayerNodeCompletionStream {}

impl Drop for PlayerNodeCompletionStream {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_player_node_stream_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
    }
}

unsafe extern "C" fn player_completion_cb(kind: i32, _payload: *const c_void, ctx: *mut c_void) {
    unsafe {
        PlayerCompletionContext::with(ctx, "player_completion_cb", |sender| {
            sender.push(PlayerNodeCompletionEvent::from_kind(kind));
        })
    };
}

impl PlayerNodeCompletionStream {
    pub fn subscribe(player: &AudioPlayerNode, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let context = CallbackContext::new(sender);
        let bridge_ptr = unsafe {
            ffi::ava_player_node_stream_subscribe(
                player.ptr,
                player_completion_cb,
                context.retained_ptr(),
                PlayerCompletionContext::RELEASE,
            )
        };
        Self {
            inner: stream,
            context,
            bridge_ptr,
        }
    }

    pub fn schedule_buffer(
        &self,
        buffer: &PCMBuffer,
        options: AudioPlayerNodeBufferOptions,
        callback_type: AudioPlayerNodeCompletionCallbackType,
    ) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::ava_player_node_stream_schedule_buffer(
                self.bridge_ptr,
                buffer.ptr,
                options.bits(),
                callback_type.as_raw(),
                &raw mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    pub fn schedule_file(
        &self,
        file: &AudioFile,
        callback_type: AudioPlayerNodeCompletionCallbackType,
    ) -> Result<(), AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::ava_player_node_stream_schedule_file(
                self.bridge_ptr,
                file.ptr,
                callback_type.as_raw(),
                &raw mut err,
            )
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
    context: RecorderEventContext,
    bridge_ptr: *mut c_void,
}

// SAFETY: Same rationale as `ConfigChangeStream`. The recorder handle is
// thread-safe per Apple documentation.
unsafe impl Send for RecorderEventStream {}

impl Drop for RecorderEventStream {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_recorder_stream_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
    }
}

unsafe extern "C" fn recorder_event_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    let deliver = |sender: &AsyncStreamSender<RecorderEvent>| {
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
    };
    unsafe { RecorderEventContext::with(ctx, "recorder_event_cb", deliver) };
}

impl RecorderEventStream {
    pub fn subscribe(recorder: &AudioRecorder, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let context = CallbackContext::new(sender);
        let bridge_ptr = unsafe {
            ffi::ava_recorder_stream_subscribe(
                recorder.ptr(),
                recorder_event_cb,
                context.retained_ptr(),
                RecorderEventContext::RELEASE,
            )
        };
        Self {
            inner: stream,
            context,
            bridge_ptr,
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
    context: SimplePlayerEventContext,
    bridge_ptr: *mut c_void,
}

// SAFETY: Same rationale as `ConfigChangeStream`. The player handle is
// thread-safe per Apple documentation.
unsafe impl Send for SimplePlayerEventStream {}

impl Drop for SimplePlayerEventStream {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_simple_player_stream_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
    }
}

unsafe extern "C" fn simple_player_event_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    let deliver = |sender: &AsyncStreamSender<SimplePlayerEvent>| {
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
    };
    unsafe { SimplePlayerEventContext::with(ctx, "simple_player_event_cb", deliver) };
}

impl SimplePlayerEventStream {
    pub fn subscribe(player: &AudioSimplePlayer, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let context = CallbackContext::new(sender);
        let bridge_ptr = unsafe {
            ffi::ava_simple_player_stream_subscribe(
                player.ptr(),
                simple_player_event_cb,
                context.retained_ptr(),
                SimplePlayerEventContext::RELEASE,
            )
        };
        Self {
            inner: stream,
            context,
            bridge_ptr,
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
/// # Delivery
///
/// Apple's `AVAudioNode.installTap(onBus:bufferSize:format:block:)` calls its
/// block on an internal, non-real-time thread. The tap callback copies the
/// buffer's samples and hands each event off through a lock-free single-producer /
/// single-consumer ring.
///
/// The ring is intentionally **lossy**: if the consumer falls behind, the
/// oldest buffered tap event is overwritten so the tap thread can keep
/// running without waiting. Drain the stream promptly if every tap snapshot is
/// important to your application.
///
/// Requested capacities above `TAP_BUFFER_STREAM_MAX_CAPACITY` are clamped to
/// that fixed pre-allocated maximum.
pub struct TapBufferStream {
    inner: TapBufferConsumer,
    context: TapBufferContext,
    bridge_ptr: *mut c_void,
}

// SAFETY: `bridge_ptr` is an AVFoundation opaque tap handle whose install/
// remove APIs are thread-safe per Apple documentation. The producer lives in a
// reference-counted `CallbackContext`, and `SpscConsumer` is itself `Send`.
unsafe impl Send for TapBufferStream {}

impl Drop for TapBufferStream {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.bridge_ptr.is_null() {
            unsafe { ffi::ava_node_tap_unsubscribe(self.bridge_ptr) };
            self.bridge_ptr = ptr::null_mut();
        }
    }
}

// Called on the tap delivery thread.
//
// SAFETY contract for this function:
//   • `payload`, when non-null, is a +1 retained `AVAudioPCMBuffer`; this
//     function takes ownership of that reference.
//   • `ctx` is the `TapBufferContext` reference retained by the Swift bridge,
//     which releases it only from its deinit.
unsafe extern "C" fn tap_event_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    if payload.is_null() {
        return;
    }
    let buffer = PCMBuffer {
        ptr: payload.cast_mut(),
    };
    if kind != 0 {
        return;
    }
    unsafe {
        TapBufferContext::with(ctx, "tap_event_cb", |producer| {
            let _ = producer.push_overwrite(TapBufferEvent::capture(&buffer));
        })
    };
}

impl TapBufferStream {
    /// Install a tap on `bus` of `node` and return a stream of buffer
    /// snapshot events.
    ///
    /// Each event contains the frame length, channel count, sample rate, and a
    /// copy of the samples of the buffer delivered by the tap. When the
    /// requested capacity is exceeded, the **oldest** buffered event is
    /// overwritten so the tap thread never waits for the async consumer.
    ///
    /// `capacity` must be greater than 0; values above
    /// `TAP_BUFFER_STREAM_MAX_CAPACITY` are clamped to that fixed pre-allocated
    /// maximum.
    pub fn subscribe_to_node(
        node: &dyn AudioNodeHandle,
        bus: usize,
        buffer_size: u32,
        format: Option<&AudioFormat>,
        capacity: usize,
    ) -> Result<Self, AVAudioError> {
        if capacity == 0 {
            return Err(AVAudioError::InvalidArgument(
                "TapBufferStream capacity must be greater than 0".into(),
            ));
        }
        let bus = u32::try_from(bus)
            .map_err(|_| AVAudioError::InvalidArgument("bus index exceeds UInt32 range".into()))?;
        let ring_capacity = capacity.min(TAP_BUFFER_STREAM_MAX_CAPACITY);
        let (producer, consumer) =
            SpscRing::<TapBufferEvent, TAP_BUFFER_STREAM_MAX_CAPACITY>::with_capacity(
                ring_capacity,
            );
        let context = CallbackContext::new(producer);
        let mut err: *mut c_char = ptr::null_mut();
        let bridge_ptr = unsafe {
            ffi::ava_node_tap_subscribe(
                node.as_node_ptr(),
                bus,
                buffer_size,
                format.map_or(ptr::null_mut(), |format| format.ptr),
                tap_event_cb,
                context.retained_ptr(),
                TapBufferContext::RELEASE,
                &raw mut err,
            )
        };
        if bridge_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::CALLBACK_ERROR, err) });
        }
        Ok(Self {
            inner: consumer,
            context,
            bridge_ptr,
        })
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
