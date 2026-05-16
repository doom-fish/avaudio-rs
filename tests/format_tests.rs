use avaudio::prelude::*;

#[test]
fn standard_format_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let format = AudioFormat::standard(44_100.0, 2, false)?;
    let info = format.info()?;

    assert_eq!(info.channel_count, 2);
    assert!((info.sample_rate - 44_100.0).abs() < f64::EPSILON);
    assert_eq!(format.common_format()?, AudioCommonFormat::PcmFloat32);
    assert!(!format.is_interleaved()?);
    Ok(())
}
