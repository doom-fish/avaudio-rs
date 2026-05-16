import AVFoundation
import Foundation

@_cdecl("av_audio_unit_delay_create")
public func av_audio_unit_delay_create() -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioUnitDelay()).toOpaque()
}

@_cdecl("av_audio_unit_delay_release")
public func av_audio_unit_delay_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnitDelay>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_unit_delay_get_delay_time")
public func av_audio_unit_delay_get_delay_time(_ ptr: UnsafeMutableRawPointer) -> Double {
    Unmanaged<AVAudioUnitDelay>.fromOpaque(ptr).takeUnretainedValue().delayTime
}

@_cdecl("av_audio_unit_delay_set_delay_time")
public func av_audio_unit_delay_set_delay_time(_ ptr: UnsafeMutableRawPointer, _ delayTime: Double) {
    let node = Unmanaged<AVAudioUnitDelay>.fromOpaque(ptr).takeUnretainedValue()
    node.delayTime = delayTime
}

@_cdecl("av_audio_unit_delay_get_feedback")
public func av_audio_unit_delay_get_feedback(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitDelay>.fromOpaque(ptr).takeUnretainedValue().feedback
}

@_cdecl("av_audio_unit_delay_set_feedback")
public func av_audio_unit_delay_set_feedback(_ ptr: UnsafeMutableRawPointer, _ feedback: Float) {
    let node = Unmanaged<AVAudioUnitDelay>.fromOpaque(ptr).takeUnretainedValue()
    node.feedback = feedback
}

@_cdecl("av_audio_unit_delay_get_low_pass_cutoff")
public func av_audio_unit_delay_get_low_pass_cutoff(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitDelay>.fromOpaque(ptr).takeUnretainedValue().lowPassCutoff
}

@_cdecl("av_audio_unit_delay_set_low_pass_cutoff")
public func av_audio_unit_delay_set_low_pass_cutoff(_ ptr: UnsafeMutableRawPointer, _ lowPassCutoff: Float) {
    let node = Unmanaged<AVAudioUnitDelay>.fromOpaque(ptr).takeUnretainedValue()
    node.lowPassCutoff = lowPassCutoff
}

@_cdecl("av_audio_unit_delay_get_wet_dry_mix")
public func av_audio_unit_delay_get_wet_dry_mix(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitDelay>.fromOpaque(ptr).takeUnretainedValue().wetDryMix
}

@_cdecl("av_audio_unit_delay_set_wet_dry_mix")
public func av_audio_unit_delay_set_wet_dry_mix(_ ptr: UnsafeMutableRawPointer, _ wetDryMix: Float) {
    let node = Unmanaged<AVAudioUnitDelay>.fromOpaque(ptr).takeUnretainedValue()
    node.wetDryMix = wetDryMix
}
