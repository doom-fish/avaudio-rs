mod common;

use avaudio::prelude::*;

#[test]
fn pcm_buffer_new_and_set_length() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 1, false)?;
    let mut buffer = PCMBuffer::new(&format, 512)?;
    buffer.set_frame_length(256)?;

    assert_eq!(buffer.frame_capacity()?, 512);
    assert_eq!(buffer.frame_length()?, 256);
    assert!((buffer.format()?.sample_rate()? - 48_000.0).abs() < f64::EPSILON);
    Ok(())
}
