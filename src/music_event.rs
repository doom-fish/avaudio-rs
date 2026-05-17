//! `AVMusicEvent` / `AVMIDI*Event` value models used by sequencer tracks.

#![allow(
    clippy::default_trait_access,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::or_fun_call,
    clippy::redundant_pub_crate,
    clippy::too_many_lines
)]

use core::ffi::c_char;
use std::ffi::{CStr, CString};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::AVAudioError;

fn missing_field(field: &str, kind: &str) -> AVAudioError {
    AVAudioError::InvalidArgument(format!("missing `{field}` for `{kind}` music event payload"))
}

fn require<T>(value: Option<T>, field: &str, kind: &str) -> Result<T, AVAudioError> {
    value.ok_or_else(|| missing_field(field, kind))
}

/// Types of MIDI control-change messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum MidiControlChangeMessageType {
    BankSelect = 0,
    ModWheel = 1,
    Breath = 2,
    Foot = 4,
    PortamentoTime = 5,
    DataEntry = 6,
    Volume = 7,
    Balance = 8,
    Pan = 10,
    Expression = 11,
    Sustain = 64,
    Portamento = 65,
    Sostenuto = 66,
    Soft = 67,
    LegatoPedal = 68,
    Hold2Pedal = 69,
    FilterResonance = 71,
    ReleaseTime = 72,
    AttackTime = 73,
    Brightness = 74,
    DecayTime = 75,
    VibratoRate = 76,
    VibratoDepth = 77,
    VibratoDelay = 78,
    ReverbLevel = 91,
    ChorusLevel = 93,
    RpnLsb = 100,
    RpnMsb = 101,
    AllSoundOff = 120,
    ResetAllControllers = 121,
    AllNotesOff = 123,
    OmniModeOff = 124,
    OmniModeOn = 125,
    MonoModeOn = 126,
    MonoModeOff = 127,
}

impl TryFrom<i64> for MidiControlChangeMessageType {
    type Error = AVAudioError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let kind = match value {
            0 => Self::BankSelect,
            1 => Self::ModWheel,
            2 => Self::Breath,
            4 => Self::Foot,
            5 => Self::PortamentoTime,
            6 => Self::DataEntry,
            7 => Self::Volume,
            8 => Self::Balance,
            10 => Self::Pan,
            11 => Self::Expression,
            64 => Self::Sustain,
            65 => Self::Portamento,
            66 => Self::Sostenuto,
            67 => Self::Soft,
            68 => Self::LegatoPedal,
            69 => Self::Hold2Pedal,
            71 => Self::FilterResonance,
            72 => Self::ReleaseTime,
            73 => Self::AttackTime,
            74 => Self::Brightness,
            75 => Self::DecayTime,
            76 => Self::VibratoRate,
            77 => Self::VibratoDepth,
            78 => Self::VibratoDelay,
            91 => Self::ReverbLevel,
            93 => Self::ChorusLevel,
            100 => Self::RpnLsb,
            101 => Self::RpnMsb,
            120 => Self::AllSoundOff,
            121 => Self::ResetAllControllers,
            123 => Self::AllNotesOff,
            124 => Self::OmniModeOff,
            125 => Self::OmniModeOn,
            126 => Self::MonoModeOn,
            127 => Self::MonoModeOff,
            _ => {
                return Err(AVAudioError::InvalidArgument(format!(
                    "unsupported MIDI control-change message type {value}"
                )))
            }
        };
        Ok(kind)
    }
}

/// Types of MIDI meta events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum MidiMetaEventType {
    SequenceNumber = 0x00,
    Text = 0x01,
    Copyright = 0x02,
    TrackName = 0x03,
    Instrument = 0x04,
    Lyric = 0x05,
    Marker = 0x06,
    CuePoint = 0x07,
    MidiChannel = 0x20,
    MidiPort = 0x21,
    EndOfTrack = 0x2f,
    Tempo = 0x51,
    SmpteOffset = 0x54,
    TimeSignature = 0x58,
    KeySignature = 0x59,
    ProprietaryEvent = 0x7f,
}

impl TryFrom<i64> for MidiMetaEventType {
    type Error = AVAudioError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let kind = match value {
            0x00 => Self::SequenceNumber,
            0x01 => Self::Text,
            0x02 => Self::Copyright,
            0x03 => Self::TrackName,
            0x04 => Self::Instrument,
            0x05 => Self::Lyric,
            0x06 => Self::Marker,
            0x07 => Self::CuePoint,
            0x20 => Self::MidiChannel,
            0x21 => Self::MidiPort,
            0x2f => Self::EndOfTrack,
            0x51 => Self::Tempo,
            0x54 => Self::SmpteOffset,
            0x58 => Self::TimeSignature,
            0x59 => Self::KeySignature,
            0x7f => Self::ProprietaryEvent,
            _ => {
                return Err(AVAudioError::InvalidArgument(format!(
                    "unsupported MIDI meta-event type {value}"
                )))
            }
        };
        Ok(kind)
    }
}

/// `AVMIDINoteEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiNoteEvent {
    pub channel: u32,
    pub key: u32,
    pub velocity: u32,
    pub duration: f64,
}

impl MidiNoteEvent {
    pub fn new(channel: u32, key: u32, velocity: u32, duration: f64) -> Self {
        Self {
            channel,
            key,
            velocity,
            duration,
        }
    }
}

/// `AVMIDIControlChangeEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiControlChangeEvent {
    pub channel: u32,
    pub message_type: MidiControlChangeMessageType,
    pub value: u32,
}

impl MidiControlChangeEvent {
    pub fn new(channel: u32, message_type: MidiControlChangeMessageType, value: u32) -> Self {
        Self {
            channel,
            message_type,
            value,
        }
    }
}

/// `AVMIDIPolyPressureEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiPolyPressureEvent {
    pub channel: u32,
    pub key: u32,
    pub pressure: u32,
}

impl MidiPolyPressureEvent {
    pub fn new(channel: u32, key: u32, pressure: u32) -> Self {
        Self {
            channel,
            key,
            pressure,
        }
    }
}

/// `AVMIDIProgramChangeEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiProgramChangeEvent {
    pub channel: u32,
    pub program_number: u32,
}

impl MidiProgramChangeEvent {
    pub fn new(channel: u32, program_number: u32) -> Self {
        Self {
            channel,
            program_number,
        }
    }
}

/// `AVMIDIChannelPressureEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiChannelPressureEvent {
    pub channel: u32,
    pub pressure: u32,
}

impl MidiChannelPressureEvent {
    pub fn new(channel: u32, pressure: u32) -> Self {
        Self { channel, pressure }
    }
}

/// `AVMIDIPitchBendEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiPitchBendEvent {
    pub channel: u32,
    pub value: u32,
}

impl MidiPitchBendEvent {
    pub fn new(channel: u32, value: u32) -> Self {
        Self { channel, value }
    }
}

/// `AVMIDISysexEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiSysexEvent {
    pub data: Vec<u8>,
    pub reported_size_in_bytes: Option<u32>,
}

impl MidiSysexEvent {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self {
            data: data.into(),
            reported_size_in_bytes: None,
        }
    }

    pub fn size_in_bytes(&self) -> u32 {
        self.reported_size_in_bytes
            .unwrap_or(u32::try_from(self.data.len()).unwrap_or(u32::MAX))
    }
}

/// `AVMIDIMetaEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiMetaEvent {
    pub meta_type: MidiMetaEventType,
    pub data: Vec<u8>,
}

impl MidiMetaEvent {
    pub fn new(meta_type: MidiMetaEventType, data: impl Into<Vec<u8>>) -> Self {
        Self {
            meta_type,
            data: data.into(),
        }
    }
}

/// `AVMusicUserEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct MusicUserEvent {
    pub data: Vec<u8>,
    pub reported_size_in_bytes: Option<u32>,
}

impl MusicUserEvent {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self {
            data: data.into(),
            reported_size_in_bytes: None,
        }
    }

    pub fn size_in_bytes(&self) -> u32 {
        self.reported_size_in_bytes
            .unwrap_or(u32::try_from(self.data.len()).unwrap_or(u32::MAX))
    }
}

/// `AVExtendedNoteOnEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtendedNoteOnEvent {
    pub midi_note: f32,
    pub velocity: f32,
    pub instrument_id: u32,
    pub group_id: u32,
    pub duration: f64,
}

impl ExtendedNoteOnEvent {
    pub fn new(midi_note: f32, velocity: f32, instrument_id: u32, group_id: u32, duration: f64) -> Self {
        Self {
            midi_note,
            velocity,
            instrument_id,
            group_id,
            duration,
        }
    }
}

/// `AVParameterEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterEvent {
    pub parameter_id: u32,
    pub scope: u32,
    pub element: u32,
    pub value: f32,
}

impl ParameterEvent {
    pub fn new(parameter_id: u32, scope: u32, element: u32, value: f32) -> Self {
        Self {
            parameter_id,
            scope,
            element,
            value,
        }
    }
}

/// `AVAUPresetEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct AUPresetEvent {
    pub scope: u32,
    pub element: u32,
    pub preset_dictionary: Value,
}

impl AUPresetEvent {
    pub fn new(scope: u32, element: u32, preset_dictionary: Value) -> Self {
        Self {
            scope,
            element,
            preset_dictionary,
        }
    }
}

/// `AVExtendedTempoEvent`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtendedTempoEvent {
    pub tempo: f64,
}

impl ExtendedTempoEvent {
    pub fn new(tempo: f64) -> Self {
        Self { tempo }
    }
}

/// A Rust value-model for the `AVMusicEvent` class family.
#[derive(Debug, Clone, PartialEq)]
pub enum MusicEvent {
    MidiNote(MidiNoteEvent),
    MidiControlChange(MidiControlChangeEvent),
    MidiPolyPressure(MidiPolyPressureEvent),
    MidiProgramChange(MidiProgramChangeEvent),
    MidiChannelPressure(MidiChannelPressureEvent),
    MidiPitchBend(MidiPitchBendEvent),
    MidiSysex(MidiSysexEvent),
    MidiMeta(MidiMetaEvent),
    MusicUser(MusicUserEvent),
    ExtendedNoteOn(ExtendedNoteOnEvent),
    Parameter(ParameterEvent),
    AUPreset(AUPresetEvent),
    ExtendedTempo(ExtendedTempoEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MusicEventPayload {
    kind: String,
    channel: Option<u32>,
    key: Option<u32>,
    velocity: Option<u32>,
    duration: Option<f64>,
    message_type: Option<i64>,
    value: Option<u32>,
    pressure: Option<u32>,
    program_number: Option<u32>,
    data: Option<Vec<u8>>,
    size_in_bytes: Option<u32>,
    meta_type: Option<i64>,
    midi_note: Option<f32>,
    velocity_float: Option<f32>,
    instrument_id: Option<u32>,
    group_id: Option<u32>,
    parameter_id: Option<u32>,
    scope: Option<u32>,
    element: Option<u32>,
    value_float: Option<f32>,
    tempo: Option<f64>,
    preset_dictionary_json: Option<String>,
}

impl From<&MusicEvent> for MusicEventPayload {
    fn from(event: &MusicEvent) -> Self {
        match event {
            MusicEvent::MidiNote(event) => Self {
                kind: "midiNote".into(),
                channel: Some(event.channel),
                key: Some(event.key),
                velocity: Some(event.velocity),
                duration: Some(event.duration),
                message_type: None,
                value: None,
                pressure: None,
                program_number: None,
                data: None,
                size_in_bytes: None,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::MidiControlChange(event) => Self {
                kind: "midiControlChange".into(),
                channel: Some(event.channel),
                key: None,
                velocity: None,
                duration: None,
                message_type: Some(event.message_type as i64),
                value: Some(event.value),
                pressure: None,
                program_number: None,
                data: None,
                size_in_bytes: None,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::MidiPolyPressure(event) => Self {
                kind: "midiPolyPressure".into(),
                channel: Some(event.channel),
                key: Some(event.key),
                velocity: None,
                duration: None,
                message_type: None,
                value: None,
                pressure: Some(event.pressure),
                program_number: None,
                data: None,
                size_in_bytes: None,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::MidiProgramChange(event) => Self {
                kind: "midiProgramChange".into(),
                channel: Some(event.channel),
                key: None,
                velocity: None,
                duration: None,
                message_type: None,
                value: None,
                pressure: None,
                program_number: Some(event.program_number),
                data: None,
                size_in_bytes: None,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::MidiChannelPressure(event) => Self {
                kind: "midiChannelPressure".into(),
                channel: Some(event.channel),
                key: None,
                velocity: None,
                duration: None,
                message_type: None,
                value: None,
                pressure: Some(event.pressure),
                program_number: None,
                data: None,
                size_in_bytes: None,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::MidiPitchBend(event) => Self {
                kind: "midiPitchBend".into(),
                channel: Some(event.channel),
                key: None,
                velocity: None,
                duration: None,
                message_type: None,
                value: Some(event.value),
                pressure: None,
                program_number: None,
                data: None,
                size_in_bytes: None,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::MidiSysex(event) => Self {
                kind: "midiSysex".into(),
                channel: None,
                key: None,
                velocity: None,
                duration: None,
                message_type: None,
                value: None,
                pressure: None,
                program_number: None,
                data: Some(event.data.clone()),
                size_in_bytes: event.reported_size_in_bytes,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::MidiMeta(event) => Self {
                kind: "midiMeta".into(),
                channel: None,
                key: None,
                velocity: None,
                duration: None,
                message_type: None,
                value: None,
                pressure: None,
                program_number: None,
                data: Some(event.data.clone()),
                size_in_bytes: None,
                meta_type: Some(event.meta_type as i64),
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::MusicUser(event) => Self {
                kind: "musicUser".into(),
                channel: None,
                key: None,
                velocity: None,
                duration: None,
                message_type: None,
                value: None,
                pressure: None,
                program_number: None,
                data: Some(event.data.clone()),
                size_in_bytes: event.reported_size_in_bytes,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::ExtendedNoteOn(event) => Self {
                kind: "extendedNoteOn".into(),
                channel: None,
                key: None,
                velocity: None,
                duration: Some(event.duration),
                message_type: None,
                value: None,
                pressure: None,
                program_number: None,
                data: None,
                size_in_bytes: None,
                meta_type: None,
                midi_note: Some(event.midi_note),
                velocity_float: Some(event.velocity),
                instrument_id: Some(event.instrument_id),
                group_id: Some(event.group_id),
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::Parameter(event) => Self {
                kind: "parameter".into(),
                channel: None,
                key: None,
                velocity: None,
                duration: None,
                message_type: None,
                value: None,
                pressure: None,
                program_number: None,
                data: None,
                size_in_bytes: None,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: Some(event.parameter_id),
                scope: Some(event.scope),
                element: Some(event.element),
                value_float: Some(event.value),
                tempo: None,
                preset_dictionary_json: None,
            },
            MusicEvent::AUPreset(event) => Self {
                kind: "auPreset".into(),
                channel: None,
                key: None,
                velocity: None,
                duration: None,
                message_type: None,
                value: None,
                pressure: None,
                program_number: None,
                data: None,
                size_in_bytes: None,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: Some(event.scope),
                element: Some(event.element),
                value_float: None,
                tempo: None,
                preset_dictionary_json: Some(event.preset_dictionary.to_string()),
            },
            MusicEvent::ExtendedTempo(event) => Self {
                kind: "extendedTempo".into(),
                channel: None,
                key: None,
                velocity: None,
                duration: None,
                message_type: None,
                value: None,
                pressure: None,
                program_number: None,
                data: None,
                size_in_bytes: None,
                meta_type: None,
                midi_note: None,
                velocity_float: None,
                instrument_id: None,
                group_id: None,
                parameter_id: None,
                scope: None,
                element: None,
                value_float: None,
                tempo: Some(event.tempo),
                preset_dictionary_json: None,
            },
        }
    }
}

impl TryFrom<MusicEventPayload> for MusicEvent {
    type Error = AVAudioError;

    fn try_from(payload: MusicEventPayload) -> Result<Self, Self::Error> {
        match payload.kind.as_str() {
            "midiNote" => Ok(Self::MidiNote(MidiNoteEvent {
                channel: require(payload.channel, "channel", "midiNote")?,
                key: require(payload.key, "key", "midiNote")?,
                velocity: require(payload.velocity, "velocity", "midiNote")?,
                duration: require(payload.duration, "duration", "midiNote")?,
            })),
            "midiControlChange" => Ok(Self::MidiControlChange(MidiControlChangeEvent {
                channel: require(payload.channel, "channel", "midiControlChange")?,
                message_type: MidiControlChangeMessageType::try_from(require(
                    payload.message_type,
                    "messageType",
                    "midiControlChange",
                )?)?,
                value: require(payload.value, "value", "midiControlChange")?,
            })),
            "midiPolyPressure" => Ok(Self::MidiPolyPressure(MidiPolyPressureEvent {
                channel: require(payload.channel, "channel", "midiPolyPressure")?,
                key: require(payload.key, "key", "midiPolyPressure")?,
                pressure: require(payload.pressure, "pressure", "midiPolyPressure")?,
            })),
            "midiProgramChange" => Ok(Self::MidiProgramChange(MidiProgramChangeEvent {
                channel: require(payload.channel, "channel", "midiProgramChange")?,
                program_number: require(payload.program_number, "programNumber", "midiProgramChange")?,
            })),
            "midiChannelPressure" => Ok(Self::MidiChannelPressure(MidiChannelPressureEvent {
                channel: require(payload.channel, "channel", "midiChannelPressure")?,
                pressure: require(payload.pressure, "pressure", "midiChannelPressure")?,
            })),
            "midiPitchBend" => Ok(Self::MidiPitchBend(MidiPitchBendEvent {
                channel: require(payload.channel, "channel", "midiPitchBend")?,
                value: require(payload.value, "value", "midiPitchBend")?,
            })),
            "midiSysex" => Ok(Self::MidiSysex(MidiSysexEvent {
                data: payload.data.unwrap_or_default(),
                reported_size_in_bytes: payload.size_in_bytes,
            })),
            "midiMeta" => Ok(Self::MidiMeta(MidiMetaEvent {
                meta_type: MidiMetaEventType::try_from(require(
                    payload.meta_type,
                    "metaType",
                    "midiMeta",
                )?)?,
                data: payload.data.unwrap_or_default(),
            })),
            "musicUser" => Ok(Self::MusicUser(MusicUserEvent {
                data: payload.data.unwrap_or_default(),
                reported_size_in_bytes: payload.size_in_bytes,
            })),
            "extendedNoteOn" => Ok(Self::ExtendedNoteOn(ExtendedNoteOnEvent {
                midi_note: require(payload.midi_note, "midiNote", "extendedNoteOn")?,
                velocity: require(payload.velocity_float, "velocityFloat", "extendedNoteOn")?,
                instrument_id: require(payload.instrument_id, "instrumentId", "extendedNoteOn")?,
                group_id: require(payload.group_id, "groupId", "extendedNoteOn")?,
                duration: require(payload.duration, "duration", "extendedNoteOn")?,
            })),
            "parameter" => Ok(Self::Parameter(ParameterEvent {
                parameter_id: require(payload.parameter_id, "parameterId", "parameter")?,
                scope: require(payload.scope, "scope", "parameter")?,
                element: require(payload.element, "element", "parameter")?,
                value: require(payload.value_float, "valueFloat", "parameter")?,
            })),
            "auPreset" => {
                let preset_dictionary = payload
                    .preset_dictionary_json
                    .as_deref()
                    .map(serde_json::from_str)
                    .transpose()
                    .map_err(|error| {
                        AVAudioError::InvalidArgument(format!(
                            "failed to decode AVAUPresetEvent dictionary JSON: {error}"
                        ))
                    })?
                    .unwrap_or(Value::Object(Default::default()));
                Ok(Self::AUPreset(AUPresetEvent {
                    scope: require(payload.scope, "scope", "auPreset")?,
                    element: require(payload.element, "element", "auPreset")?,
                    preset_dictionary,
                }))
            }
            "extendedTempo" => Ok(Self::ExtendedTempo(ExtendedTempoEvent {
                tempo: require(payload.tempo, "tempo", "extendedTempo")?,
            })),
            other => Err(AVAudioError::InvalidArgument(format!(
                "unsupported music event kind `{other}`"
            ))),
        }
    }
}

pub(crate) fn event_to_json_cstring(event: &MusicEvent) -> Result<CString, AVAudioError> {
    let payload = MusicEventPayload::from(event);
    let json = serde_json::to_string(&payload)
        .map_err(|error| AVAudioError::InvalidArgument(format!("failed to encode music event: {error}")))?;
    CString::new(json)
        .map_err(|error| AVAudioError::InvalidArgument(format!("music event JSON contains NUL byte: {error}")))
}

pub(crate) fn event_from_json_str(json: &str) -> Result<MusicEvent, AVAudioError> {
    let payload = serde_json::from_str::<MusicEventPayload>(json).map_err(|error| {
        AVAudioError::InvalidArgument(format!("failed to decode music event payload JSON: {error}"))
    })?;
    MusicEvent::try_from(payload)
}

pub(crate) fn event_from_json_ptr(json_ptr: *const c_char) -> Result<MusicEvent, AVAudioError> {
    if json_ptr.is_null() {
        return Err(AVAudioError::InvalidArgument(
            "music event payload pointer was null".into(),
        ));
    }
    let json = unsafe { CStr::from_ptr(json_ptr) }
        .to_string_lossy()
        .into_owned();
    event_from_json_str(&json)
}
