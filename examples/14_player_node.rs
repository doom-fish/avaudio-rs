use avaudio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 1, false)?;
    let mut buffer = PCMBuffer::new(&format, 256)?;
    buffer.set_frame_length(256)?;

    let engine = AudioEngine::new()?;
    let player = AudioPlayerNode::new()?;
    engine.attach_node(&player);
    engine.connect_node_to_main_mixer(&player, Some(&format));
    player.schedule_buffer(&buffer)?;

    println!("player info: {:?}", player.info()?);
    println!("scheduled buffer frames: {}", buffer.frame_length()?);
    Ok(())
}
