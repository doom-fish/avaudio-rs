import AVFoundation
import Foundation

struct AudioSequencerInfoPayload: Codable {
    let trackCount: Int
    let currentPositionInSeconds: Double
    let currentPositionInBeats: Double
    let isPlaying: Bool
    let rate: Float
    let hasTempoTrack: Bool
}

struct AudioSequencerInfoDictionaryKeysPayload: Codable {
    let album: String
    let approximateDurationInSeconds: String
    let artist: String
    let channelLayout: String
    let comments: String
    let composer: String
    let copyright: String
    let encodingApplication: String
    let genre: String
    let isrc: String
    let keySignature: String
    let lyricist: String
    let nominalBitRate: String
    let recordedDate: String
    let sourceBitDepth: String
    let sourceEncoder: String
    let subTitle: String
    let tempo: String
    let timeSignature: String
    let title: String
    let trackNumber: String
    let year: String
}

final class SequencerUserCallbackBox {
    let callback: AVASequencerUserCallback?
    let userData: UnsafeMutableRawPointer?
    let dropUserData: AVADropCallback?

    init(
        callback: AVASequencerUserCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) {
        self.callback = callback
        self.userData = userData
        self.dropUserData = dropUserData
    }

    func fire(track: AVMusicTrack, eventData: Data, beat: Double) {
        let trackPtr = Unmanaged.passUnretained(track).toOpaque()
        eventData.withUnsafeBytes { rawBuffer in
            let baseAddress = rawBuffer.bindMemory(to: UInt8.self).baseAddress
            callback?(userData, trackPtr, baseAddress, eventData.count, beat)
        }
    }

    deinit {
        if let userData, let dropUserData {
            dropUserData(userData)
        }
    }
}

private func sequencerInfoPayload(_ sequencer: AVAudioSequencer) -> AudioSequencerInfoPayload {
    AudioSequencerInfoPayload(
        trackCount: sequencer.tracks.count,
        currentPositionInSeconds: sequencer.currentPositionInSeconds,
        currentPositionInBeats: sequencer.currentPositionInBeats,
        isPlaying: sequencer.isPlaying,
        rate: sequencer.rate,
        hasTempoTrack: true
    )
}

@available(macOS 13.0, *)
private func infoDictionaryKeysPayload() -> AudioSequencerInfoDictionaryKeysPayload {
    AudioSequencerInfoDictionaryKeysPayload(
        album: AVAudioSequencer.InfoDictionaryKey.album.rawValue,
        approximateDurationInSeconds: AVAudioSequencer.InfoDictionaryKey.approximateDurationInSeconds.rawValue,
        artist: AVAudioSequencer.InfoDictionaryKey.artist.rawValue,
        channelLayout: AVAudioSequencer.InfoDictionaryKey.channelLayout.rawValue,
        comments: AVAudioSequencer.InfoDictionaryKey.comments.rawValue,
        composer: AVAudioSequencer.InfoDictionaryKey.composer.rawValue,
        copyright: AVAudioSequencer.InfoDictionaryKey.copyright.rawValue,
        encodingApplication: AVAudioSequencer.InfoDictionaryKey.encodingApplication.rawValue,
        genre: AVAudioSequencer.InfoDictionaryKey.genre.rawValue,
        isrc: AVAudioSequencer.InfoDictionaryKey.ISRC.rawValue,
        keySignature: AVAudioSequencer.InfoDictionaryKey.keySignature.rawValue,
        lyricist: AVAudioSequencer.InfoDictionaryKey.lyricist.rawValue,
        nominalBitRate: AVAudioSequencer.InfoDictionaryKey.nominalBitRate.rawValue,
        recordedDate: AVAudioSequencer.InfoDictionaryKey.recordedDate.rawValue,
        sourceBitDepth: AVAudioSequencer.InfoDictionaryKey.sourceBitDepth.rawValue,
        sourceEncoder: AVAudioSequencer.InfoDictionaryKey.sourceEncoder.rawValue,
        subTitle: AVAudioSequencer.InfoDictionaryKey.subTitle.rawValue,
        tempo: AVAudioSequencer.InfoDictionaryKey.tempo.rawValue,
        timeSignature: AVAudioSequencer.InfoDictionaryKey.timeSignature.rawValue,
        title: AVAudioSequencer.InfoDictionaryKey.title.rawValue,
        trackNumber: AVAudioSequencer.InfoDictionaryKey.trackNumber.rawValue,
        year: AVAudioSequencer.InfoDictionaryKey.year.rawValue
    )
}

@available(macOS 13.0, *)
private func sequencerUserInfoJSONObject(_ sequencer: AVAudioSequencer) -> [String: Any] {
    Dictionary(uniqueKeysWithValues: sequencer.userInfo.map { key, value in
        (key, jsonCompatibleValue(value))
    })
}

private func sequencerData(_ sequencer: AVAudioSequencer, resolution: Int) throws -> Data {
    var error: NSError?
    let data = sequencer.data(withSMPTEResolution: resolution, error: &error)
    if let error {
        throw error
    }
    return data
}

private func hostTime(forBeats beats: Double, sequencer: AVAudioSequencer) throws -> UInt64 {
    var error: NSError?
    let value = sequencer.hostTime(forBeats: beats, error: &error)
    if let error {
        throw error
    }
    return value
}

private func beats(forHostTime hostTime: UInt64, sequencer: AVAudioSequencer) throws -> Double {
    var error: NSError?
    let value = sequencer.beats(forHostTime: hostTime, error: &error)
    if let error {
        throw error
    }
    return value
}

@_cdecl("av_audio_sequencer_create")
public func av_audio_sequencer_create(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioSequencer()).toOpaque()
}

@_cdecl("av_audio_sequencer_create_with_engine")
public func av_audio_sequencer_create_with_engine(
    _ enginePtr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    return Unmanaged.passRetained(AVAudioSequencer(audioEngine: engine)).toOpaque()
}

@_cdecl("av_audio_sequencer_release")
public func av_audio_sequencer_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioSequencer>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_sequencer_info_json")
public func av_audio_sequencer_info_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(sequencerInfoPayload(sequencer)))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_sequencer_info_dictionary_keys_json")
public func av_audio_sequencer_info_dictionary_keys_json(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("AVAudioSequencer info dictionary keys require macOS 13.0")
        return nil
    }
    do {
        return ffiString(try avaEncodeJSON(infoDictionaryKeysPayload()))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_sequencer_user_info_json")
public func av_audio_sequencer_user_info_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("AVAudioSequencer userInfo requires macOS 13.0")
        return nil
    }
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    do {
        return ffiString(try jsonString(fromJSONObject: sequencerUserInfoJSONObject(sequencer)))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_sequencer_load_from_url")
public func av_audio_sequencer_load_from_url(
    _ ptr: UnsafeMutableRawPointer,
    _ pathPtr: UnsafePointer<CChar>,
    _ optionsRaw: UInt,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    let path = String(cString: pathPtr)
    do {
        try sequencer.load(
            from: URL(fileURLWithPath: path),
            options: AVMusicSequenceLoadOptions(rawValue: optionsRaw)
        )
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}

@_cdecl("av_audio_sequencer_load_from_data")
public func av_audio_sequencer_load_from_data(
    _ ptr: UnsafeMutableRawPointer,
    _ bytes: UnsafePointer<UInt8>?,
    _ count: Int,
    _ optionsRaw: UInt,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    let data = bytes.map { Data(bytes: $0, count: count) } ?? Data()
    do {
        try sequencer.load(from: data, options: AVMusicSequenceLoadOptions(rawValue: optionsRaw))
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}

@_cdecl("av_audio_sequencer_write_to_url")
public func av_audio_sequencer_write_to_url(
    _ ptr: UnsafeMutableRawPointer,
    _ pathPtr: UnsafePointer<CChar>,
    _ resolution: Int,
    _ replaceExisting: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    let path = String(cString: pathPtr)
    do {
        try sequencer.write(
            to: URL(fileURLWithPath: path),
            smpteResolution: resolution,
            replaceExisting: replaceExisting
        )
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}

@_cdecl("av_audio_sequencer_copy_data")
public func av_audio_sequencer_copy_data(
    _ ptr: UnsafeMutableRawPointer,
    _ resolution: Int,
    _ outLength: UnsafeMutablePointer<Int>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<UInt8>? {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    do {
        let data = try sequencerData(sequencer, resolution: resolution)
        let count = data.count
        let allocationSize = max(count, 1)
        guard let raw = malloc(allocationSize) else {
            throw BridgeError.message("failed to allocate sequence data buffer")
        }
        let buffer = raw.assumingMemoryBound(to: UInt8.self)
        if count > 0 {
            data.copyBytes(to: buffer, count: count)
        }
        outLength?.pointee = count
        return buffer
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_sequencer_copy_track_at_index")
public func av_audio_sequencer_copy_track_at_index(
    _ ptr: UnsafeMutableRawPointer,
    _ index: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    guard index >= 0, index < sequencer.tracks.count else {
        outError?.pointee = ffiString("track index out of bounds")
        return nil
    }
    return Unmanaged.passRetained(sequencer.tracks[index]).toOpaque()
}

@_cdecl("av_audio_sequencer_copy_tempo_track")
public func av_audio_sequencer_copy_tempo_track(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    return Unmanaged.passRetained(sequencer.tempoTrack).toOpaque()
}

@_cdecl("av_audio_sequencer_create_and_append_track")
public func av_audio_sequencer_create_and_append_track(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("AVAudioSequencer track creation requires macOS 13.0")
        return nil
    }
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    return Unmanaged.passRetained(sequencer.createAndAppendTrack()).toOpaque()
}

@_cdecl("av_audio_sequencer_remove_track")
public func av_audio_sequencer_remove_track(
    _ ptr: UnsafeMutableRawPointer,
    _ trackPtr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("AVAudioSequencer track removal requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    let track = Unmanaged<AVMusicTrack>.fromOpaque(trackPtr).takeUnretainedValue()
    sequencer.removeTrack(track)
    return AVA_OK
}

@_cdecl("av_audio_sequencer_reverse_events")
public func av_audio_sequencer_reverse_events(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    guard #available(macOS 13.0, *) else {
        outError?.pointee = ffiString("reverseEvents requires macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    sequencer.reverseEvents()
    return AVA_OK
}

@_cdecl("av_audio_sequencer_set_current_position_in_seconds")
public func av_audio_sequencer_set_current_position_in_seconds(
    _ ptr: UnsafeMutableRawPointer,
    _ seconds: Double
) {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    sequencer.currentPositionInSeconds = seconds
}

@_cdecl("av_audio_sequencer_set_current_position_in_beats")
public func av_audio_sequencer_set_current_position_in_beats(
    _ ptr: UnsafeMutableRawPointer,
    _ beats: Double
) {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    sequencer.currentPositionInBeats = beats
}

@_cdecl("av_audio_sequencer_set_rate")
public func av_audio_sequencer_set_rate(_ ptr: UnsafeMutableRawPointer, _ rate: Float) {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    sequencer.rate = rate
}

@_cdecl("av_audio_sequencer_seconds_for_beats")
public func av_audio_sequencer_seconds_for_beats(_ ptr: UnsafeMutableRawPointer, _ beats: Double) -> Double {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    return sequencer.seconds(forBeats: beats)
}

@_cdecl("av_audio_sequencer_beats_for_seconds")
public func av_audio_sequencer_beats_for_seconds(_ ptr: UnsafeMutableRawPointer, _ seconds: Double) -> Double {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    return sequencer.beats(forSeconds: seconds)
}

@_cdecl("av_audio_sequencer_host_time_for_beats")
public func av_audio_sequencer_host_time_for_beats(
    _ ptr: UnsafeMutableRawPointer,
    _ beats: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UInt64 {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    do {
        return try hostTime(forBeats: beats, sequencer: sequencer)
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return 0
    }
}

@_cdecl("av_audio_sequencer_beats_for_host_time")
public func av_audio_sequencer_beats_for_host_time(
    _ ptr: UnsafeMutableRawPointer,
    _ hostTime: UInt64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Double {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    do {
        return try beats(forHostTime: hostTime, sequencer: sequencer)
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return 0
    }
}

@_cdecl("av_audio_sequencer_prepare_to_play")
public func av_audio_sequencer_prepare_to_play(_ ptr: UnsafeMutableRawPointer) {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    sequencer.prepareToPlay()
}

@_cdecl("av_audio_sequencer_start")
public func av_audio_sequencer_start(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    do {
        try sequencer.start()
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}

@_cdecl("av_audio_sequencer_stop")
public func av_audio_sequencer_stop(_ ptr: UnsafeMutableRawPointer) {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    sequencer.stop()
}

@_cdecl("av_audio_sequencer_set_user_callback")
public func av_audio_sequencer_set_user_callback(
    _ ptr: UnsafeMutableRawPointer,
    _ callback: AVASequencerUserCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    guard #available(macOS 13.0, *) else {
        if callback == nil {
            return AVA_OK
        }
        outError?.pointee = ffiString("AVAudioSequencer user callbacks require macOS 13.0")
        return AVA_OPERATION_FAILED
    }
    guard let callback else {
        sequencer.setUserCallback(nil)
        return AVA_OK
    }
    let box = SequencerUserCallbackBox(callback: callback, userData: userData, dropUserData: dropUserData)
    sequencer.setUserCallback { track, eventData, beat in
        box.fire(track: track, eventData: eventData as Data, beat: beat)
    }
    return AVA_OK
}
