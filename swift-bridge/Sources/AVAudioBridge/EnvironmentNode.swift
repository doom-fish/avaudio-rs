import AVFoundation
import Foundation

struct ListenerPositionPayload: Codable {
    let x: Float
    let y: Float
    let z: Float
}

struct ListenerOrientationPayload: Codable {
    let yaw: Float
    let pitch: Float
    let roll: Float
}

struct DistanceAttenuationPayload: Codable {
    let model: Int32
    let referenceDistance: Float
    let maximumDistance: Float
    let rolloffFactor: Float
}

@_cdecl("av_audio_environment_node_create")
public func av_audio_environment_node_create() -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioEnvironmentNode()).toOpaque()
}

@_cdecl("av_audio_environment_node_release")
public func av_audio_environment_node_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioEnvironmentNode>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_environment_node_set_listener_position")
public func av_audio_environment_node_set_listener_position(
    _ ptr: UnsafeMutableRawPointer,
    _ x: Float,
    _ y: Float,
    _ z: Float
) {
    let node = Unmanaged<AVAudioEnvironmentNode>.fromOpaque(ptr).takeUnretainedValue()
    node.listenerPosition = AVAudio3DPoint(x: x, y: y, z: z)
}

@_cdecl("av_audio_environment_node_get_listener_position_json")
public func av_audio_environment_node_get_listener_position_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let node = Unmanaged<AVAudioEnvironmentNode>.fromOpaque(ptr).takeUnretainedValue()
    let payload = ListenerPositionPayload(
        x: node.listenerPosition.x,
        y: node.listenerPosition.y,
        z: node.listenerPosition.z
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_environment_node_set_listener_orientation")
public func av_audio_environment_node_set_listener_orientation(
    _ ptr: UnsafeMutableRawPointer,
    _ yaw: Float,
    _ pitch: Float,
    _ roll: Float
) {
    let node = Unmanaged<AVAudioEnvironmentNode>.fromOpaque(ptr).takeUnretainedValue()
    node.listenerAngularOrientation = AVAudio3DAngularOrientation(yaw: yaw, pitch: pitch, roll: roll)
}

@_cdecl("av_audio_environment_node_get_listener_orientation_json")
public func av_audio_environment_node_get_listener_orientation_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let node = Unmanaged<AVAudioEnvironmentNode>.fromOpaque(ptr).takeUnretainedValue()
    let orientation = node.listenerAngularOrientation
    let payload = ListenerOrientationPayload(yaw: orientation.yaw, pitch: orientation.pitch, roll: orientation.roll)
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_environment_node_set_distance_attenuation")
public func av_audio_environment_node_set_distance_attenuation(
    _ ptr: UnsafeMutableRawPointer,
    _ model: Int32,
    _ referenceDistance: Float,
    _ maximumDistance: Float,
    _ rolloffFactor: Float
) {
    let node = Unmanaged<AVAudioEnvironmentNode>.fromOpaque(ptr).takeUnretainedValue()
    let params = node.distanceAttenuationParameters
    params.distanceAttenuationModel = AVAudioEnvironmentDistanceAttenuationModel(rawValue: Int(model)) ?? .exponential
    params.referenceDistance = referenceDistance
    params.maximumDistance = maximumDistance
    params.rolloffFactor = rolloffFactor
}

@_cdecl("av_audio_environment_node_get_distance_attenuation_json")
public func av_audio_environment_node_get_distance_attenuation_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let node = Unmanaged<AVAudioEnvironmentNode>.fromOpaque(ptr).takeUnretainedValue()
    let params = node.distanceAttenuationParameters
    let payload = DistanceAttenuationPayload(
        model: Int32(params.distanceAttenuationModel.rawValue),
        referenceDistance: params.referenceDistance,
        maximumDistance: params.maximumDistance,
        rolloffFactor: params.rolloffFactor
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_environment_node_set_reverb_blend")
public func av_audio_environment_node_set_reverb_blend(
    _ ptr: UnsafeMutableRawPointer,
    _ blend: Float
) {
    let node = Unmanaged<AVAudioEnvironmentNode>.fromOpaque(ptr).takeUnretainedValue()
    node.reverbParameters.enable = blend > 0
    node.reverbParameters.level = blend
}

@_cdecl("av_audio_environment_node_get_reverb_blend")
public func av_audio_environment_node_get_reverb_blend(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioEnvironmentNode>.fromOpaque(ptr).takeUnretainedValue().reverbParameters.level
}
