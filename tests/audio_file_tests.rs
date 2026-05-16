mod common;

use avaudio::prelude::*;

#[test]
fn audio_file_open_and_read() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = common::artifacts_dir()?.join("audio-file-test.aiff");
    common::make_test_audio(&audio_path)?;

    let file = AudioFile::open_for_reading(&audio_path)?;
    let info = file.info()?;
    let processing = file.processing_format()?;
    let file_format = file.file_format()?;
    let buffer = file.read_pcm_buffer(1024)?;

    assert!(info.length_frames > 0);
    assert!(processing.info()?.sample_rate > 0.0);
    assert!(file_format.info()?.channel_count > 0);
    assert!(buffer.frame_capacity()? >= buffer.frame_length()?);
    Ok(())
}
