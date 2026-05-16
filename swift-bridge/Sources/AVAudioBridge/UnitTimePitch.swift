import AVFoundation
import Foundation

@_cdecl("av_audio_unit_time_pitch_create")
public func av_audio_unit_time_pitch_create() -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioUnitTimePitch()).toOpaque()
}

@_cdecl("av_audio_unit_time_pitch_release")
public func av_audio_unit_time_pitch_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnitTimePitch>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_unit_time_pitch_get_pitch")
public func av_audio_unit_time_pitch_get_pitch(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitTimePitch>.fromOpaque(ptr).takeUnretainedValue().pitch
}

@_cdecl("av_audio_unit_time_pitch_set_pitch")
public func av_audio_unit_time_pitch_set_pitch(_ ptr: UnsafeMutableRawPointer, _ pitch: Float) {
    let node = Unmanaged<AVAudioUnitTimePitch>.fromOpaque(ptr).takeUnretainedValue()
    node.pitch = pitch
}

@_cdecl("av_audio_unit_time_pitch_get_rate")
public func av_audio_unit_time_pitch_get_rate(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitTimePitch>.fromOpaque(ptr).takeUnretainedValue().rate
}

@_cdecl("av_audio_unit_time_pitch_set_rate")
public func av_audio_unit_time_pitch_set_rate(_ ptr: UnsafeMutableRawPointer, _ rate: Float) {
    let node = Unmanaged<AVAudioUnitTimePitch>.fromOpaque(ptr).takeUnretainedValue()
    node.rate = rate
}

@_cdecl("av_audio_unit_time_pitch_get_overlap")
public func av_audio_unit_time_pitch_get_overlap(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitTimePitch>.fromOpaque(ptr).takeUnretainedValue().overlap
}

@_cdecl("av_audio_unit_time_pitch_set_overlap")
public func av_audio_unit_time_pitch_set_overlap(_ ptr: UnsafeMutableRawPointer, _ overlap: Float) {
    let node = Unmanaged<AVAudioUnitTimePitch>.fromOpaque(ptr).takeUnretainedValue()
    node.overlap = overlap
}
