import AVFoundation
import CoreMIDI
import Foundation

struct AudioBeatRangePayload: Codable {
    let start: Double
    let length: Double
}

struct MusicTrackInfoPayload: Codable {
    let destinationMIDIEndpoint: UInt64
    let loopRange: AudioBeatRangePayload
    let loopingEnabled: Bool
    let numberOfLoops: Int64
    let offsetTime: Double
    let muted: Bool
    let soloed: Bool
    let lengthInBeats: Double
    let lengthInSeconds: Double
    let timeResolution: Int
    let usesAutomatedParameters: Bool
    let hasDestinationAudioUnit: Bool
}

private struct MusicEventPayload: Codable {
    let kind: String
    let channel: UInt32?
    let key: UInt32?
    let velocity: UInt32?
    let duration: Double?
    let messageType: Int64?
    let value: UInt32?
    let pressure: UInt32?
    let programNumber: UInt32?
    let data: [UInt8]?
    let sizeInBytes: UInt32?
    let metaType: Int64?
    let midiNote: Float?
    let velocityFloat: Float?
    let instrumentID: UInt32?
    let groupID: UInt32?
    let parameterID: UInt32?
    let scope: UInt32?
    let element: UInt32?
    let valueFloat: Float?
    let tempo: Double?
    let presetDictionaryJSON: String?
}

private func encodeBeatRange(_ range: AVBeatRange) -> AudioBeatRangePayload {
    AudioBeatRangePayload(start: range.start, length: range.length)
}

private func decodeBeatRange(_ payload: AudioBeatRangePayload) -> AVBeatRange {
    AVMakeBeatRange(payload.start, payload.length)
}

func jsonCompatibleValue(_ value: Any) -> Any {
    if let dictionary = value as? [String: Any] {
        return dictionary.mapValues(jsonCompatibleValue)
    }
    if let array = value as? [Any] {
        return array.map(jsonCompatibleValue)
    }
    if let string = value as? String {
        return string
    }
    if let number = value as? NSNumber {
        return number
    }
    if let bool = value as? Bool {
        return bool
    }
    if value is NSNull {
        return NSNull()
    }
    if let url = value as? URL {
        return url.absoluteString
    }
    if let date = value as? Date {
        return ISO8601DateFormatter().string(from: date)
    }
    if let data = value as? Data {
        return [UInt8](data)
    }
    return String(describing: value)
}

func jsonString(fromJSONObject object: Any) throws -> String {
    let compatible = jsonCompatibleValue(object)
    guard JSONSerialization.isValidJSONObject(compatible) else {
        throw BridgeError.message("object is not JSON-serializable")
    }
    let data = try JSONSerialization.data(withJSONObject: compatible, options: [.sortedKeys])
    guard let string = String(data: data, encoding: .utf8) else {
        throw BridgeError.message("failed to encode JSON string")
    }
    return string
}

func foundationObject(fromJSONString json: String) throws -> Any {
    try JSONSerialization.jsonObject(with: Data(json.utf8))
}

@available(macOS 13.0, *)
private func decodeMusicEvent(_ payload: MusicEventPayload) throws -> AVMusicEvent {
    switch payload.kind {
    case "midiNote":
        return AVMIDINoteEvent(
            channel: payload.channel ?? 0,
            key: payload.key ?? 0,
            velocity: payload.velocity ?? 0,
            duration: payload.duration ?? 0
        )
    case "midiControlChange":
        guard let raw = payload.messageType,
              let messageType = AVMIDIControlChangeEvent.MessageType(rawValue: Int(raw))
        else {
            throw BridgeError.message("invalid MIDI control-change message type")
        }
        return AVMIDIControlChangeEvent(
            channel: payload.channel ?? 0,
            messageType: messageType,
            value: payload.value ?? 0
        )
    case "midiPolyPressure":
        return AVMIDIPolyPressureEvent(
            channel: payload.channel ?? 0,
            key: payload.key ?? 0,
            pressure: payload.pressure ?? 0
        )
    case "midiProgramChange":
        return AVMIDIProgramChangeEvent(
            channel: payload.channel ?? 0,
            programNumber: payload.programNumber ?? 0
        )
    case "midiChannelPressure":
        return AVMIDIChannelPressureEvent(
            channel: payload.channel ?? 0,
            pressure: payload.pressure ?? 0
        )
    case "midiPitchBend":
        return AVMIDIPitchBendEvent(channel: payload.channel ?? 0, value: payload.value ?? 0)
    case "midiSysex":
        return AVMIDISysexEvent(data: Data(payload.data ?? []))
    case "midiMeta":
        guard let raw = payload.metaType,
              let metaType = AVMIDIMetaEvent.EventType(rawValue: Int(raw))
        else {
            throw BridgeError.message("invalid MIDI meta-event type")
        }
        return AVMIDIMetaEvent(type: metaType, data: Data(payload.data ?? []))
    case "musicUser":
        return AVMusicUserEvent(data: Data(payload.data ?? []))
    case "extendedNoteOn":
        return AVExtendedNoteOnEvent(
            midiNote: payload.midiNote ?? 0,
            velocity: payload.velocityFloat ?? 0,
            instrumentID: payload.instrumentID ?? AVExtendedNoteOnEvent.defaultInstrument,
            groupID: payload.groupID ?? 0,
            duration: payload.duration ?? 0
        )
    case "parameter":
        return AVParameterEvent(
            parameterID: payload.parameterID ?? 0,
            scope: payload.scope ?? 0,
            element: payload.element ?? 0,
            value: payload.valueFloat ?? 0
        )
    case "auPreset":
        let presetObject = try foundationObject(fromJSONString: payload.presetDictionaryJSON ?? "{}")
        guard let dictionary = presetObject as? [AnyHashable: Any] else {
            throw BridgeError.message("AVAUPresetEvent preset dictionary must be a JSON object")
        }
        return AVAUPresetEvent(
            scope: payload.scope ?? 0,
            element: payload.element ?? 0,
            dictionary: dictionary
        )
    case "extendedTempo":
        return AVExtendedTempoEvent(tempo: payload.tempo ?? 120)
    default:
        throw BridgeError.message("unsupported music event kind: \(payload.kind)")
    }
}

@available(macOS 13.0, *)
private func encodeMusicEvent(_ event: AVMusicEvent) throws -> MusicEventPayload {
    if let event = event as? AVMIDINoteEvent {
        return MusicEventPayload(
            kind: "midiNote",
            channel: event.channel,
            key: event.key,
            velocity: event.velocity,
            duration: event.duration,
            messageType: nil,
            value: nil,
            pressure: nil,
            programNumber: nil,
            data: nil,
            sizeInBytes: nil,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVMIDIControlChangeEvent {
        return MusicEventPayload(
            kind: "midiControlChange",
            channel: event.channel,
            key: nil,
            velocity: nil,
            duration: nil,
            messageType: Int64(event.messageType.rawValue),
            value: event.value,
            pressure: nil,
            programNumber: nil,
            data: nil,
            sizeInBytes: nil,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVMIDIPolyPressureEvent {
        return MusicEventPayload(
            kind: "midiPolyPressure",
            channel: event.channel,
            key: event.key,
            velocity: nil,
            duration: nil,
            messageType: nil,
            value: nil,
            pressure: event.pressure,
            programNumber: nil,
            data: nil,
            sizeInBytes: nil,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVMIDIProgramChangeEvent {
        return MusicEventPayload(
            kind: "midiProgramChange",
            channel: event.channel,
            key: nil,
            velocity: nil,
            duration: nil,
            messageType: nil,
            value: nil,
            pressure: nil,
            programNumber: event.programNumber,
            data: nil,
            sizeInBytes: nil,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVMIDIChannelPressureEvent {
        return MusicEventPayload(
            kind: "midiChannelPressure",
            channel: event.channel,
            key: nil,
            velocity: nil,
            duration: nil,
            messageType: nil,
            value: nil,
            pressure: event.pressure,
            programNumber: nil,
            data: nil,
            sizeInBytes: nil,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVMIDIPitchBendEvent {
        return MusicEventPayload(
            kind: "midiPitchBend",
            channel: event.channel,
            key: nil,
            velocity: nil,
            duration: nil,
            messageType: nil,
            value: event.value,
            pressure: nil,
            programNumber: nil,
            data: nil,
            sizeInBytes: nil,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVMIDISysexEvent {
        return MusicEventPayload(
            kind: "midiSysex",
            channel: nil,
            key: nil,
            velocity: nil,
            duration: nil,
            messageType: nil,
            value: nil,
            pressure: nil,
            programNumber: nil,
            data: nil,
            sizeInBytes: event.sizeInBytes,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVMIDIMetaEvent {
        return MusicEventPayload(
            kind: "midiMeta",
            channel: nil,
            key: nil,
            velocity: nil,
            duration: nil,
            messageType: nil,
            value: nil,
            pressure: nil,
            programNumber: nil,
            data: nil,
            sizeInBytes: nil,
            metaType: Int64(event.type.rawValue),
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVMusicUserEvent {
        return MusicEventPayload(
            kind: "musicUser",
            channel: nil,
            key: nil,
            velocity: nil,
            duration: nil,
            messageType: nil,
            value: nil,
            pressure: nil,
            programNumber: nil,
            data: nil,
            sizeInBytes: event.sizeInBytes,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVExtendedNoteOnEvent {
        return MusicEventPayload(
            kind: "extendedNoteOn",
            channel: nil,
            key: nil,
            velocity: nil,
            duration: event.duration,
            messageType: nil,
            value: nil,
            pressure: nil,
            programNumber: nil,
            data: nil,
            sizeInBytes: nil,
            metaType: nil,
            midiNote: event.midiNote,
            velocityFloat: event.velocity,
            instrumentID: event.instrumentID,
            groupID: event.groupID,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVParameterEvent {
        return MusicEventPayload(
            kind: "parameter",
            channel: nil,
            key: nil,
            velocity: nil,
            duration: nil,
            messageType: nil,
            value: nil,
            pressure: nil,
            programNumber: nil,
            data: nil,
            sizeInBytes: nil,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: event.parameterID,
            scope: event.scope,
            element: event.element,
            valueFloat: event.value,
            tempo: nil,
            presetDictionaryJSON: nil
        )
    }
    if let event = event as? AVAUPresetEvent {
        return MusicEventPayload(
            kind: "auPreset",
            channel: nil,
            key: nil,
            velocity: nil,
            duration: nil,
            messageType: nil,
            value: nil,
            pressure: nil,
            programNumber: nil,
            data: nil,
            sizeInBytes: nil,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: event.scope,
            element: event.element,
            valueFloat: nil,
            tempo: nil,
            presetDictionaryJSON: try jsonString(fromJSONObject: event.presetDictionary)
        )
    }
    if let event = event as? AVExtendedTempoEvent {
        return MusicEventPayload(
            kind: "extendedTempo",
            channel: nil,
            key: nil,
            velocity: nil,
            duration: nil,
            messageType: nil,
            value: nil,
            pressure: nil,
            programNumber: nil,
            data: nil,
            sizeInBytes: nil,
            metaType: nil,
            midiNote: nil,
            velocityFloat: nil,
            instrumentID: nil,
            groupID: nil,
            parameterID: nil,
            scope: nil,
            element: nil,
            valueFloat: nil,
            tempo: event.tempo,
            presetDictionaryJSON: nil
        )
    }
    throw BridgeError.message("unsupported AVMusicEvent subclass: \(type(of: event))")
}

private func musicTrackInfoPayload(_ track: AVMusicTrack) -> MusicTrackInfoPayload {
    let hasDestinationAudioUnit = track.destinationAudioUnit != nil
    return MusicTrackInfoPayload(
        destinationMIDIEndpoint: 0,
        loopRange: encodeBeatRange(track.loopRange),
        loopingEnabled: track.isLoopingEnabled,
        numberOfLoops: Int64(track.numberOfLoops),
        offsetTime: track.offsetTime,
        muted: track.isMuted,
        soloed: track.isSoloed,
        lengthInBeats: track.lengthInBeats,
        lengthInSeconds: track.lengthInSeconds,
        timeResolution: Int(track.timeResolution),
        usesAutomatedParameters: false,
        hasDestinationAudioUnit: hasDestinationAudioUnit
    )
}

@_cdecl("av_music_track_release")
public func av_music_track_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVMusicTrack>.fromOpaque(ptr).release()
}

@_cdecl("av_music_track_info_json")
public func av_music_track_info_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(musicTrackInfoPayload(track)))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_music_track_copy_destination_audio_unit")
public func av_music_track_copy_destination_audio_unit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    guard let unit = track.destinationAudioUnit else { return nil }
    return Unmanaged.passRetained(unit).toOpaque()
}

@_cdecl("av_music_track_set_destination_audio_unit")
public func av_music_track_set_destination_audio_unit(
    _ ptr: UnsafeMutableRawPointer,
    _ unitPtr: UnsafeMutableRawPointer?
) {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    let unit = unitPtr.map { Unmanaged<AVAudioUnit>.fromOpaque($0).takeUnretainedValue() }
    track.destinationAudioUnit = unit
}

@_cdecl("av_music_track_set_destination_midi_endpoint")
public func av_music_track_set_destination_midi_endpoint(_ ptr: UnsafeMutableRawPointer, _ endpoint: UInt64) {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.destinationMIDIEndpoint = MIDIEndpointRef(endpoint)
}

@_cdecl("av_music_track_set_loop_range")
public func av_music_track_set_loop_range(_ ptr: UnsafeMutableRawPointer, _ start: Double, _ length: Double) {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.loopRange = AVMakeBeatRange(start, length)
}

@_cdecl("av_music_track_set_looping_enabled")
public func av_music_track_set_looping_enabled(_ ptr: UnsafeMutableRawPointer, _ enabled: Bool) {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.isLoopingEnabled = enabled
}

@_cdecl("av_music_track_set_number_of_loops")
public func av_music_track_set_number_of_loops(_ ptr: UnsafeMutableRawPointer, _ count: Int64) {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.numberOfLoops = Int(count)
}

@_cdecl("av_music_track_set_offset_time")
public func av_music_track_set_offset_time(_ ptr: UnsafeMutableRawPointer, _ offsetTime: Double) {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.offsetTime = offsetTime
}

@_cdecl("av_music_track_set_muted")
public func av_music_track_set_muted(_ ptr: UnsafeMutableRawPointer, _ muted: Bool) {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.isMuted = muted
}

@_cdecl("av_music_track_set_soloed")
public func av_music_track_set_soloed(_ ptr: UnsafeMutableRawPointer, _ soloed: Bool) {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.isSoloed = soloed
}

@_cdecl("av_music_track_set_length_in_beats")
public func av_music_track_set_length_in_beats(_ ptr: UnsafeMutableRawPointer, _ length: Double) {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.lengthInBeats = length
}

@_cdecl("av_music_track_set_length_in_seconds")
public func av_music_track_set_length_in_seconds(_ ptr: UnsafeMutableRawPointer, _ length: Double) {
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.lengthInSeconds = length
}

@_cdecl("av_music_track_set_uses_automated_parameters")
public func av_music_track_set_uses_automated_parameters(
    _ ptr: UnsafeMutableRawPointer,
    _ usesAutomatedParameters: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("usesAutomatedParameters requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.usesAutomatedParameters = usesAutomatedParameters
    return AVA_OK
}

@_cdecl("av_music_track_add_event_json")
public func av_music_track_add_event_json(
    _ ptr: UnsafeMutableRawPointer,
    _ jsonPtr: UnsafePointer<CChar>,
    _ beat: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("track event editing requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    do {
        let payload = try JSONDecoder().decode(MusicEventPayload.self, from: Data(String(cString: jsonPtr).utf8))
        let event = try decodeMusicEvent(payload)
        track.addEvent(event, at: beat)
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}

@_cdecl("av_music_track_move_events_in_range")
public func av_music_track_move_events_in_range(
    _ ptr: UnsafeMutableRawPointer,
    _ start: Double,
    _ length: Double,
    _ beatAmount: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("track event editing requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.moveEvents(in: AVMakeBeatRange(start, length), by: beatAmount)
    return AVA_OK
}

@_cdecl("av_music_track_clear_events_in_range")
public func av_music_track_clear_events_in_range(
    _ ptr: UnsafeMutableRawPointer,
    _ start: Double,
    _ length: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("track event editing requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.clearEvents(in: AVMakeBeatRange(start, length))
    return AVA_OK
}

@_cdecl("av_music_track_cut_events_in_range")
public func av_music_track_cut_events_in_range(
    _ ptr: UnsafeMutableRawPointer,
    _ start: Double,
    _ length: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("track event editing requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    track.cutEvents(in: AVMakeBeatRange(start, length))
    return AVA_OK
}

@_cdecl("av_music_track_copy_events_in_range")
public func av_music_track_copy_events_in_range(
    _ ptr: UnsafeMutableRawPointer,
    _ start: Double,
    _ length: Double,
    _ sourceTrackPtr: UnsafeMutableRawPointer,
    _ insertAt: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("track event editing requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    let sourceTrack = Unmanaged<AVMusicTrack>.fromOpaque(sourceTrackPtr).takeUnretainedValue()
    track.copyEvents(in: AVMakeBeatRange(start, length), from: sourceTrack, insertAt: insertAt)
    return AVA_OK
}

@_cdecl("av_music_track_copy_and_merge_events_in_range")
public func av_music_track_copy_and_merge_events_in_range(
    _ ptr: UnsafeMutableRawPointer,
    _ start: Double,
    _ length: Double,
    _ sourceTrackPtr: UnsafeMutableRawPointer,
    _ mergeAt: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("track event editing requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    let sourceTrack = Unmanaged<AVMusicTrack>.fromOpaque(sourceTrackPtr).takeUnretainedValue()
    track.copyAndMergeEvents(in: AVMakeBeatRange(start, length), from: sourceTrack, mergeAt: mergeAt)
    return AVA_OK
}

@_cdecl("av_music_track_enumerate_events")
public func av_music_track_enumerate_events(
    _ ptr: UnsafeMutableRawPointer,
    _ start: Double,
    _ length: Double,
    _ callback: AVAMusicTrackEnumerationCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("track event enumeration requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    guard let callback else {
        outError?.pointee = ffiString("event enumeration callback was null")
        return AVA_INVALID_ARGUMENT
    }
    let track = Unmanaged<AVMusicTrack>.fromOpaque(ptr).takeUnretainedValue()
    var callbackStatus: Int32 = AVA_OK
    var callbackErrorMessage: UnsafeMutablePointer<CChar>?
    track.enumerateEvents(in: AVMakeBeatRange(start, length)) { event, timeStamp, removeEvent in
        guard callbackStatus == AVA_OK else { return }
        do {
            let payload = try avaEncodeJSON(encodeMusicEvent(event))
            var newBeat = timeStamp.pointee
            var remove = false
            let status = payload.withCString { jsonPtr in
                callback(userData, jsonPtr, timeStamp.pointee, &newBeat, &remove, &callbackErrorMessage)
            }
            callbackStatus = status
            if status == AVA_OK {
                timeStamp.pointee = newBeat
                removeEvent.pointee = ObjCBool(remove)
            }
        } catch {
            callbackStatus = AVA_CALLBACK_ERROR
            callbackErrorMessage = ffiString(error.localizedDescription)
        }
    }
    if callbackStatus != AVA_OK {
        if let callbackErrorMessage {
            outError?.pointee = callbackErrorMessage
        } else if outError?.pointee == nil {
            outError?.pointee = ffiString("music track enumeration callback failed")
        }
    }
    return callbackStatus
}
