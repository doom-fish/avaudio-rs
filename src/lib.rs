#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod engine;
mod error;
mod file;
mod ffi;
mod format;
mod player;

pub use engine::{AudioEngine, AudioEngineInfo};
pub use error::AVAudioError;
pub use file::{AudioFile, AudioFileInfo, PCMBuffer, PCMBufferInfo};
pub use format::{AudioCommonFormat, AudioFormat, AudioFormatInfo};
pub use player::{AudioPlayerNode, AudioPlayerNodeInfo};

/// Common imports.
pub mod prelude {
    pub use crate::engine::{AudioEngine, AudioEngineInfo};
    pub use crate::error::AVAudioError;
    pub use crate::file::{AudioFile, AudioFileInfo, PCMBuffer, PCMBufferInfo};
    pub use crate::format::{AudioCommonFormat, AudioFormat, AudioFormatInfo};
    pub use crate::player::{AudioPlayerNode, AudioPlayerNodeInfo};
}
