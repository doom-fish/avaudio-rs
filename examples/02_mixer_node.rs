#[path = "support/mod.rs"]
mod support;

use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = AudioEngine::new()?;
    let mixer = AudioMixerNode::new()?;
    engine.attach_node(&mixer);
    engine.connect_node_to_main_mixer(&mixer, None);
    mixer.set_output_volume(0.42);

    let main_mixer = engine.main_mixer_node()?;
    println!(
        "standalone mixer output volume: {:.2}",
        mixer.output_volume()
    );
    println!(
        "engine main mixer volume: {:.2}",
        main_mixer.output_volume()
    );
    println!("✅ mixer node ready");
    Ok(())
}
