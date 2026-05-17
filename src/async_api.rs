//! Async stream wrappers for `AVFAudio` notifications, delegates, and taps.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CStr;

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
        unsafe { drop(Box::from_raw(*sender_raw)) };
        *sender_raw = ptr::null_mut();
    }
}

pub struct ConfigChangeStream {
    inner: BoundedAsyncStream<ConfigChangeEvent>,
    bridge_ptr: *mut c_void,
    sender_raw: *mut AsyncStreamSender<ConfigChangeEvent>,
}

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
    let Some(sender) = ctx.cast::<AsyncStreamSender<ConfigChangeEvent>>().as_ref() else {
        return;
    };
    sender.push(ConfigChangeEvent);
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
    let Some(sender) = ctx
        .cast::<AsyncStreamSender<PlayerNodeCompletionEvent>>()
        .as_ref()
    else {
        return;
    };
    sender.push(PlayerNodeCompletionEvent::from_kind(kind));
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
                CStr::from_ptr(payload.cast::<c_char>())
                    .to_string_lossy()
                    .into_owned()
            };
            RecorderEvent::EncodeError { message }
        }
        _ => return,
    };
    sender.push(event);
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
                CStr::from_ptr(payload.cast::<c_char>())
                    .to_string_lossy()
                    .into_owned()
            };
            SimplePlayerEvent::DecodeError { message }
        }
        _ => return,
    };
    sender.push(event);
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

pub struct TapBufferStream {
    inner: BoundedAsyncStream<TapBufferEvent>,
    bridge_ptr: *mut c_void,
    sender_raw: *mut AsyncStreamSender<TapBufferEvent>,
}

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

unsafe extern "C" fn tap_event_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    if kind != 0 || payload.is_null() {
        return;
    }
    let Some(sender) = ctx.cast::<AsyncStreamSender<TapBufferEvent>>().as_ref() else {
        return;
    };
    let raw = &*payload.cast::<TapEventPayloadRaw>();
    sender.push(TapBufferEvent {
        frame_length: raw.frame_length,
        channel_count: raw.channel_count,
        sample_rate: raw.sample_rate,
    });
}

impl TapBufferStream {
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
