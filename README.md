# avaudio-rs

Safe Rust bindings for Apple `AVFoundation` audio APIs on macOS.

## Features

`avaudio` now covers the core pieces needed to build and inspect `AVFoundation` audio graphs from Rust:

- `AVAudioEngine` graph creation, preparation, start/stop/reset, and generic node attach/connect helpers.
- `AVAudioPlayerNode`, `AVAudioMixerNode`, `AVAudioInputNode`, `AVAudioOutputNode`, `AVAudioEnvironmentNode`, `AVAudioSourceNode`, and `AVAudioSinkNode` wrappers.
- `AVAudioFile`, `AVAudioPCMBuffer`, `AVAudioCompressedBuffer`, `AVAudioBuffer`, `AVAudioFormat`, `AVAudioChannelLayout`, `AVAudioConnectionPoint`, `AVAudioTime`, `AVAudioConverter`, and `AVAudioSequencer` support, including converter prime/status helpers and sequencer data/file round-tripping plus `AVMusicTrack` event editing helpers.
- Generic `AVAudioUnit`, `AVAudioUnitEffect`, `AVAudioUnitTimeEffect`, `AVAudioUnitGenerator`, `AVAudioUnitMIDIInstrument`, `AVAudioUnitTimePitch`, `AVAudioUnitReverb`, `AVAudioUnitEQ`, `AVAudioUnitDelay`, `AVAudioUnitDistortion`, `AVAudioUnitSampler`, `AVAudioUnitVarispeed`, and shared audio-unit bypass/metadata helpers.
- Public Rust mirrors plus protocol traits for core `AVAudioTypes.h`, `AVAudioMixing.h`, `AVAudioSettings.h`, `AVAudioSessionTypes.h`, and the macOS-visible `AVAudioIONode` / `AVAudioSessionRoute` helper types, including `AudioMixingDestination`, routing arbitration, session capability, manual-rendering input blocks, and voice-processing ducking/speech-activity helpers.
- `AVAudioPlayer` (`AudioSimplePlayer`) and `AVAudioRecorder` (`AudioRecorder`) convenience playback/capture APIs, including delegate-bridge helpers.
- `AVAudioApplication` permission/input-mute queries and `AVAudioUnitComponentManager` discovery snapshots/constants.
- `AVAudioSession`-style session queries with a macOS-friendly compatibility stub.
- Optional Rust callbacks for `AVAudioPlayerNode`, `AVAudioSourceNode`, `AVAudioSinkNode`, `AVAudioSequencer`, `AVAudioPlayerDelegate`, and `AVAudioRecorderDelegate` blocks/callbacks.

See [COVERAGE.md](COVERAGE.md) for the API coverage table.

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

    engine.attach_node(&player);
    engine.attach_node(&environment);
    engine.connect_nodes(&player, &environment, Some(&format));
    engine.connect_node_to_main_mixer(&environment, Some(&format));
    engine.prepare();
    engine.start()?;

    player.schedule_buffer(&buffer)?;
    player.play();
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

Examples that require playback or capture hardware print a skip message and still exit successfully on headless hosts.

## Smoke test

```bash
cargo run --example 01_smoke_surface
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
