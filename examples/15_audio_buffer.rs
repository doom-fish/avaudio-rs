use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 1, false)?;
    let mut buffer = PCMBuffer::new(&format, 128)?;
    buffer.set_frame_length(64)?;

    let info = buffer.buffer_info()?;
    println!("audio buffer info: {info:?}");
    Ok(())
}
