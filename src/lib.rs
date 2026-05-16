#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod audio_file;
mod buffer;
mod converter;
mod engine;
mod environment_node;
mod error;
mod ffi;
mod file;
mod format;
mod input_node;
mod input_output_node;
mod mixer;
mod node;
mod output_node;
mod pcm_buffer;
mod player;
mod recorder;
mod session;
mod simple_player;
mod unit_effect;
mod unit_eq;
mod unit_reverb;
mod unit_time_pitch;
mod util;

pub use audio_file::{AudioFile, AudioFileInfo};
pub use buffer::{AudioBufferHandle, AudioBufferInfo};
pub use converter::{AudioConverter, AudioConverterInfo};
pub use engine::{AudioEngine, AudioEngineInfo};
pub use environment_node::{
    AudioDistanceAttenuation, AudioEnvironmentNode, AudioListenerOrientation, AudioListenerPosition,
};
pub use error::AVAudioError;
pub use format::{AudioCommonFormat, AudioFormat, AudioFormatInfo};
pub use input_node::AudioInputNode;
pub use mixer::AudioMixerNode;
pub use node::AudioNodeHandle;
pub use output_node::AudioOutputNode;
pub use pcm_buffer::{PCMBuffer, PCMBufferInfo};
pub use player::{AudioPlayerNode, AudioPlayerNodeInfo};
pub use recorder::AudioRecorder;
pub use session::AudioSession;
pub use simple_player::AudioSimplePlayer;
pub use unit_effect::{AudioUnitHandle, AudioUnitInfo};
pub use unit_eq::{AudioEQBandInfo, AudioEQBandParams, AudioUnitEQ};
pub use unit_reverb::{AudioUnitReverb, AudioUnitReverbPreset};
pub use unit_time_pitch::AudioUnitTimePitch;

/// Common imports.
pub mod prelude {
    pub use crate::audio_file::{AudioFile, AudioFileInfo};
    pub use crate::buffer::{AudioBufferHandle, AudioBufferInfo};
    pub use crate::converter::{AudioConverter, AudioConverterInfo};
    pub use crate::engine::{AudioEngine, AudioEngineInfo};
    pub use crate::environment_node::{
        AudioDistanceAttenuation, AudioEnvironmentNode, AudioListenerOrientation,
        AudioListenerPosition,
    };
    pub use crate::error::AVAudioError;
    pub use crate::format::{AudioCommonFormat, AudioFormat, AudioFormatInfo};
    pub use crate::input_node::AudioInputNode;
    pub use crate::mixer::AudioMixerNode;
    pub use crate::node::AudioNodeHandle;
    pub use crate::output_node::AudioOutputNode;
    pub use crate::pcm_buffer::{PCMBuffer, PCMBufferInfo};
    pub use crate::player::{AudioPlayerNode, AudioPlayerNodeInfo};
    pub use crate::recorder::AudioRecorder;
    pub use crate::session::AudioSession;
    pub use crate::simple_player::AudioSimplePlayer;
    pub use crate::unit_effect::{AudioUnitHandle, AudioUnitInfo};
    pub use crate::unit_eq::{AudioEQBandInfo, AudioEQBandParams, AudioUnitEQ};
    pub use crate::unit_reverb::{AudioUnitReverb, AudioUnitReverbPreset};
    pub use crate::unit_time_pitch::AudioUnitTimePitch;
}
