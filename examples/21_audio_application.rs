use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = AudioApplication::shared();
    println!("record permission: {:?}", app.record_permission()?);
    println!("input muted: {}", app.input_muted()?);
    Ok(())
}
