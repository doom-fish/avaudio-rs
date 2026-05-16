mod common;

use avaudio::prelude::*;

#[test]
fn component_manager_lists_components() -> Result<(), Box<dyn std::error::Error>> {
    let manager = AudioUnitComponentManager::shared();
    let constants = manager.standard_constants()?;
    let components = manager.components()?;
    let _tags = manager.tag_names()?;
    let _localized_tags = manager.standard_localized_tag_names()?;

    assert!(!constants.tags_did_change_notification.is_empty());
    assert!(!constants.manufacturer_name_apple.is_empty());
    assert!(!constants.type_effect.is_empty());
    assert!(!components.is_empty());
    assert!(components.iter().any(|component| !component.name.is_empty()));
    Ok(())
}
