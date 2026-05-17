import AudioToolbox
import AVFoundation
import CoreMIDI
import Foundation

private struct MIDIPacketPayload: Decodable {
    let timeStamp: UInt64
    let words: [UInt32]
}

private func avAudioMIDIInstrumentTarget(_ ptr: UnsafeMutableRawPointer) -> AVAudioUnitMIDIInstrument {
    Unmanaged<AVAudioUnitMIDIInstrument>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("av_audio_unit_midi_instrument_create_with_component_description")
public func av_audio_unit_midi_instrument_create_with_component_description(
    _ componentType: UInt32,
    _ componentSubtype: UInt32,
    _ componentManufacturer: UInt32,
    _ componentFlags: UInt32,
    _ componentFlagsMask: UInt32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let description = avaAudioComponentDescription(
        componentType,
        componentSubtype,
        componentManufacturer,
        componentFlags,
        componentFlagsMask
    )
    let unit = AVAudioUnitMIDIInstrument(audioComponentDescription: description)
    return Unmanaged.passRetained(unit).toOpaque()
}

@_cdecl("av_audio_unit_midi_instrument_release")
public func av_audio_unit_midi_instrument_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnitMIDIInstrument>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_unit_midi_instrument_start_note")
public func av_audio_unit_midi_instrument_start_note(
    _ ptr: UnsafeMutableRawPointer,
    _ note: UInt8,
    _ velocity: UInt8,
    _ channel: UInt8
) {
    avAudioMIDIInstrumentTarget(ptr).startNote(note, withVelocity: velocity, onChannel: channel)
}

@_cdecl("av_audio_unit_midi_instrument_stop_note")
public func av_audio_unit_midi_instrument_stop_note(
    _ ptr: UnsafeMutableRawPointer,
    _ note: UInt8,
    _ channel: UInt8
) {
    avAudioMIDIInstrumentTarget(ptr).stopNote(note, onChannel: channel)
}

@_cdecl("av_audio_unit_midi_instrument_send_controller")
public func av_audio_unit_midi_instrument_send_controller(
    _ ptr: UnsafeMutableRawPointer,
    _ controller: UInt8,
    _ value: UInt8,
    _ channel: UInt8
) {
    avAudioMIDIInstrumentTarget(ptr).sendController(controller, withValue: value, onChannel: channel)
}

@_cdecl("av_audio_unit_midi_instrument_send_pitch_bend")
public func av_audio_unit_midi_instrument_send_pitch_bend(
    _ ptr: UnsafeMutableRawPointer,
    _ pitchBend: UInt16,
    _ channel: UInt8
) {
    avAudioMIDIInstrumentTarget(ptr).sendPitchBend(pitchBend, onChannel: channel)
}

@_cdecl("av_audio_unit_midi_instrument_send_pressure")
public func av_audio_unit_midi_instrument_send_pressure(
    _ ptr: UnsafeMutableRawPointer,
    _ pressure: UInt8,
    _ channel: UInt8
) {
    avAudioMIDIInstrumentTarget(ptr).sendPressure(pressure, onChannel: channel)
}

@_cdecl("av_audio_unit_midi_instrument_send_pressure_for_key")
public func av_audio_unit_midi_instrument_send_pressure_for_key(
    _ ptr: UnsafeMutableRawPointer,
    _ key: UInt8,
    _ value: UInt8,
    _ channel: UInt8
) {
    avAudioMIDIInstrumentTarget(ptr).sendPressure(forKey: key, withValue: value, onChannel: channel)
}

@_cdecl("av_audio_unit_midi_instrument_send_program_change")
public func av_audio_unit_midi_instrument_send_program_change(
    _ ptr: UnsafeMutableRawPointer,
    _ program: UInt8,
    _ channel: UInt8
) {
    avAudioMIDIInstrumentTarget(ptr).sendProgramChange(program, onChannel: channel)
}

@_cdecl("av_audio_unit_midi_instrument_send_program_change_bank")
public func av_audio_unit_midi_instrument_send_program_change_bank(
    _ ptr: UnsafeMutableRawPointer,
    _ program: UInt8,
    _ bankMSB: UInt8,
    _ bankLSB: UInt8,
    _ channel: UInt8
) {
    avAudioMIDIInstrumentTarget(ptr).sendProgramChange(program, bankMSB: bankMSB, bankLSB: bankLSB, onChannel: channel)
}

@_cdecl("av_audio_unit_midi_instrument_send_midi_event")
public func av_audio_unit_midi_instrument_send_midi_event(
    _ ptr: UnsafeMutableRawPointer,
    _ midiStatus: UInt8,
    _ data1: UInt8,
    _ data2: UInt8
) {
    avAudioMIDIInstrumentTarget(ptr).sendMIDIEvent(midiStatus, data1: data1, data2: data2)
}

@_cdecl("av_audio_unit_midi_instrument_send_midi_event_one_data_byte")
public func av_audio_unit_midi_instrument_send_midi_event_one_data_byte(
    _ ptr: UnsafeMutableRawPointer,
    _ midiStatus: UInt8,
    _ data1: UInt8
) {
    avAudioMIDIInstrumentTarget(ptr).sendMIDIEvent(midiStatus, data1: data1)
}

@_cdecl("av_audio_unit_midi_instrument_send_midi_sysex_event")
public func av_audio_unit_midi_instrument_send_midi_sysex_event(
    _ ptr: UnsafeMutableRawPointer,
    _ bytesPtr: UnsafePointer<UInt8>?,
    _ bytesLen: Int
) {
    let data = bytesPtr.map { Data(bytes: $0, count: bytesLen) } ?? Data()
    avAudioMIDIInstrumentTarget(ptr).sendMIDISysExEvent(data)
}

@_cdecl("av_audio_unit_midi_instrument_send_midi_event_list_json")
public func av_audio_unit_midi_instrument_send_midi_event_list_json(
    _ ptr: UnsafeMutableRawPointer,
    _ protocolRaw: Int32,
    _ jsonPtr: UnsafePointer<CChar>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("sendMIDIEventList requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    guard let protocolID = MIDIProtocolID(rawValue: protocolRaw) else {
        outError?.pointee = ffiString("invalid MIDI protocol id")
        return AVA_INVALID_ARGUMENT
    }
    let json = String(cString: jsonPtr)
    do {
        let packets = try JSONDecoder().decode([MIDIPacketPayload].self, from: Data(json.utf8))
        let bufferSize = 65_536
        let rawBuffer = UnsafeMutableRawPointer.allocate(
            byteCount: bufferSize,
            alignment: MemoryLayout<MIDIEventList>.alignment
        )
        defer { rawBuffer.deallocate() }
        let eventList = rawBuffer.bindMemory(to: MIDIEventList.self, capacity: 1)
        var packet = MIDIEventListInit(eventList, protocolID)
        for payload in packets {
            guard !payload.words.isEmpty else {
                throw BridgeError.message("MIDIEventList packets must contain at least one word")
            }
            let nextPacket: UnsafeMutablePointer<MIDIEventPacket>? = payload.words.withUnsafeBufferPointer { wordsBuffer in
                MIDIEventListAdd(
                    eventList,
                    bufferSize,
                    packet,
                    MIDITimeStamp(payload.timeStamp),
                    wordsBuffer.count,
                    wordsBuffer.baseAddress!
                )
            }
            guard let nextPacket else {
                throw BridgeError.message("failed to build MIDIEventList")
            }
            packet = nextPacket
        }
        avAudioMIDIInstrumentTarget(ptr).send(eventList)
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}
