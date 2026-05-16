mod common;

use avaudio::prelude::*;

#[test]
fn file_open_and_read() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = common::artifacts_dir()?.join("file-test.aiff");
    common::make_test_audio(&audio_path)?;

    let file = AudioFile::open_for_reading(&audio_path)?;
    let info = file.info()?;
    let buffer = file.read_pcm_buffer(1024)?;

    assert!(info.length_frames > 0);
    assert!(buffer.frame_capacity()? >= buffer.frame_length()?);
    Ok(())
}
