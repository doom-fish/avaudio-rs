import AVFoundation
import Foundation

@_cdecl("av_audio_unit_reverb_create")
public func av_audio_unit_reverb_create() -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioUnitReverb()).toOpaque()
}

@_cdecl("av_audio_unit_reverb_release")
public func av_audio_unit_reverb_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnitReverb>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_unit_reverb_get_wet_dry_mix")
public func av_audio_unit_reverb_get_wet_dry_mix(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitReverb>.fromOpaque(ptr).takeUnretainedValue().wetDryMix
}

@_cdecl("av_audio_unit_reverb_set_wet_dry_mix")
public func av_audio_unit_reverb_set_wet_dry_mix(_ ptr: UnsafeMutableRawPointer, _ mix: Float) {
    let node = Unmanaged<AVAudioUnitReverb>.fromOpaque(ptr).takeUnretainedValue()
    node.wetDryMix = mix
}

@_cdecl("av_audio_unit_reverb_load_factory_preset")
public func av_audio_unit_reverb_load_factory_preset(_ ptr: UnsafeMutableRawPointer, _ preset: Int32) {
    let reverb = Unmanaged<AVAudioUnitReverb>.fromOpaque(ptr).takeUnretainedValue()
    let mappedPreset = AVAudioUnitReverbPreset(rawValue: Int(preset)) ?? .smallRoom
    reverb.loadFactoryPreset(mappedPreset)
}
