use avaudio::prelude::*;

#[test]
fn audio_buffer_info_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 1, false)?;
    let mut buffer = PCMBuffer::new(&format, 128)?;
    buffer.set_frame_length(64)?;

    let info = buffer.buffer_info()?;
    assert_eq!(info.format.channel_count, 1);
    assert_eq!(info.buffer_count as usize, info.bytes_per_buffer.len());
    assert_eq!(info.buffer_count as usize, info.channel_counts.len());
    Ok(())
}
