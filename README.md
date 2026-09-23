# avaudio-rs

Safe Rust bindings for Apple `AVFoundation` audio APIs on macOS 12 or later.

## Installation

```toml
[dependencies]
avaudio = "0.6"
```

Enable the `async` feature for the future and stream wrappers:

```toml
[dependencies]
avaudio = { version = "0.6", features = ["async"] }
```

## Features

`avaudio` now covers the core pieces needed to build and inspect `AVFoundation` audio graphs from Rust:

- `AVAudioEngine` graph creation, preparation, start/stop/reset, and generic node attach/connect helpers.
- `AVAudioPlayerNode`, `AVAudioMixerNode`, `AVAudioInputNode`, `AVAudioOutputNode`, `AVAudioEnvironmentNode`, `AVAudioSourceNode`, and `AVAudioSinkNode` wrappers.
- `AVAudioPCMBuffer` sample access: `PCMBuffer::channel_data::<T>()` and `channel_data_mut::<T>()` for `f32`, `i16`, and `i32` samples (deinterleaved or interleaved), plus owned copies through `copy_samples()`. Buffers scheduled on a player node reject writes until the player releases them.
- `AVAudioFile`, `AVAudioPCMBuffer`, `AVAudioCompressedBuffer`, `AVAudioBuffer`, `AVAudioFormat`, `AVAudioChannelLayout`, `AVAudioConnectionPoint`, `AVAudioTime`, `AVAudioConverter`, and `AVAudioSequencer` support, including converter prime/status helpers and sequencer data/file round-tripping plus `AVMusicTrack` event editing helpers.
- Generic `AVAudioUnit`, `AVAudioUnitEffect`, `AVAudioUnitTimeEffect`, `AVAudioUnitGenerator`, `AVAudioUnitMIDIInstrument`, `AVAudioUnitTimePitch`, `AVAudioUnitReverb`, `AVAudioUnitEQ`, `AVAudioUnitDelay`, `AVAudioUnitDistortion`, `AVAudioUnitSampler`, `AVAudioUnitVarispeed`, and shared audio-unit bypass/metadata helpers.
- Public Rust mirrors plus protocol traits for core `AVAudioTypes.h`, `AVAudioMixing.h`, `AVAudioSettings.h`, `AVAudioSessionTypes.h`, and the macOS-visible `AVAudioIONode` / `AVAudioSessionRoute` helper types, including `AudioMixingDestination`, routing arbitration, session capability, manual-rendering input blocks, and voice-processing ducking/speech-activity helpers.
- `AVAudioPlayer` (`AudioSimplePlayer`) and `AVAudioRecorder` (`AudioRecorder`) convenience playback/capture APIs, including delegate-bridge helpers.
- `AVAudioApplication` permission/input-mute queries and `AVAudioUnitComponentManager` discovery snapshots/constants.
- `AVAudioSession`-style session queries with a macOS-friendly compatibility stub.
- Optional Rust callbacks for `AVAudioPlayerNode`, `AVAudioSourceNode`, `AVAudioSinkNode`, `AVAudioSequencer`, `AVAudioPlayerDelegate`, and `AVAudioRecorderDelegate` blocks/callbacks.
- Optional `async` feature exposing executor-agnostic future/stream wrappers for record permission, muted-speech activity, engine configuration changes, player-node completions, recorder/player delegates, and `AVAudioNode.installTap` buffers with copied samples.
- Graph and playback calls that `AVFoundation` guards with Objective-C exceptions (attaching a node owned by another engine, connecting detached nodes, starting an empty engine, playing a detached player, scheduling a buffer with the wrong channel count, installing a second tap on a bus) return `AVAudioError` instead of aborting the process.

See [COVERAGE.md](COVERAGE.md) for the API coverage table.

## Async futures and streams

Enable the `async` feature to use `avaudio::async_api` and executor-agnostic wrappers around `AVFAudio`'s callback surfaces. `AsyncAudioApplication::request_record_permission` exposes the one-shot microphone permission callback as a `Future`; `MutedSpeechActivityStream`, `ConfigChangeStream`, `PlayerNodeCompletionStream`, `RecorderEventStream`, `SimplePlayerEventStream`, and `TapBufferStream` expose event/listener surfaces as bounded async streams.

`TapBufferStream` is special-cased to use a lossy `doom-fish-utils::spsc::SpscRing`; each event carries a copy of the tap buffer's samples. `AVFoundation` calls tap blocks on an internal, non-real-time thread. `TapBufferStream::subscribe_to_node` fails if the bus already has a tap instead of replacing it. Every other stream uses `doom-fish-utils::stream::BoundedAsyncStream`. As with the underlying Apple API, only one muted-speech activity listener should be active per input node at a time.

```bash
cargo run --features async --example 26_async_config_change
cargo run --features async --example 27_async_player_completion
```

## Example

```rust,no_run
use avaudio::prelude::*;

fn main() -> Result<(), AVAudioError> {
    let file = AudioFile::open_for_reading("speech.aiff")?;
    let format = file.processing_format()?;
    let buffer = file.read_pcm_buffer(2048)?;

    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    let environment = AudioEnvironmentNode::new()?;

    engine.attach_node(&player)?;
    engine.attach_node(&environment)?;
    engine.connect_nodes(&player, &environment, Some(&format))?;
    engine.connect_node_to_main_mixer(&environment, Some(&format))?;
    engine.prepare()?;
    engine.start()?;

    player.schedule_buffer(&buffer)?;
    player.play()?;
    Ok(())
}
```

## Examples

The crate ships with a numbered example set:

- `01_smoke_surface`
- `02_mixer_node`
- `03_input_output_nodes`
- `04_environment_node`
- `05_unit_time_pitch`
- `06_unit_reverb`
- `07_unit_eq`
- `08_converter`
- `09_simple_player`
- `10_recorder`
- `11_session`
- `12_environment_node_chain`
- `13_format`
- `14_player_node`
- `15_audio_buffer`
- `16_unit_effect`
- `17_pcm_buffer`
- `18_input_node`
- `19_output_node`
- `20_audio_file`
- `21_audio_application`
- `22_source_sink_nodes`
- `23_sequencer`
- `24_unit_component`
- `25_unit_variants`
- `26_async_config_change` *(requires `--features async`)*
- `27_async_player_completion` *(requires `--features async`)*

Examples that require playback or capture hardware print a skip message and still exit successfully on headless hosts.

## Smoke test

```bash
cargo run --example 01_smoke_surface
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
