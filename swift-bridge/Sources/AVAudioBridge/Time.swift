import AVFoundation
import Foundation

struct AudioTimeInfoPayload: Codable {
    let hostTimeValid: Bool
    let hostTime: UInt64
    let sampleTimeValid: Bool
    let sampleTime: Int64
    let sampleRate: Double
}

func avaEncodeAudioTimeInfo(_ time: AVAudioTime) -> AudioTimeInfoPayload {
    AudioTimeInfoPayload(
        hostTimeValid: time.isHostTimeValid,
        hostTime: time.hostTime,
        sampleTimeValid: time.isSampleTimeValid,
        sampleTime: time.sampleTime,
        sampleRate: time.sampleRate
    )
}

@_cdecl("av_audio_time_create_with_host_time")
public func av_audio_time_create_with_host_time(_ hostTime: UInt64) -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioTime(hostTime: hostTime)).toOpaque()
}

@_cdecl("av_audio_time_create_with_sample_time")
public func av_audio_time_create_with_sample_time(
    _ sampleTime: Int64,
    _ sampleRate: Double
) -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioTime(sampleTime: sampleTime, atRate: sampleRate)).toOpaque()
}

@_cdecl("av_audio_time_create_with_host_and_sample_time")
public func av_audio_time_create_with_host_and_sample_time(
    _ hostTime: UInt64,
    _ sampleTime: Int64,
    _ sampleRate: Double
) -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioTime(hostTime: hostTime, sampleTime: sampleTime, atRate: sampleRate)).toOpaque()
}

@_cdecl("av_audio_time_release")
public func av_audio_time_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioTime>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_time_info_json")
public func av_audio_time_info_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let time = Unmanaged<AVAudioTime>.fromOpaque(ptr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(avaEncodeAudioTimeInfo(time)))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_time_extrapolate_from_anchor")
public func av_audio_time_extrapolate_from_anchor(
    _ ptr: UnsafeMutableRawPointer,
    _ anchorPtr: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    let time = Unmanaged<AVAudioTime>.fromOpaque(ptr).takeUnretainedValue()
    let anchor = Unmanaged<AVAudioTime>.fromOpaque(anchorPtr).takeUnretainedValue()
    guard let extrapolated = time.extrapolateTime(fromAnchor: anchor) else {
        return nil
    }
    return Unmanaged.passRetained(extrapolated).toOpaque()
}

@_cdecl("av_audio_time_host_time_for_seconds")
public func av_audio_time_host_time_for_seconds(_ seconds: Double) -> UInt64 {
    AVAudioTime.hostTime(forSeconds: seconds)
}

@_cdecl("av_audio_time_seconds_for_host_time")
public func av_audio_time_seconds_for_host_time(_ hostTime: UInt64) -> Double {
    AVAudioTime.seconds(forHostTime: hostTime)
}
