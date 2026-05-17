//! Public `AVAudioSessionTypes.h` enums and option sets.

#![allow(
    clippy::enum_variant_names,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

const fn fourcc(bytes: [u8; 4]) -> i64 {
    ((bytes[0] as i64) << 24)
        | ((bytes[1] as i64) << 16)
        | ((bytes[2] as i64) << 8)
        | (bytes[3] as i64)
}

macro_rules! option_flags {
    ($name:ident { $($const_name:ident = $value:expr,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        #[repr(transparent)]
        pub struct $name(pub u64);

        impl $name {
            $(pub const $const_name: Self = Self($value);)*

            pub const fn bits(self) -> u64 {
                self.0
            }

            pub const fn from_bits(bits: u64) -> Self {
                Self(bits)
            }

            pub const fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }
        }

        impl BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }
    };
}

option_flags!(AudioSessionActivationOptions {
    NONE = 0,
});

option_flags!(AudioSessionInterruptionOptions {
    SHOULD_RESUME = 1,
});

option_flags!(AudioSessionSetActiveOptions {
    NOTIFY_OTHERS_ON_DEACTIVATION = 1,
});

/// Mirrors `AVAudioSessionIOType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum AudioSessionIOType {
    NotSpecified = 0,
    Aggregated = 1,
}

/// Mirrors `AVAudioSessionInterruptionType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum AudioSessionInterruptionType {
    Ended = 0,
    Began = 1,
}

/// Mirrors `AVAudioSessionPromptStyle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioSessionPromptStyle {
    None = fourcc(*b"none"),
    Short = fourcc(*b"shrt"),
    Normal = fourcc(*b"nrml"),
}

/// Mirrors `AVAudioSessionRouteChangeReason`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum AudioSessionRouteChangeReason {
    Unknown = 0,
    NewDeviceAvailable = 1,
    OldDeviceUnavailable = 2,
    CategoryChange = 3,
    Override = 4,
    WakeFromSleep = 6,
    NoSuitableRouteForCategory = 7,
    RouteConfigurationChange = 8,
}

/// Mirrors `AVAudioStereoOrientation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioStereoOrientation {
    None = 0,
    Portrait = 1,
    PortraitUpsideDown = 2,
    LandscapeRight = 3,
    LandscapeLeft = 4,
}

/// Mirrors `AVAudioSessionRenderingMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioSessionRenderingMode {
    NotApplicable = 0,
    MonoStereo = 1,
    Surround = 2,
    SpatialAudio = 3,
    DolbyAudio = 4,
    DolbyAtmos = 5,
}

/// Mirrors `AVAudioSessionMicrophoneInjectionMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioSessionMicrophoneInjectionMode {
    None = 0,
    SpokenAudio = 1,
}

/// Mirrors `AVAudioSessionSoundStageSize`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioSessionSoundStageSize {
    Automatic = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

/// Mirrors `AVAudioSessionAnchoringStrategy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioSessionAnchoringStrategy {
    Automatic = 0,
    Scene = 1,
    Front = 2,
}

/// Mirrors `AVAudioSessionSpatialExperience`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum AudioSessionSpatialExperience {
    HeadTracked = 0,
    Fixed = 1,
    Bypassed = 2,
}

/// Mirrors `AVAudioSessionSilenceSecondaryAudioHintType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum AudioSessionSilenceSecondaryAudioHintType {
    End = 0,
    Begin = 1,
}
