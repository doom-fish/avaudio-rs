//! [`AudioSession`] — lightweight session queries.

#![allow(clippy::must_use_candidate, clippy::module_name_repetitions)]

use crate::ffi;

/// Lightweight access to `AVAudioSession`-like session data.
pub struct AudioSession;

impl AudioSession {
    /// Returns the current hardware sample rate, or a best-effort macOS stub value.
    pub fn sample_rate() -> f64 {
        unsafe { ffi::av_audio_session_get_sample_rate() }
    }

    /// Returns the current output volume, or a macOS stub value.
    pub fn output_volume() -> f32 {
        unsafe { ffi::av_audio_session_get_output_volume() }
    }

    /// Returns whether other audio is currently playing.
    pub fn is_other_audio_playing() -> bool {
        unsafe { ffi::av_audio_session_is_other_audio_playing() }
    }
}
