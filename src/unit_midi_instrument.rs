//! Generic [`AVAudioUnitMIDIInstrument`] support.

#![allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::semicolon_if_nothing_returned
)]

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;

use serde::{Deserialize, Serialize};

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::node::AudioNodeHandle;
use crate::unit::AudioComponentDescription;
use crate::unit_effect::AudioUnitHandle;

/// CoreMIDI UMP protocol IDs accepted by `send_midi_event_list`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MidiProtocol {
    /// MIDI 1.0 Universal MIDI Packets.
    Midi1_0 = 1,
    /// MIDI 2.0 Universal MIDI Packets.
    Midi2_0 = 2,
}

/// One packet in a CoreMIDI `MIDIEventList`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MidiEventPacket {
    /// Host-time timestamp for the packet. `0` means “now”.
    pub time_stamp: u64,
    /// Raw UMP words for the packet.
    pub words: Vec<u32>,
}

/// Shared MIDI-event sending helpers for `AVAudioUnitMIDIInstrument` subclasses.
pub trait AudioUnitMIDIInstrumentHandle: AudioUnitHandle {
    /// Returns the underlying `AVAudioUnitMIDIInstrument` pointer.
    #[doc(hidden)]
    fn as_midi_instrument_ptr(&self) -> *mut c_void;

    /// Sends a MIDI note-on event.
    fn start_note(&self, note: u8, velocity: u8, channel: u8) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_start_note(
                self.as_midi_instrument_ptr(),
                note,
                velocity,
                channel,
            )
        };
    }

    /// Sends a MIDI note-off event.
    fn stop_note(&self, note: u8, channel: u8) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_stop_note(
                self.as_midi_instrument_ptr(),
                note,
                channel,
            )
        };
    }

    /// Sends a standard MIDI controller event.
    fn send_controller(&self, controller: u8, value: u8, channel: u8) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_send_controller(
                self.as_midi_instrument_ptr(),
                controller,
                value,
                channel,
            )
        };
    }

    /// Sends a pitch-bend event.
    fn send_pitch_bend(&self, pitch_bend: u16, channel: u8) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_send_pitch_bend(
                self.as_midi_instrument_ptr(),
                pitch_bend,
                channel,
            )
        };
    }

    /// Sends a channel-pressure event.
    fn send_pressure(&self, pressure: u8, channel: u8) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_send_pressure(
                self.as_midi_instrument_ptr(),
                pressure,
                channel,
            )
        };
    }

    /// Sends a polyphonic key-pressure event.
    fn send_pressure_for_key(&self, key: u8, value: u8, channel: u8) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_send_pressure_for_key(
                self.as_midi_instrument_ptr(),
                key,
                value,
                channel,
            )
        };
    }

    /// Sends a program-change event using the previously-selected bank.
    fn send_program_change(&self, program: u8, channel: u8) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_send_program_change(
                self.as_midi_instrument_ptr(),
                program,
                channel,
            )
        };
    }

    /// Sends a program change along with explicit bank-select values.
    fn send_program_change_with_bank(&self, program: u8, bank_msb: u8, bank_lsb: u8, channel: u8) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_send_program_change_bank(
                self.as_midi_instrument_ptr(),
                program,
                bank_msb,
                bank_lsb,
                channel,
            )
        };
    }

    /// Sends a MIDI event with two data bytes.
    fn send_midi_event(&self, midi_status: u8, data1: u8, data2: u8) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_send_midi_event(
                self.as_midi_instrument_ptr(),
                midi_status,
                data1,
                data2,
            )
        };
    }

    /// Sends a MIDI event with a single data byte.
    fn send_midi_event_one_data_byte(&self, midi_status: u8, data1: u8) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_send_midi_event_one_data_byte(
                self.as_midi_instrument_ptr(),
                midi_status,
                data1,
            )
        };
    }

    /// Sends a MIDI SysEx event.
    fn send_midi_sysex_event(&self, bytes: &[u8]) {
        unsafe {
            ffi::av_audio_unit_midi_instrument_send_midi_sysex_event(
                self.as_midi_instrument_ptr(),
                bytes.as_ptr(),
                bytes.len(),
            )
        };
    }

    /// Sends a CoreMIDI `MIDIEventList` described by UMP packets.
    fn send_midi_event_list(
        &self,
        protocol: MidiProtocol,
        packets: &[MidiEventPacket],
    ) -> Result<(), AVAudioError> {
        let json = serde_json::to_string(packets).map_err(|error| {
            AVAudioError::InvalidArgument(format!("failed to encode MIDI event list: {error}"))
        })?;
        let json = CString::new(json).map_err(|error| {
            AVAudioError::InvalidArgument(format!(
                "MIDI event list JSON contains NUL byte: {error}"
            ))
        })?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_unit_midi_instrument_send_midi_event_list_json(
                self.as_midi_instrument_ptr(),
                protocol as i32,
                json.as_ptr(),
                &mut err,
            )
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }
}

/// Wraps a generic `AVAudioUnitMIDIInstrument`.
pub struct AudioUnitMIDIInstrument {
    pub(crate) ptr: *mut c_void,
}

unsafe impl Send for AudioUnitMIDIInstrument {}

impl Drop for AudioUnitMIDIInstrument {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_unit_midi_instrument_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl AudioNodeHandle for AudioUnitMIDIInstrument {
    fn as_node_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitHandle for AudioUnitMIDIInstrument {
    fn as_audio_unit_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitMIDIInstrumentHandle for AudioUnitMIDIInstrument {
    fn as_midi_instrument_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl AudioUnitMIDIInstrument {
    /// Instantiates a generic `AVAudioUnitMIDIInstrument` for the provided component description.
    pub fn new_with_component_description(
        description: AudioComponentDescription,
    ) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr = unsafe {
            ffi::av_audio_unit_midi_instrument_create_with_component_description(
                description.component_type,
                description.component_subtype,
                description.component_manufacturer,
                description.component_flags,
                description.component_flags_mask,
                &mut err,
            )
        };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::OPERATION_FAILED, err) });
        }
        Ok(Self { ptr })
    }
}
