mod common;

use avaudio::prelude::*;

#[test]
fn converter_buffer_conversion() -> Result<(), Box<dyn std::error::Error>> {
    let input_format = AudioFormat::standard(48_000.0, 1, false)?;
    let output_format = AudioFormat::standard(24_000.0, 1, false)?;
    let converter = AudioConverter::new(&input_format, &output_format)?;
    let mut input = PCMBuffer::new(&input_format, 480)?;
    input.set_frame_length(480)?;
    let mut output = PCMBuffer::new(&output_format, 512)?;

    converter.convert_buffer(&input, &mut output)?;

    let info = converter.info()?;
    assert!((info.input_format.sample_rate - 48_000.0).abs() < f64::EPSILON);
    assert!((info.output_format.sample_rate - 24_000.0).abs() < f64::EPSILON);
    assert!(output.frame_length()? > 0);
    Ok(())
}
