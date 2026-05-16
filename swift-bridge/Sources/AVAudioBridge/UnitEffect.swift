import AVFoundation
import Foundation

struct AudioUnitInfoPayload: Codable {
    let bypass: Bool
}

private func av_audio_unit_bypass_target(_ ptr: UnsafeMutableRawPointer) -> AVAudioUnit {
    Unmanaged<AVAudioUnit>.fromOpaque(ptr).takeUnretainedValue()
}

private func av_audio_unit_bypass_value(_ unit: AVAudioUnit) -> Bool {
    if let effect = unit as? AVAudioUnitEffect {
        return effect.bypass
    }
    if let timeEffect = unit as? AVAudioUnitTimeEffect {
        return timeEffect.bypass
    }
    return false
}

private func av_audio_unit_set_bypass_value(_ unit: AVAudioUnit, _ bypass: Bool) {
    if let effect = unit as? AVAudioUnitEffect {
        effect.bypass = bypass
        return
    }
    if let timeEffect = unit as? AVAudioUnitTimeEffect {
        timeEffect.bypass = bypass
    }
}

@_cdecl("av_audio_unit_info_json")
public func av_audio_unit_info_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let unit = av_audio_unit_bypass_target(ptr)
    do {
        return ffiString(try avaEncodeJSON(AudioUnitInfoPayload(bypass: av_audio_unit_bypass_value(unit))))
    } catch {
        outErrorMessage?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_unit_get_bypass")
public func av_audio_unit_get_bypass(_ ptr: UnsafeMutableRawPointer) -> Bool {
    av_audio_unit_bypass_value(av_audio_unit_bypass_target(ptr))
}

@_cdecl("av_audio_unit_set_bypass")
public func av_audio_unit_set_bypass(_ ptr: UnsafeMutableRawPointer, _ bypass: Bool) {
    av_audio_unit_set_bypass_value(av_audio_unit_bypass_target(ptr), bypass)
}
