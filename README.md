# avaudio-rs

Safe Rust bindings for Apple's `AVAudioEngine`, `AVAudioPlayerNode`, `AVAudioFile`, `AVAudioFormat`, and `AVAudioPCMBuffer` on macOS.

## Features

- Read audio files into `AVAudioPCMBuffer`s.
- Build and drive an `AVAudioEngine` graph from Rust.
- Schedule `AVAudioFile` and `AVAudioPCMBuffer` playback on `AVAudioPlayerNode`.
- Receive one-shot playback-completion callbacks with a Rust closure.

## Example

```rust,no_run
use avaudio::prelude::*;

fn main() -> Result<(), AVAudioError> {
    let file = AudioFile::open_for_reading("speech.aiff")?;
    let format = file.processing_format()?;
    let buffer = file.read_pcm_buffer(2048)?;

    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    engine.attach_player_node(&player);
    engine.connect_player_node_to_main_mixer(&player, Some(&format));
    engine.prepare();
    engine.start()?;

    player.schedule_buffer(&buffer)?;
    player.play();
    Ok(())
}
```

## Smoke test

```bash
cargo run --all-features --example 01_smoke_surface
```

The smoke example only reads an AIFF file and plays a tiny buffer through the default output device. It does not access the microphone or request capture permissions.

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
