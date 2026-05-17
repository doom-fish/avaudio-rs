import AudioToolbox
import AVFoundation
import Foundation

@_cdecl("av_audio_unit_time_effect_create_with_component_description")
public func av_audio_unit_time_effect_create_with_component_description(
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
    let unit = AVAudioUnitTimeEffect(audioComponentDescription: description)
    return Unmanaged.passRetained(unit).toOpaque()
}

@_cdecl("av_audio_unit_time_effect_release")
public func av_audio_unit_time_effect_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnitTimeEffect>.fromOpaque(ptr).release()
}
