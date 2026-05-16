#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_format = AudioFormat::standard(48_000.0, 1, false)?;
    let output_format = AudioFormat::standard(24_000.0, 1, false)?;
    let converter = AudioConverter::new(&input_format, &output_format)?;

    let mut input = PCMBuffer::new(&input_format, 480)?;
    input.set_frame_length(480)?;
    let mut output = PCMBuffer::new(&output_format, 512)?;
    converter.convert_buffer(&input, &mut output)?;

    println!("converter info: {:?}", converter.info()?);
    println!("output buffer info: {:?}", output.info()?);
    Ok(())
}
