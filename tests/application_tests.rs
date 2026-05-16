mod common;

use avaudio::prelude::*;

#[test]
fn audio_application_queries() -> Result<(), Box<dyn std::error::Error>> {
    let app = AudioApplication::shared();
    let input_muted = app.input_muted()?;
    let permission = app.record_permission()?;

    assert_eq!(app.input_muted()?, input_muted);
    assert!(matches!(
        permission,
        AudioApplicationRecordPermission::Undetermined
            | AudioApplicationRecordPermission::Denied
            | AudioApplicationRecordPermission::Granted
            | AudioApplicationRecordPermission::Other(_)
    ));
    Ok(())
}
