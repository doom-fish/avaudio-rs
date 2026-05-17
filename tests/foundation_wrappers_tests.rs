mod common;

use avaudio::prelude::*;

use self::common::{artifacts_dir, make_test_audio, make_test_compressed_audio};

#[test]
fn foundation_wrapper_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let mono = AudioChannelLayout::mono()?;
    let stereo = AudioChannelLayout::stereo()?;
    let stereo_again = AudioChannelLayout::new_with_layout_tag(AUDIO_CHANNEL_LAYOUT_TAG_STEREO)?;
    assert_eq!(mono.channel_count()?, 1);
    assert_eq!(stereo.channel_count()?, 2);
    assert!(stereo.equals(&stereo_again));

    let host_time = AudioTime::host_time_for_seconds(1.0);
    let anchor = AudioTime::new_host_and_sample_time(host_time, 48_000, 48_000.0);
    assert!(anchor.host_time_valid()?);
    assert!(anchor.sample_time_valid()?);
    assert!((AudioTime::seconds_for_host_time(anchor.host_time()?) - 1.0).abs() < 0.05);
    let sample_only = AudioTime::new_sample_time(128, 48_000.0);
    let extrapolated = sample_only
        .extrapolate_from_anchor(&anchor)
        .ok_or("failed to extrapolate audio time")?;
    assert!(extrapolated.host_time_valid()?);
    assert_eq!(extrapolated.sample_time()?, 128);

    let player = AudioPlayerNode::new()?;
    let point = AudioConnectionPoint::new(&player, 0)?;
    assert_eq!(point.bus()?, 0);
    assert!(point.points_to(&player)?);

    let artifacts = artifacts_dir()?;
    let input_path = artifacts.join("foundation-wrapper-input.aiff");
    let output_path = artifacts.join("foundation-wrapper-output.m4a");
    make_test_audio(&input_path)?;
    make_test_compressed_audio(&input_path, &output_path)?;
    let file = AudioFile::open_for_reading(&output_path)?;
    let format = file.file_format()?;
    let buffer = AudioCompressedBuffer::new(&format, 8, 4096)?;
    assert_eq!(buffer.packet_capacity()?, 8);
    buffer.set_packet_count(0)?;
    buffer.set_byte_length(0)?;
    assert!(buffer.maximum_packet_size()? > 0);
    let _buffer_info = buffer.buffer_info()?;
    let _format = buffer.format()?;
    Ok(())
}
