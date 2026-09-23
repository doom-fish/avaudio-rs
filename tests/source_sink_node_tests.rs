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

    engine.attach_node(&source)?;
    engine.attach_node(&sink)?;
    engine.connect_node_to_main_mixer(&source, Some(&format))?;

    assert!(!engine.is_running()?);
    Ok(())
}

#[repr(C)]
struct AudioBufferRaw {
    number_channels: u32,
    data_byte_size: u32,
    data: *mut core::ffi::c_void,
}

#[repr(C)]
struct AudioBufferListRaw {
    number_buffers: u32,
    buffers: [AudioBufferRaw; 1],
}

fn fill_output(context: &AudioSourceRenderContext, value: f32) {
    let list = context.output_data_ptr().cast::<AudioBufferListRaw>();
    unsafe {
        let count = (*list).number_buffers as usize;
        let buffers = core::ptr::addr_of_mut!((*list).buffers).cast::<AudioBufferRaw>();
        for index in 0..count {
            let buffer = buffers.add(index);
            let samples = (*buffer).data_byte_size as usize / core::mem::size_of::<f32>();
            core::slice::from_raw_parts_mut((*buffer).data.cast::<f32>(), samples).fill(value);
        }
    }
}

fn render_source(panics: bool) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 2, false)?;
    let engine = AudioEngine::new()?;
    engine.enable_manual_rendering_mode(AudioEngineManualRenderingMode::Offline, &format, 1024)?;
    let source = AudioSourceNode::new_with_format(&format, move |context| {
        fill_output(context, 0.5);
        assert!(!panics, "source render failure");
        0
    })?;
    engine.attach_node(&source)?;
    engine.connect_node_to_main_mixer(&source, Some(&format))?;
    engine.start()?;
    let mut output = PCMBuffer::new(&engine.manual_rendering_format()?, 1024)?;
    let _ = engine.render_offline(512, &mut output);
    engine.stop();
    let Some(PCMChannelData::Deinterleaved(channels)) = output.channel_data::<f32>() else {
        return Err("manual rendering output should be deinterleaved float".into());
    };
    Ok(channels
        .iter()
        .flat_map(|channel| channel.iter().copied())
        .collect())
}

#[test]
fn source_node_output_reaches_the_mixer() -> Result<(), Box<dyn std::error::Error>> {
    let samples = render_source(false)?;
    assert!(!samples.is_empty());
    assert!(samples.iter().all(|sample| (sample - 0.5).abs() < 1e-6));
    Ok(())
}

#[test]
fn panicking_source_node_renders_silence() -> Result<(), Box<dyn std::error::Error>> {
    let samples = render_source(true)?;
    assert!(!samples.is_empty());
    assert!(samples.iter().all(|sample| sample.to_bits() == 0));
    Ok(())
}
