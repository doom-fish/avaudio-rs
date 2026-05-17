use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = AudioUnitComponentManager::shared();
    let constants = manager.standard_constants()?;
    let components = manager.components()?;

    println!(
        "Apple manufacturer constant: {}",
        constants.manufacturer_name_apple
    );
    println!("Found {} audio-unit components", components.len());
    for component in components.iter().take(5) {
        println!(
            "- {} ({}) by {}",
            component.name, component.type_name, component.manufacturer_name
        );
    }
    Ok(())
}
