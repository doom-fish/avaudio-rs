#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let input = engine.input_node()?;
    let output = engine.output_node()?;

    println!("input output format: {:?}", input.output_format(0)?);
    println!("input input format: {:?}", input.input_format(0)?);
    println!("output format: {:?}", output.output_format(0)?);

    input.install_tap_scaffold(0, 256, None)?;
    input.remove_tap(0)?;
    println!("installed and removed input tap scaffold");
    Ok(())
}
