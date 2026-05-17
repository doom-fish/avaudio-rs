//! Core `AVAudioTypes.h` / `AVAudioMixing.h` value-models.

#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

use serde::{Deserialize, Serialize};

/// Mirrors `AVAudioChannelCount`.
pub type AudioChannelCount = u32;
/// Mirrors `AVAudioFrameCount`.
pub type AudioFrameCount = u32;
/// Mirrors `AVAudioFramePosition`.
pub type AudioFramePosition = i64;
/// Mirrors `AVAudioPacketCount`.
pub type AudioPacketCount = u32;
/// Mirrors `AVAudioNodeBus`.
pub type AudioNodeBus = usize;

/// Mirrors `AVAudio3DVector`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Audio3DVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Audio3DVector {
    /// Creates a new 3D vector.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Mirrors `AVAudio3DVectorOrientation`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Audio3DVectorOrientation {
    pub forward: Audio3DVector,
    pub up: Audio3DVector,
}

impl Audio3DVectorOrientation {
    /// Creates a new vector orientation.
    pub const fn new(forward: Audio3DVector, up: Audio3DVector) -> Self {
        Self { forward, up }
    }
}

/// Mirrors `AVAudio3DMixingRenderingAlgorithm`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum Audio3DMixingRenderingAlgorithm {
    EqualPowerPanning = 0,
    SphericalHead = 1,
    Hrtf = 2,
    SoundField = 3,
    StereoPassThrough = 5,
    HrtfHighQuality = 6,
    Auto = 7,
}

/// Mirrors `AVAudio3DMixingSourceMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum Audio3DMixingSourceMode {
    SpatializeIfMono = 0,
    Bypass = 1,
    PointSource = 2,
    AmbienceBed = 3,
}

/// Mirrors `AVAudio3DMixingPointSourceInHeadMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum Audio3DMixingPointSourceInHeadMode {
    Mono = 0,
    Bypass = 1,
}

/// Mirrors `AVAudioEnvironmentOutputType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioEnvironmentOutputType {
    Auto = 0,
    Headphones = 1,
    BuiltInSpeakers = 2,
    ExternalSpeakers = 3,
}
