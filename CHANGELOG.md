# Changelog

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
