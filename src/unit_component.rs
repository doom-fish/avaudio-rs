//! [`AudioUnitComponentManager`] — component discovery and standard type constants.

#![allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools
)]

use core::ffi::c_char;
use core::ptr;

use serde::Deserialize;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::unit::AudioComponentDescription;
use crate::util::parse_json_and_free;

/// Metadata about an installed audio-unit component.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioUnitComponentInfo {
    /// Component display name.
    pub name: String,
    /// Raw component description.
    pub component_description: AudioComponentDescription,
    /// Standard component type name.
    pub type_name: String,
    /// Localized display name for the component type.
    pub localized_type_name: String,
    /// Component manufacturer name.
    pub manufacturer_name: String,
    /// Hex-encoded version number split into major/minor/bugfix nybbles.
    pub version: u64,
    /// Human-readable version string.
    pub version_string: String,
    /// Supported Mach-O architecture constants.
    pub available_architectures: Vec<i64>,
    /// Whether the component is safe to load in a sandbox.
    pub sandbox_safe: bool,
    /// Whether the component accepts MIDI input.
    #[serde(rename = "hasMIDIInput")]
    pub has_midi_input: bool,
    /// Whether the component emits MIDI output.
    #[serde(rename = "hasMIDIOutput")]
    pub has_midi_output: bool,
    /// User-assigned component tags.
    pub user_tag_names: Vec<String>,
    /// All user and system component tags.
    pub all_tag_names: Vec<String>,
    /// Optional icon URL.
    #[serde(rename = "iconURL")]
    pub icon_url: Option<String>,
    /// Whether the component has passed AU validation.
    #[serde(rename = "passesAUVal")]
    pub passes_auval: bool,
    /// Whether the component exposes a custom view.
    pub has_custom_view: bool,
}

/// Standard strings surfaced by `AVAudioUnitComponent.h`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioUnitComponentConstants {
    /// `AVAudioUnitComponentTagsDidChangeNotification`.
    pub tags_did_change_notification: String,
    /// `AVAudioUnitManufacturerNameApple`.
    pub manufacturer_name_apple: String,
    /// `AVAudioUnitTypeEffect`.
    pub type_effect: String,
    /// `AVAudioUnitTypeFormatConverter`.
    pub type_format_converter: String,
    /// `AVAudioUnitTypeGenerator`.
    pub type_generator: String,
    /// `AVAudioUnitTypeMIDIProcessor`.
    #[serde(rename = "typeMIDIProcessor")]
    pub type_midi_processor: String,
    /// `AVAudioUnitTypeMixer`.
    pub type_mixer: String,
    /// `AVAudioUnitTypeMusicDevice`.
    pub type_music_device: String,
    /// `AVAudioUnitTypeMusicEffect`.
    pub type_music_effect: String,
    /// `AVAudioUnitTypeOfflineEffect`.
    pub type_offline_effect: String,
    /// `AVAudioUnitTypeOutput`.
    pub type_output: String,
    /// `AVAudioUnitTypePanner`.
    pub type_panner: String,
}

/// Access to the shared `AVAudioUnitComponentManager` singleton.
pub struct AudioUnitComponentManager;

impl AudioUnitComponentManager {
    /// Returns the shared component manager.
    pub const fn shared() -> Self {
        Self
    }

    /// Returns all user and system tags known to the component manager.
    pub fn tag_names(&self) -> Result<Vec<String>, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_unit_component_manager_tag_names_json(&mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns localized standard system tags.
    pub fn standard_localized_tag_names(&self) -> Result<Vec<String>, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr =
            unsafe { ffi::av_audio_unit_component_manager_standard_localized_tag_names_json(&mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns lightweight snapshots of installed audio-unit components.
    pub fn components(&self) -> Result<Vec<AudioUnitComponentInfo>, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_unit_component_manager_components_json(&mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }

    /// Returns the standard AVAudioUnit component type/manufacturer strings.
    pub fn standard_constants(&self) -> Result<AudioUnitComponentConstants, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_unit_component_constants_json(&mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        parse_json_and_free(json_ptr)
    }
}
