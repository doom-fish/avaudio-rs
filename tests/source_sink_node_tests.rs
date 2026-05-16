mod common;

use avaudio::prelude::*;

#[test]
fn source_and_sink_node_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(44_100.0, 1, false)?;
    let source = AudioSourceNode::new_with_format(&format, |context| {
        context.set_is_silence(true);
        0
    })?;
    let sink = AudioSinkNode::new(|context| {
        let _ = context.frame_count();
        0
    })?;
    let engine = AudioEngine::new()?;

    engine.attach_node(&source);
    engine.attach_node(&sink);
    engine.connect_node_to_main_mixer(&source, Some(&format));

    assert!(!engine.is_running()?);
    Ok(())
}
