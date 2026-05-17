import AVFoundation
import Foundation

struct AudioConnectionPointInfoPayload: Codable {
    let bus: UInt
    let nodeRaw: UInt64
}

private func avaRawPointerValue(_ object: AnyObject?) -> UInt64 {
    guard let object else { return 0 }
    return UInt64(UInt(bitPattern: Unmanaged.passUnretained(object).toOpaque()))
}

@_cdecl("av_audio_connection_point_create")
public func av_audio_connection_point_create(
    _ nodePtr: UnsafeMutableRawPointer,
    _ bus: UInt,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let node = Unmanaged<AVAudioNode>.fromOpaque(nodePtr).takeUnretainedValue()
    let point = AVAudioConnectionPoint(node: node, bus: AVAudioNodeBus(bus))
    return Unmanaged.passRetained(point).toOpaque()
}

@_cdecl("av_audio_connection_point_release")
public func av_audio_connection_point_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioConnectionPoint>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_connection_point_info_json")
public func av_audio_connection_point_info_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let point = Unmanaged<AVAudioConnectionPoint>.fromOpaque(ptr).takeUnretainedValue()
    let payload = AudioConnectionPointInfoPayload(
        bus: UInt(point.bus),
        nodeRaw: avaRawPointerValue(point.node)
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}
