import AVFoundation
import Foundation

struct RoutingArbitrationResultPayload: Codable {
    let defaultDeviceChanged: Bool
    let error: String?
}

@_cdecl("av_audio_routing_arbiter_begin")
public func av_audio_routing_arbiter_begin(
    _ categoryRaw: Int64,
    _ resultCallback: AVAStringCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let category = AVAudioRoutingArbiter.Category(rawValue: Int(categoryRaw)) else {
        outError?.pointee = ffiString("invalid AVAudioRoutingArbiter.Category")
        return AVA_INVALID_ARGUMENT
    }
    AVAudioRoutingArbiter.shared.begin(category: category) { defaultDeviceChanged, error in
        let payload = RoutingArbitrationResultPayload(
            defaultDeviceChanged: defaultDeviceChanged,
            error: error?.localizedDescription
        )
        let json = try? avaEncodeJSON(payload)
        resultCallback?(userData, json.flatMap(ffiString))
        if let userData, let dropUserData {
            dropUserData(userData)
        }
    }
    return AVA_OK
}

@_cdecl("av_audio_routing_arbiter_leave")
public func av_audio_routing_arbiter_leave() {
    AVAudioRoutingArbiter.shared.leave()
}
