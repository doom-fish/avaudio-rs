use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(44_100.0, 2, false)?;
    println!("format info: {:?}", format.info()?);
    println!("common format: {:?}", format.common_format()?);
    println!("sample rate: {:.1}", format.sample_rate()?);
    println!("channel count: {}", format.channel_count()?);
    println!("interleaved: {}", format.is_interleaved()?);
    Ok(())
}
