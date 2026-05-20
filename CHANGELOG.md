# Changelog

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
