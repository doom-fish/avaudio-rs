use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let output = engine.output_node()?;

    println!("output format: {:?}", output.output_format(0)?);
    Ok(())
}
