use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 1, false)?;
    let mut buffer = PCMBuffer::new(&format, 256)?;
    buffer.set_frame_length(128)?;

    println!("pcm buffer info: {:?}", buffer.info()?);
    println!("pcm buffer format: {:?}", buffer.format()?.info()?);
    Ok(())
}
