import AVFoundation
import Foundation

private struct Audio3DVectorPayload: Codable {
    let x: Float
    let y: Float
    let z: Float
}

private func avaMixingObject(_ ptr: UnsafeMutableRawPointer) -> NSObject {
    Unmanaged<NSObject>.fromOpaque(ptr).takeUnretainedValue()
}

private func avaRequireMixing(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> AVAudioMixing? {
    guard let mixing = avaMixingObject(ptr) as? AVAudioMixing else {
        outError?.pointee = ffiString("object does not conform to AVAudioMixing")
        return nil
    }
    return mixing
}

private func avaRequireStereoMixing(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> AVAudioStereoMixing? {
    guard let mixing = avaMixingObject(ptr) as? AVAudioStereoMixing else {
        outError?.pointee = ffiString("object does not conform to AVAudioStereoMixing")
        return nil
    }
    return mixing
}

private func avaRequire3DMixing(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> AVAudio3DMixing? {
    guard let mixing = avaMixingObject(ptr) as? AVAudio3DMixing else {
        outError?.pointee = ffiString("object does not conform to AVAudio3DMixing")
        return nil
    }
    return mixing
}

@_cdecl("av_audio_mixing_get_volume")
public func av_audio_mixing_get_volume(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Float {
    guard let mixing = avaRequireMixing(ptr, outError) else { return 0 }
    return mixing.volume
}

@_cdecl("av_audio_mixing_set_volume")
public func av_audio_mixing_set_volume(
    _ ptr: UnsafeMutableRawPointer,
    _ volume: Float,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let mixing = avaRequireMixing(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    mixing.volume = volume
    return AVA_OK
}

@_cdecl("av_audio_mixing_destination_for_mixer")
public func av_audio_mixing_destination_for_mixer(
    _ ptr: UnsafeMutableRawPointer,
    _ mixerPtr: UnsafeMutableRawPointer,
    _ bus: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let mixing = avaRequireMixing(ptr, outError) else { return nil }
    let mixer = Unmanaged<AVAudioNode>.fromOpaque(mixerPtr).takeUnretainedValue()
    guard let destination = mixing.destination(forMixer: mixer, bus: AVAudioNodeBus(bus)) else {
        return nil
    }
    return Unmanaged.passRetained(destination).toOpaque()
}

@_cdecl("av_audio_mixing_destination_release")
public func av_audio_mixing_destination_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioMixingDestination>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_mixing_destination_copy_connection_point")
public func av_audio_mixing_destination_copy_connection_point(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let destination = Unmanaged<AVAudioMixingDestination>.fromOpaque(ptr).takeUnretainedValue()
    return Unmanaged.passRetained(destination.connectionPoint).toOpaque()
}

@_cdecl("av_audio_stereo_mixing_get_pan")
public func av_audio_stereo_mixing_get_pan(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Float {
    guard let mixing = avaRequireStereoMixing(ptr, outError) else { return 0 }
    return mixing.pan
}

@_cdecl("av_audio_stereo_mixing_set_pan")
public func av_audio_stereo_mixing_set_pan(
    _ ptr: UnsafeMutableRawPointer,
    _ pan: Float,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let mixing = avaRequireStereoMixing(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    mixing.pan = pan
    return AVA_OK
}

@_cdecl("av_audio_3d_mixing_get_rendering_algorithm")
public func av_audio_3d_mixing_get_rendering_algorithm(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int64 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return 0 }
    return Int64(mixing.renderingAlgorithm.rawValue)
}

@_cdecl("av_audio_3d_mixing_set_rendering_algorithm")
public func av_audio_3d_mixing_set_rendering_algorithm(
    _ ptr: UnsafeMutableRawPointer,
    _ renderingAlgorithmRaw: Int64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    guard let renderingAlgorithm = AVAudio3DMixingRenderingAlgorithm(rawValue: Int(renderingAlgorithmRaw)) else {
        outError?.pointee = ffiString("invalid AVAudio3DMixingRenderingAlgorithm")
        return AVA_INVALID_ARGUMENT
    }
    mixing.renderingAlgorithm = renderingAlgorithm
    return AVA_OK
}

@_cdecl("av_audio_3d_mixing_get_source_mode")
public func av_audio_3d_mixing_get_source_mode(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int64 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return 0 }
    return Int64(mixing.sourceMode.rawValue)
}

@_cdecl("av_audio_3d_mixing_set_source_mode")
public func av_audio_3d_mixing_set_source_mode(
    _ ptr: UnsafeMutableRawPointer,
    _ sourceModeRaw: Int64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    guard let sourceMode = AVAudio3DMixingSourceMode(rawValue: Int(sourceModeRaw)) else {
        outError?.pointee = ffiString("invalid AVAudio3DMixingSourceMode")
        return AVA_INVALID_ARGUMENT
    }
    mixing.sourceMode = sourceMode
    return AVA_OK
}

@_cdecl("av_audio_3d_mixing_get_point_source_in_head_mode")
public func av_audio_3d_mixing_get_point_source_in_head_mode(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int64 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return 0 }
    return Int64(mixing.pointSourceInHeadMode.rawValue)
}

@_cdecl("av_audio_3d_mixing_set_point_source_in_head_mode")
public func av_audio_3d_mixing_set_point_source_in_head_mode(
    _ ptr: UnsafeMutableRawPointer,
    _ pointSourceInHeadModeRaw: Int64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    guard let pointSourceInHeadMode = AVAudio3DMixingPointSourceInHeadMode(rawValue: Int(pointSourceInHeadModeRaw)) else {
        outError?.pointee = ffiString("invalid AVAudio3DMixingPointSourceInHeadMode")
        return AVA_INVALID_ARGUMENT
    }
    mixing.pointSourceInHeadMode = pointSourceInHeadMode
    return AVA_OK
}

@_cdecl("av_audio_3d_mixing_get_rate")
public func av_audio_3d_mixing_get_rate(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Float {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return 0 }
    return mixing.rate
}

@_cdecl("av_audio_3d_mixing_set_rate")
public func av_audio_3d_mixing_set_rate(
    _ ptr: UnsafeMutableRawPointer,
    _ rate: Float,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    mixing.rate = rate
    return AVA_OK
}

@_cdecl("av_audio_3d_mixing_get_reverb_blend")
public func av_audio_3d_mixing_get_reverb_blend(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Float {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return 0 }
    return mixing.reverbBlend
}

@_cdecl("av_audio_3d_mixing_set_reverb_blend")
public func av_audio_3d_mixing_set_reverb_blend(
    _ ptr: UnsafeMutableRawPointer,
    _ reverbBlend: Float,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    mixing.reverbBlend = reverbBlend
    return AVA_OK
}

@_cdecl("av_audio_3d_mixing_get_obstruction")
public func av_audio_3d_mixing_get_obstruction(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Float {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return 0 }
    return mixing.obstruction
}

@_cdecl("av_audio_3d_mixing_set_obstruction")
public func av_audio_3d_mixing_set_obstruction(
    _ ptr: UnsafeMutableRawPointer,
    _ obstruction: Float,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    mixing.obstruction = obstruction
    return AVA_OK
}

@_cdecl("av_audio_3d_mixing_get_occlusion")
public func av_audio_3d_mixing_get_occlusion(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Float {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return 0 }
    return mixing.occlusion
}

@_cdecl("av_audio_3d_mixing_set_occlusion")
public func av_audio_3d_mixing_set_occlusion(
    _ ptr: UnsafeMutableRawPointer,
    _ occlusion: Float,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    mixing.occlusion = occlusion
    return AVA_OK
}

@_cdecl("av_audio_3d_mixing_get_position_json")
public func av_audio_3d_mixing_get_position_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return nil }
    let position = mixing.position
    do {
        return ffiString(try avaEncodeJSON(Audio3DVectorPayload(x: position.x, y: position.y, z: position.z)))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_3d_mixing_set_position")
public func av_audio_3d_mixing_set_position(
    _ ptr: UnsafeMutableRawPointer,
    _ x: Float,
    _ y: Float,
    _ z: Float,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let mixing = avaRequire3DMixing(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    mixing.position = AVAudio3DPoint(x: x, y: y, z: z)
    return AVA_OK
}
