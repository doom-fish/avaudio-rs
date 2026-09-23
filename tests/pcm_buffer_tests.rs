mod common;

use avaudio::prelude::*;

#[test]
fn pcm_buffer_new_and_set_length() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 1, false)?;
    let mut buffer = PCMBuffer::new(&format, 512)?;
    buffer.set_frame_length(256)?;

    assert_eq!(buffer.frame_capacity()?, 512);
    assert_eq!(buffer.frame_length()?, 256);
    assert!((buffer.format()?.sample_rate()? - 48_000.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn float_channel_data_follows_frame_length() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(48_000.0, 2, false)?;
    let mut buffer = PCMBuffer::new(&format, 16)?;
    let Some(PCMChannelData::Deinterleaved(empty)) = buffer.channel_data::<f32>() else {
        panic!("standard format should be deinterleaved float");
    };
    assert_eq!(empty.len(), 2);
    assert!(empty.iter().all(|channel| channel.is_empty()));

    buffer.set_frame_length(8)?;
    let Some(PCMChannelDataMut::Deinterleaved(mut channels)) = buffer.channel_data_mut::<f32>()?
    else {
        panic!("standard format should be deinterleaved float");
    };
    assert!(channels.iter().all(|channel| channel.len() == 8));
    channels[0].fill(0.5);
    channels[1].fill(-0.5);

    let Some(PCMChannelData::Deinterleaved(channels)) = buffer.channel_data::<f32>() else {
        panic!("standard format should be deinterleaved float");
    };
    assert!(channels[0]
        .iter()
        .all(|sample| (sample - 0.5).abs() < f32::EPSILON));
    assert!(channels[1]
        .iter()
        .all(|sample| (sample + 0.5).abs() < f32::EPSILON));

    assert!(buffer.set_frame_length(17).is_err());
    assert_eq!(buffer.frame_length()?, 8);
    buffer.set_frame_length(16)?;
    let Some(PCMChannelData::Deinterleaved(full)) = buffer.channel_data::<f32>() else {
        panic!("standard format should be deinterleaved float");
    };
    assert!(full.iter().all(|channel| channel.len() == 16));
    Ok(())
}

#[test]
fn channel_data_is_none_for_other_sample_types() -> Result<(), Box<dyn std::error::Error>> {
    let float = PCMBuffer::new(&AudioFormat::standard(48_000.0, 2, false)?, 8)?;
    assert!(float.channel_data::<i16>().is_none());
    assert!(float.channel_data::<i32>().is_none());

    let mut int16 = PCMBuffer::new(
        &AudioFormat::with_common_format(AudioCommonFormat::PcmInt16, 48_000.0, 2, false)?,
        8,
    )?;
    assert!(int16.channel_data::<f32>().is_none());
    assert!(int16.channel_data_mut::<i32>()?.is_none());
    assert!(int16.channel_data::<i16>().is_some());

    let float64 = PCMBuffer::new(
        &AudioFormat::with_common_format(AudioCommonFormat::PcmFloat64, 48_000.0, 2, false)?,
        8,
    )?;
    assert!(float64.channel_data::<f32>().is_none());
    assert!(float64.channel_data::<i16>().is_none());
    assert!(float64.channel_data::<i32>().is_none());
    assert!(float64.copy_samples().is_none());

    assert!(matches!(
        AudioFormat::with_common_format(AudioCommonFormat::Other, 48_000.0, 2, false),
        Err(AVAudioError::InvalidArgument(_))
    ));
    Ok(())
}

#[test]
fn interleaved_int16_data_is_one_frame_major_slice() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::with_common_format(AudioCommonFormat::PcmInt16, 48_000.0, 2, true)?;
    let mut buffer = PCMBuffer::new(&format, 4)?;
    buffer.set_frame_length(3)?;
    let Some(PCMChannelDataMut::Interleaved {
        channel_count,
        samples,
    }) = buffer.channel_data_mut::<i16>()?
    else {
        panic!("interleaved int16 format should expose one interleaved slice");
    };
    assert_eq!(channel_count, 2);
    assert_eq!(samples.len(), 6);
    samples.copy_from_slice(&[1, -1, 2, -2, 3, -3]);

    assert_eq!(
        buffer.copy_samples(),
        Some(PCMSamples::Int16(vec![vec![1, 2, 3], vec![-1, -2, -3]]))
    );
    Ok(())
}

#[test]
fn deinterleaved_int32_data_copies_per_channel() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::with_common_format(AudioCommonFormat::PcmInt32, 48_000.0, 2, false)?;
    let mut buffer = PCMBuffer::new(&format, 4)?;
    buffer.set_frame_length(2)?;
    let Some(PCMChannelDataMut::Deinterleaved(mut channels)) = buffer.channel_data_mut::<i32>()?
    else {
        panic!("deinterleaved int32 format should expose one slice per channel");
    };
    channels[0].copy_from_slice(&[i32::MAX, 7]);
    channels[1].copy_from_slice(&[i32::MIN, -7]);

    assert_eq!(
        buffer.copy_samples(),
        Some(PCMSamples::Int32(vec![
            vec![i32::MAX, 7],
            vec![i32::MIN, -7]
        ]))
    );
    Ok(())
}
