//! [`AudioSession`] — lightweight session queries.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use crate::error::AVAudioError;

/// Lightweight access to `AVAudioSession`-like session data.
pub struct AudioSession;

fn unavailable() -> AVAudioError {
    AVAudioError::Unsupported("AVAudioSession is unavailable on macOS".into())
}

impl AudioSession {
    /// Returns the `AVAudioSession` sample rate; `AVAudioSession` is unavailable on macOS.
    pub fn sample_rate() -> Result<f64, AVAudioError> {
        Err(unavailable())
    }

    /// Returns the `AVAudioSession` output volume; `AVAudioSession` is unavailable on macOS.
    pub fn output_volume() -> Result<f32, AVAudioError> {
        Err(unavailable())
    }

    /// Returns whether other audio is currently playing; `AVAudioSession` is unavailable on macOS.
    pub fn is_other_audio_playing() -> Result<bool, AVAudioError> {
        Err(unavailable())
    }
}
