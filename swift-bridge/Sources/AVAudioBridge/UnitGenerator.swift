import AudioToolbox
import AVFoundation
import Foundation

@_cdecl("av_audio_unit_generator_create_with_component_description")
public func av_audio_unit_generator_create_with_component_description(
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
    let unit = AVAudioUnitGenerator(audioComponentDescription: description)
    return Unmanaged.passRetained(unit).toOpaque()
}

@_cdecl("av_audio_unit_generator_release")
public func av_audio_unit_generator_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnitGenerator>.fromOpaque(ptr).release()
}
