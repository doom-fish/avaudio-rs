mod common;

use avaudio::prelude::*;

#[test]
fn component_manager_lists_components() -> Result<(), Box<dyn std::error::Error>> {
    let manager = AudioUnitComponentManager::shared();
    let constants = manager.standard_constants()?;
    let components = manager.components()?;
    let _tags = manager.tag_names()?;
    let _localized_tags = manager.standard_localized_tag_names()?;

    assert_ne!(constants.tags_did_change_notification, "");
    assert_ne!(constants.manufacturer_name_apple, "");
    assert_ne!(constants.type_effect, "");
    assert_ne!(components, [] as [avaudio::AudioUnitComponentInfo; 0]);
    assert!(components
        .iter()
        .any(|component| !component.name.is_empty()));
    Ok(())
}
