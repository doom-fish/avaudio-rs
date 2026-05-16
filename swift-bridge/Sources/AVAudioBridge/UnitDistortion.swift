import AVFoundation
import Foundation

@_cdecl("av_audio_unit_distortion_create")
public func av_audio_unit_distortion_create() -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioUnitDistortion()).toOpaque()
}

@_cdecl("av_audio_unit_distortion_release")
public func av_audio_unit_distortion_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnitDistortion>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_unit_distortion_get_pre_gain")
public func av_audio_unit_distortion_get_pre_gain(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitDistortion>.fromOpaque(ptr).takeUnretainedValue().preGain
}

@_cdecl("av_audio_unit_distortion_set_pre_gain")
public func av_audio_unit_distortion_set_pre_gain(_ ptr: UnsafeMutableRawPointer, _ preGain: Float) {
    let node = Unmanaged<AVAudioUnitDistortion>.fromOpaque(ptr).takeUnretainedValue()
    node.preGain = preGain
}

@_cdecl("av_audio_unit_distortion_get_wet_dry_mix")
public func av_audio_unit_distortion_get_wet_dry_mix(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitDistortion>.fromOpaque(ptr).takeUnretainedValue().wetDryMix
}

@_cdecl("av_audio_unit_distortion_set_wet_dry_mix")
public func av_audio_unit_distortion_set_wet_dry_mix(_ ptr: UnsafeMutableRawPointer, _ wetDryMix: Float) {
    let node = Unmanaged<AVAudioUnitDistortion>.fromOpaque(ptr).takeUnretainedValue()
    node.wetDryMix = wetDryMix
}

@_cdecl("av_audio_unit_distortion_load_factory_preset")
public func av_audio_unit_distortion_load_factory_preset(_ ptr: UnsafeMutableRawPointer, _ preset: Int32) {
    let node = Unmanaged<AVAudioUnitDistortion>.fromOpaque(ptr).takeUnretainedValue()
    let mappedPreset = AVAudioUnitDistortionPreset(rawValue: Int(preset)) ?? .drumsBitBrush
    node.loadFactoryPreset(mappedPreset)
}
