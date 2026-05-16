//! Shared audio-node traits.

use core::ffi::c_void;

/// Implemented by all node types that can be attached to an [`crate::AudioEngine`].
pub trait AudioNodeHandle {
    /// Returns a borrowed, non-owning pointer to the underlying `AVAudioNode`.
    #[doc(hidden)]
    fn as_node_ptr(&self) -> *mut c_void;
}
