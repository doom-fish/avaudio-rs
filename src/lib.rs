#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod application;
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;
mod audio_file;
mod buffer;
mod channel_layout;
mod compressed_buffer;
mod connection_point;
mod converter;
mod engine;
mod environment_node;
mod error;
mod ffi;
mod file;
mod format;
mod input_node;
mod input_output_node;
mod io_node;
mod mixer;
mod mixing;
mod music_event;
mod music_track;
mod node;
mod settings;
mod session_types;
mod time;
mod types;
mod output_node;
mod pcm_buffer;
mod player;
mod recorder;
mod routing_arbiter;
mod sequencer;
mod session;
mod session_capability;
mod simple_player;
mod sink_node;
mod source_node;
mod unit;
mod unit_component;
mod unit_delay;
mod unit_generator;
mod unit_distortion;
mod unit_effect;
mod unit_eq;
mod unit_reverb;
mod unit_sampler;
mod unit_time_effect;
mod unit_time_pitch;
mod unit_varispeed;
mod unit_midi_instrument;
mod util;

pub use application::{AudioApplication, AudioApplicationRecordPermission};
pub use audio_file::{AudioFile, AudioFileInfo};
pub use buffer::{AudioBufferHandle, AudioBufferInfo};
pub use channel_layout::{
    AudioChannelLayout, AudioChannelLayoutInfo, AudioChannelLayoutTag, AUDIO_CHANNEL_LAYOUT_TAG_MONO,
    AUDIO_CHANNEL_LAYOUT_TAG_STEREO,
};
pub use compressed_buffer::{AudioCompressedBuffer, AudioCompressedBufferInfo};
pub use connection_point::{AudioConnectionPoint, AudioConnectionPointInfo};
pub use converter::{
    AudioConverter, AudioConverterInfo, AudioConverterInputStatus, AudioConverterOutputStatus,
    AudioConverterPrimeInfo, AudioConverterPrimeMethod,
};
pub use engine::{
    AudioEngine, AudioEngineInfo, AudioEngineManualRenderingError,
    AudioEngineManualRenderingInfo, AudioEngineManualRenderingMode,
    AudioEngineManualRenderingStatus,
};
pub use environment_node::{
    AudioDistanceAttenuation, AudioEnvironmentNode, AudioListenerOrientation, AudioListenerPosition,
};
pub use error::AVAudioError;
pub use format::{AudioCommonFormat, AudioFormat, AudioFormatInfo};
pub use input_node::{AudioInputNode, AudioManualRenderingInput};
pub use io_node::{
    AudioIONode, AudioVoiceProcessingOtherAudioDuckingConfiguration,
    AudioVoiceProcessingOtherAudioDuckingLevel, AudioVoiceProcessingSpeechActivityEvent,
};
pub use mixer::AudioMixerNode;
pub use mixing::{Audio3DMixing, AudioMixing, AudioMixingDestination, AudioStereoMixing};
pub use music_event::{
    AUPresetEvent, ExtendedNoteOnEvent, ExtendedTempoEvent, MidiChannelPressureEvent,
    MidiControlChangeEvent, MidiControlChangeMessageType, MidiMetaEvent, MidiMetaEventType,
    MidiNoteEvent, MidiPitchBendEvent, MidiPolyPressureEvent, MidiProgramChangeEvent,
    MidiSysexEvent, MusicEvent, MusicUserEvent, ParameterEvent,
};
pub use music_track::{
    BeatRange, MusicTrack, MusicTrackInfo, TrackEvent, TrackEventAction,
    MUSIC_TIME_STAMP_END_OF_TRACK, MUSIC_TRACK_LOOP_COUNT_FOREVER,
};
pub use node::AudioNodeHandle;
pub use output_node::AudioOutputNode;
pub use pcm_buffer::{PCMBuffer, PCMBufferInfo};
pub use player::{
    AudioPlayerNode, AudioPlayerNodeBufferOptions, AudioPlayerNodeCompletionCallbackType,
    AudioPlayerNodeInfo,
};
pub use recorder::{AudioRecorder, AudioRecorderDelegate};
pub use routing_arbiter::{AudioRoutingArbiter, AudioRoutingArbitrationCategory};
pub use sequencer::{
    AudioSequencer, AudioSequencerInfo, AudioSequencerInfoDictionaryKeys,
    AudioSequencerUserEvent, MusicSequenceLoadOptions,
};
pub use session::AudioSession;
pub use session_capability::AudioSessionCapability;
pub use session_types::{
    AudioSessionActivationOptions, AudioSessionAnchoringStrategy, AudioSessionIOType,
    AudioSessionInterruptionOptions, AudioSessionInterruptionType,
    AudioSessionMicrophoneInjectionMode, AudioSessionPromptStyle, AudioSessionRenderingMode,
    AudioSessionRouteChangeReason, AudioSessionSetActiveOptions,
    AudioSessionSilenceSecondaryAudioHintType, AudioSessionSoundStageSize,
    AudioSessionSpatialExperience, AudioStereoOrientation,
};
pub use settings::{
    AudioBitRateStrategy, AudioContentSource, AudioDynamicRangeControlConfiguration,
    AudioQuality, AudioSettingsConstants,
};
pub use simple_player::{AudioSimplePlayer, AudioSimplePlayerDelegate};
pub use time::{AudioTime, AudioTimeInfo};
pub use types::{
    Audio3DMixingPointSourceInHeadMode, Audio3DMixingRenderingAlgorithm,
    Audio3DMixingSourceMode, Audio3DVector, Audio3DVectorOrientation, AudioChannelCount,
    AudioEnvironmentOutputType, AudioFrameCount, AudioFramePosition, AudioNodeBus,
    AudioPacketCount,
};
pub use sink_node::{AudioSinkNode, AudioSinkRenderContext};
pub use source_node::{AudioSourceNode, AudioSourceRenderContext};
pub use unit::{
    AUAudioUnitHandle, AudioComponentDescription, AudioComponentInstantiationOptions, AudioUnit,
    AudioUnitMetadata,
};
pub use unit_component::{
    AudioUnitComponentConstants, AudioUnitComponentInfo, AudioUnitComponentManager,
};
pub use unit_delay::AudioUnitDelay;
pub use unit_distortion::{AudioUnitDistortion, AudioUnitDistortionPreset};
pub use unit_effect::{AudioUnitEffect, AudioUnitHandle, AudioUnitInfo};
pub use unit_generator::AudioUnitGenerator;
pub use unit_eq::{AudioEQBandInfo, AudioEQBandParams, AudioUnitEQ};
pub use unit_midi_instrument::{
    AudioUnitMIDIInstrument, AudioUnitMIDIInstrumentHandle, MidiEventPacket, MidiProtocol,
};
pub use unit_reverb::{AudioUnitReverb, AudioUnitReverbPreset};
pub use unit_sampler::AudioUnitSampler;
pub use unit_time_effect::AudioUnitTimeEffect;
pub use unit_time_pitch::AudioUnitTimePitch;
pub use unit_varispeed::AudioUnitVarispeed;

/// Common imports.
pub mod prelude {
    pub use crate::application::{AudioApplication, AudioApplicationRecordPermission};
    pub use crate::audio_file::{AudioFile, AudioFileInfo};
    pub use crate::buffer::{AudioBufferHandle, AudioBufferInfo};
    pub use crate::channel_layout::{
        AudioChannelLayout, AudioChannelLayoutInfo, AudioChannelLayoutTag,
        AUDIO_CHANNEL_LAYOUT_TAG_MONO, AUDIO_CHANNEL_LAYOUT_TAG_STEREO,
    };
    pub use crate::compressed_buffer::{AudioCompressedBuffer, AudioCompressedBufferInfo};
    pub use crate::connection_point::{AudioConnectionPoint, AudioConnectionPointInfo};
    pub use crate::converter::{
        AudioConverter, AudioConverterInfo, AudioConverterInputStatus,
        AudioConverterOutputStatus, AudioConverterPrimeInfo, AudioConverterPrimeMethod,
    };
    pub use crate::engine::{
        AudioEngine, AudioEngineInfo, AudioEngineManualRenderingError,
        AudioEngineManualRenderingInfo, AudioEngineManualRenderingMode,
        AudioEngineManualRenderingStatus,
    };
    pub use crate::environment_node::{
        AudioDistanceAttenuation, AudioEnvironmentNode, AudioListenerOrientation,
        AudioListenerPosition,
    };
    pub use crate::error::AVAudioError;
    pub use crate::format::{AudioCommonFormat, AudioFormat, AudioFormatInfo};
    pub use crate::input_node::{AudioInputNode, AudioManualRenderingInput};
    pub use crate::io_node::{
        AudioIONode, AudioVoiceProcessingOtherAudioDuckingConfiguration,
        AudioVoiceProcessingOtherAudioDuckingLevel, AudioVoiceProcessingSpeechActivityEvent,
    };
    pub use crate::mixer::AudioMixerNode;
    pub use crate::mixing::{Audio3DMixing, AudioMixing, AudioMixingDestination, AudioStereoMixing};
    pub use crate::music_event::{
        AUPresetEvent, ExtendedNoteOnEvent, ExtendedTempoEvent, MidiChannelPressureEvent,
        MidiControlChangeEvent, MidiControlChangeMessageType, MidiMetaEvent, MidiMetaEventType,
        MidiNoteEvent, MidiPitchBendEvent, MidiPolyPressureEvent, MidiProgramChangeEvent,
        MidiSysexEvent, MusicEvent, MusicUserEvent, ParameterEvent,
    };
    pub use crate::music_track::{
        BeatRange, MusicTrack, MusicTrackInfo, TrackEvent, TrackEventAction,
        MUSIC_TIME_STAMP_END_OF_TRACK, MUSIC_TRACK_LOOP_COUNT_FOREVER,
    };
    pub use crate::node::AudioNodeHandle;
    pub use crate::output_node::AudioOutputNode;
    pub use crate::pcm_buffer::{PCMBuffer, PCMBufferInfo};
    pub use crate::player::{
        AudioPlayerNode, AudioPlayerNodeBufferOptions, AudioPlayerNodeCompletionCallbackType,
        AudioPlayerNodeInfo,
    };
    pub use crate::recorder::{AudioRecorder, AudioRecorderDelegate};
    pub use crate::routing_arbiter::{AudioRoutingArbiter, AudioRoutingArbitrationCategory};
    pub use crate::sequencer::{
        AudioSequencer, AudioSequencerInfo, AudioSequencerInfoDictionaryKeys,
        AudioSequencerUserEvent, MusicSequenceLoadOptions,
    };
    pub use crate::session::AudioSession;
    pub use crate::session_capability::AudioSessionCapability;
    pub use crate::session_types::{
        AudioSessionActivationOptions, AudioSessionAnchoringStrategy, AudioSessionIOType,
        AudioSessionInterruptionOptions, AudioSessionInterruptionType,
        AudioSessionMicrophoneInjectionMode, AudioSessionPromptStyle,
        AudioSessionRenderingMode, AudioSessionRouteChangeReason,
        AudioSessionSetActiveOptions, AudioSessionSilenceSecondaryAudioHintType,
        AudioSessionSoundStageSize, AudioSessionSpatialExperience, AudioStereoOrientation,
    };
    pub use crate::settings::{
        AudioBitRateStrategy, AudioContentSource, AudioDynamicRangeControlConfiguration,
        AudioQuality, AudioSettingsConstants,
    };
    pub use crate::simple_player::{AudioSimplePlayer, AudioSimplePlayerDelegate};
    pub use crate::time::{AudioTime, AudioTimeInfo};
    pub use crate::types::{
        Audio3DMixingPointSourceInHeadMode, Audio3DMixingRenderingAlgorithm,
        Audio3DMixingSourceMode, Audio3DVector, Audio3DVectorOrientation, AudioChannelCount,
        AudioEnvironmentOutputType, AudioFrameCount, AudioFramePosition, AudioNodeBus,
        AudioPacketCount,
    };
    pub use crate::sink_node::{AudioSinkNode, AudioSinkRenderContext};
    pub use crate::source_node::{AudioSourceNode, AudioSourceRenderContext};
    pub use crate::unit::{
        AUAudioUnitHandle, AudioComponentDescription, AudioComponentInstantiationOptions,
        AudioUnit, AudioUnitMetadata,
    };
    pub use crate::unit_component::{
        AudioUnitComponentConstants, AudioUnitComponentInfo, AudioUnitComponentManager,
    };
    pub use crate::unit_delay::AudioUnitDelay;
    pub use crate::unit_distortion::{AudioUnitDistortion, AudioUnitDistortionPreset};
    pub use crate::unit_effect::{AudioUnitEffect, AudioUnitHandle, AudioUnitInfo};
    pub use crate::unit_generator::AudioUnitGenerator;
    pub use crate::unit_eq::{AudioEQBandInfo, AudioEQBandParams, AudioUnitEQ};
    pub use crate::unit_midi_instrument::{
        AudioUnitMIDIInstrument, AudioUnitMIDIInstrumentHandle, MidiEventPacket, MidiProtocol,
    };
    pub use crate::unit_reverb::{AudioUnitReverb, AudioUnitReverbPreset};
    pub use crate::unit_sampler::AudioUnitSampler;
    pub use crate::unit_time_effect::AudioUnitTimeEffect;
    pub use crate::unit_time_pitch::AudioUnitTimePitch;
    pub use crate::unit_varispeed::AudioUnitVarispeed;
}
