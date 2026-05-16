import AVFoundation
import Foundation

@_cdecl("av_audio_unit_varispeed_create")
public func av_audio_unit_varispeed_create() -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioUnitVarispeed()).toOpaque()
}

@_cdecl("av_audio_unit_varispeed_release")
public func av_audio_unit_varispeed_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnitVarispeed>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_unit_varispeed_get_rate")
public func av_audio_unit_varispeed_get_rate(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitVarispeed>.fromOpaque(ptr).takeUnretainedValue().rate
}

@_cdecl("av_audio_unit_varispeed_set_rate")
public func av_audio_unit_varispeed_set_rate(_ ptr: UnsafeMutableRawPointer, _ rate: Float) {
    let node = Unmanaged<AVAudioUnitVarispeed>.fromOpaque(ptr).takeUnretainedValue()
    node.rate = rate
}
