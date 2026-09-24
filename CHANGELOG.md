# Changelog

All notable changes to `avaudio` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2026-09-24

### Security

- `AudioManualRenderingInput::from_buffer` kept only a raw pointer to the caller's buffer, so a safe callback could hand the engine a buffer that had already been freed. The input now owns the buffer, and the input block keeps it alive until the engine calls the block again.
- Dropping an `AudioPlayerNode` while the engine still held the node freed its pending completion state, and a later completion then ran on freed memory. The completion handler the player holds now owns that state, fires it at most once, and frees it when the player releases the handler.
- A rejected `set_manual_rendering_input_pcm_format_with_callback` or `set_muted_speech_activity_event_listener` call freed the callback state twice, crashing from safe code. Swift now owns the state on every path.
- The async streams freed their sender right after unsubscribing, while a notification, delegate, completion or tap callback on another thread could still use it. The senders now live in a reference-counted `doom_fish_utils::callback_context::CallbackContext` that each Swift bridge releases from its deinit.
- Safe code could write samples of a buffer while a player node was reading it. Buffers scheduled on a player node now reject writes until the player releases them.
- Code outside the crate could implement `AudioNodeHandle`, `AudioBufferHandle`, `AudioUnitHandle` or `AudioUnitMIDIInstrumentHandle` and return any pointer from their hidden accessor, which the bridge then dereferences. These traits are now sealed.

### Fixed

- Graph and playback misuse that AVFoundation reports with Objective-C exceptions no longer aborts the process: connecting nodes that are not attached to the engine, attaching a node owned by another engine, preparing or starting an engine without input or output nodes, playing a detached player, scheduling a buffer whose channel count differs from the player's output format, and installing a second tap on a bus now return `AVAudioError`.
- `TapBufferStream` no longer removes an existing tap on the bus, and install failures are reported. `AudioInputNode::install_tap_scaffold` no longer removes an existing tap either.
- A panicking `AudioSourceNode` render callback now zeroes the output buffers and sets the silence flag instead of leaving their previous contents.
- Callback-state destructors run inside a panic guard, so a panicking captured value no longer aborts the process from an `extern "C"` drop callback.
- Manual-rendering input buffers whose format differs from the registered format are no longer handed to the engine.
- Scheduling completions no longer mutate an unsynchronized dictionary from the player's completion thread.
- The `TapBufferStream` docs said tap blocks run on the real-time render thread; AVFoundation calls them on an internal, normal-priority thread.
- `AudioSession::{sample_rate, output_volume, is_other_audio_playing}` returned the fixed values 48 000 Hz, 1.0 and `false`, because `AVAudioSession` is unavailable on macOS. They now return `AVAudioError::Unsupported`.
- Swift's `init(rawValue:)` accepts any value for an imported enum, so unknown `AudioConverterPrimeMethod`, `AudioEngineManualRenderingMode`, `AudioVoiceProcessingOtherAudioDuckingLevel`, `AudioPlayerNodeCompletionCallbackType` and `AudioRoutingArbitrationCategory` values (`Other(_)`), EQ band filter types and distance-attenuation models reached AVFAudio unchecked. AVFAudio stored an undefined prime method or manual-rendering mode, or ignored the EQ and attenuation settings while the call reported success. These calls now return `AVAudioError::InvalidArgument`.
- `PlayerNodeCompletionStream::{schedule_buffer, schedule_file}` always requested `.dataPlayedBack` completions, which never fire in manual rendering mode, so the stream received no events there. The caller now chooses the completion callback type.
- Subscribing a `RecorderEventStream` or `SimplePlayerEventStream` replaced the delegate installed with `set_delegate` and freed its callbacks, and dropping the stream left the recorder or player with no delegate. Each recorder and player now owns one delegate hub that forwards every event to the `set_delegate` callbacks and to every subscribed stream, and unsubscribing removes only that stream.
- `AudioInputNode::{input_format, output_format}` and `AudioOutputNode::output_format` passed a 32-bit bus index to a Swift `Int` parameter, which the C calling convention does not require to be extended. Bus indices now cross the FFI as `usize`/`UInt` wherever Rust passes a `usize`.
- `AudioEngine::main_mixer_output_format` with a bus the main mixer does not have raised an Objective-C `NSRangeException` and aborted the process, and the input and output node format getters returned a made-up format for such buses. They now return `FormatError` (main mixer) or `OperationFailed` (IO nodes). `AudioConnectionPoint::new` trapped for a bus above `isize::MAX` and now returns an error.
- `AudioCompressedBuffer::new` with a zero `maximum_packet_size` raised an Objective-C exception and aborted the process. A `packet_capacity * maximum_packet_size` above `u32::MAX` wrapped the buffer's byte capacity, and a size above `isize::MAX` produced a buffer with a 4 GB capacity and no storage. These now return `InvalidArgument`.
- Framework values the Swift bridge narrows for Rust (enum raw values, EQ band and buffer counts, the player loop count, the speech-activity and player-completion kinds, converter statuses) used trapping `Int32(_:)`/`UInt32(_:)` conversions and now clamp.

### Changed

- **Breaking:** `AudioEngine::{prepare, attach_node, attach_player_node, connect_nodes, connect_node_to_main_mixer, connect_player_node_to_main_mixer}` and `AudioPlayerNode::play` return `Result<(), AVAudioError>`.
- **Breaking:** `AudioManualRenderingInput::from_buffer` and its `From` impl take a `PCMBuffer` by value; the type is no longer `Copy`, `Clone`, `PartialEq` or `Eq`.
- **Breaking:** `TapBufferStream::subscribe_to_node` returns `Result` and rejects a zero capacity or a bus index above `u32::MAX` instead of panicking. `TapBufferEvent` has a new `samples` field and is no longer `Copy`.
- **Breaking:** `PCMBuffer::set_frame_length`, the output buffer of `AudioConverter::{convert_buffer, convert_buffer_status}`, and the target of `AudioEngine::{render_offline, manual_rendering_block_render}` return an error while the buffer is scheduled on a player node.
- **Breaking:** `AudioNodeHandle`, `AudioBufferHandle`, `AudioUnitHandle` and `AudioUnitMIDIInstrumentHandle` are sealed, so only this crate's types implement them. `AudioMixingHandle` and `AudioIONodeHandle`, which code outside the crate could not name, are sealed as well.
- **Breaking:** `AudioSession::{sample_rate, output_volume, is_other_audio_playing}` return `Result`.
- **Breaking:** `AudioConverter::set_prime_method` and `AudioEnvironmentNode::set_distance_attenuation` return `Result<(), AVAudioError>`.
- **Breaking:** `PlayerNodeCompletionStream::{schedule_buffer, schedule_file}` take an `AudioPlayerNodeCompletionCallbackType`.
- Depends on `doom-fish-utils` `>=0.4.1, <0.5`.
- `rust-version` is now 1.82 (was 1.76).

### Added

- `PCMBuffer::channel_data::<T>()` and `PCMBuffer::channel_data_mut::<T>()` for `f32`, `i16` and `i32` samples in deinterleaved or interleaved layout, `PCMBuffer::copy_samples()`, `PCMBuffer::is_scheduled()`, and the `PCMSample`, `PCMChannelData`, `PCMChannelDataMut` and `PCMSamples` types.
- `AudioFormat::with_common_format` for `Float32`, `Float64`, `Int16` and `Int32` PCM formats; `AudioFormat::standard` only creates `Float32` formats.
- `TapBufferEvent::samples`, a copy of each tap buffer's samples.
- `AVAudioError::Unsupported`.

## [0.5.1] - 2026-06-06

- Guarded the render and FFI callback trampolines against panics unwinding across the FFI boundary, and pinned the tap-event FFI struct layout.

## [0.5.0] - 2026-05-20

### Added

- `AsyncAudioApplication::request_record_permission` plus `RecordPermissionFuture` for the one-shot `AVAudioApplication` permission callback.
- `MutedSpeechActivityStream` for `AVAudioInputNode.setMutedSpeechActivityEventListener(_:)`, matching the existing bounded async-stream style.
- Refreshed async README/docs and added a subscribe/drop async test for muted-speech activity listeners.

### Notes

- Phase 32 completeness + async sweep.

## [0.4.2] - 2026-05-20

- Added in-`src/` unit tests across `format`, `converter`, `io_node`, `types`, and `error` (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.4.1] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.4.0] - 2026-05-19

- Replaced `TapBufferStream`'s render-thread handoff with a lock-free SPSC ring from `doom-fish-utils::spsc`, removing the render-thread `std::sync::Mutex` from `AVAudioNode.installTap` delivery.
- `TapBufferStream` now overwrites the oldest buffered tap event when the requested capacity is exhausted, preserving real-time safety under overload.
- Added a 5-second async tap-buffer stress test that simulates render callbacks at 48 kHz aggregate throughput and verifies the consumer drains without hanging.

## [0.3.3] - 2026-05-19

- Bump MSRV from 1.70 to 1.76 to match fleet baseline.

## 0.3.2 - 2026-05-18

- Re-exported the shared `SimpleCallback` and `DropCallback` aliases from `doom-fish-utils::ffi_callbacks` instead of duplicating those FFI typedefs locally.

## 0.3.1

- **async_api**: Added `catch_user_panic` wrapping to all five `extern "C"` callbacks
  (`config_change_cb`, `player_completion_cb`, `recorder_event_cb`,
  `simple_player_event_cb`, `tap_event_cb`) — an unhandled Rust panic across an
  FFI boundary is undefined behaviour.
- **async_api / TapBufferStream**: Documented the real-time thread safety caveat:
  the tap callback fires on Apple's CoreAudio high-priority I/O render thread and
  internally acquires a `std::sync::Mutex`; added guidance on capacity sizing and
  consumer drain discipline, plus a note that a lock-free SPSC replacement is
  planned.
- **async_api / TapBufferStream**: Added SAFETY comments on the `payload` pointer
  dereference in `tap_event_cb` and on the `Box::from_raw` call in `drop_sender`.
- **async_api**: Added rationale comments on all `unsafe impl Send` blocks.
- **async_api / recorder_event_cb, simple_player_event_cb**: Added SAFETY comment
  on the `CStr::from_ptr` calls for the error-message payload.
- **Cargo.toml**: Widened `doom-fish-utils` version constraint to `>=0.1, <0.3`
  to allow the next minor release without a lockstep bump.

## 0.3.0

- Added the `async` Cargo feature with a new `async_api` module built on `doom-fish-utils::stream::BoundedAsyncStream`.
- Added `ConfigChangeStream` for `AVAudioEngine.configurationChangeNotification` events.
- Added `PlayerNodeCompletionStream` with stream-level `schedule_buffer` / `schedule_file` helpers for `AVAudioPlayerNode` completion callbacks.
- Added `RecorderEventStream` for `AVAudioRecorderDelegate` `DidFinishRecording` and `EncodeError` events.
- Added `SimplePlayerEventStream` for `AVAudioPlayerDelegate` `DidFinishPlaying` and `DecodeError` events.
- Added `TapBufferStream` for lossy `AVAudioNode.installTap` buffer snapshots.
- Added async examples and integration tests for the new stream surfaces.

## 0.2.3

- Closed the remaining macOS SDK audit gaps: added `AVAudioChannelLayout`, `AVAudioCompressedBuffer`, `AVAudioConnectionPoint`, `AVAudioTime`, `AVAudioMixing` / `AVAudioStereoMixing` / `AVAudio3DMixing`, `AVAudioMixingDestination`, `AVAudioRoutingArbiter`, and `AVAudioSessionCapability` wrappers.
- Expanded `AVAudioEngine`, `AVAudioConverter`, `AVAudioPlayerNode`, `AVAudioPlayer`, `AVAudioRecorder`, and `AVAudioIONode` coverage with manual-rendering, prime/status, typed scheduling, delegate bridging, voice-processing, ducking, and speech-activity surfaces.
- Added gap-closure smoke tests plus refreshed coverage documentation to reflect 100% audited macOS symbol coverage (with the single iOS-only exemption retained).

## 0.2.2

- Added generic `AVAudioUnit`, `AVAudioUnitEffect`, `AVAudioUnitTimeEffect`, `AVAudioUnitGenerator`, and `AVAudioUnitMIDIInstrument` wrappers, plus richer audio-unit metadata/preset/component-description support.
- Expanded `AVAudioUnitSampler` with audio-file loading, `masterGain`, and the shared MIDI-instrument send APIs.
- Greatly expanded `AVAudioSequencer` with data/file round-tripping, info-dictionary key access, track creation/removal, tempo-track access, and `AVMusicTrack` event editing/enumeration helpers.
- Added public Rust mirrors for core `AVAudioTypes.h`, `AVAudioMixing.h`, `AVAudioSettings.h`, and `AVAudioSessionTypes.h` enums/typealiases/constants that were previously missing from the public surface.
- Added new sequencer/settings smoke tests, refreshed the sequencer example, and updated the unit-variants example to exercise audio-file sampler loading.

## 0.2.1

- Added `AVAudioApplication` record-permission and input-mute wrappers, with the iOS-only microphone-injection permission moved to the audit exemptions.
- Added `AVAudioSourceNode`, `AVAudioSinkNode`, and `AVAudioSequencer` wrappers, including Rust callback trampolines for render, receiver, and user-event blocks.
- Added `AVAudioUnitComponentManager` discovery snapshots and standard `AVAudioUnitComponent` type/manufacturer constants.
- Added wrappers for `AVAudioUnitDelay`, `AVAudioUnitDistortion`, `AVAudioUnitSampler`, and `AVAudioUnitVarispeed`.
- Expanded the example suite and integration tests, and refreshed the API coverage documentation.

## 0.2.0

- Added generic `AVAudioEngine` node attach/connect helpers and engine accessors for mixer/input/output nodes.
- Added wrappers for `AVAudioMixerNode`, `AVAudioInputNode`, `AVAudioOutputNode`, and `AVAudioEnvironmentNode`.
- Added wrappers for `AVAudioUnitTimePitch`, `AVAudioUnitReverb`, `AVAudioUnitEQ`, and shared audio-unit bypass helpers.
- Added `AVAudioConverter`, `AVAudioPlayer`, `AVAudioRecorder`, and `AVAudioSession` coverage.
- Extended `AVAudioPCMBuffer` with buffer allocation and frame-length setters, plus base `AVAudioBuffer` inspection helpers.
- Added integration tests, expanded examples, and API coverage documentation.

## 0.1.0

- Initial `AVAudioEngine` / `AVAudioPlayerNode` / `AVAudioFile` bindings.
- Read audio files into `AVAudioPCMBuffer` objects.
- Schedule buffer / file playback with optional Rust completion callbacks.
- Smoke example that exercises file loading, engine setup, short playback, and callback delivery.
