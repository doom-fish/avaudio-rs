import AVFoundation
import Foundation

struct AudioSequencerInfoPayload: Codable {
    let trackCount: Int
    let currentPositionInSeconds: Double
    let currentPositionInBeats: Double
    let isPlaying: Bool
    let rate: Float
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
    let payload = AudioSequencerInfoPayload(
        trackCount: sequencer.tracks.count,
        currentPositionInSeconds: sequencer.currentPositionInSeconds,
        currentPositionInBeats: sequencer.currentPositionInBeats,
        isPlaying: sequencer.isPlaying,
        rate: sequencer.rate
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_sequencer_load_from_url")
public func av_audio_sequencer_load_from_url(
    _ ptr: UnsafeMutableRawPointer,
    _ pathPtr: UnsafePointer<CChar>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let sequencer = Unmanaged<AVAudioSequencer>.fromOpaque(ptr).takeUnretainedValue()
    let path = String(cString: pathPtr)
    do {
        try sequencer.load(from: URL(fileURLWithPath: path), options: [])
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
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
    sequencer.setUserCallback({ track, eventData, beat in
        box.fire(track: track, eventData: eventData as Data, beat: beat)
    })
    return AVA_OK
}
