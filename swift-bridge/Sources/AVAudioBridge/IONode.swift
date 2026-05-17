import AVFoundation
import Foundation

private func avaRequireIONode(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> AVAudioIONode? {
    guard let node = Unmanaged<NSObject>.fromOpaque(ptr).takeUnretainedValue() as? AVAudioIONode else {
        outError?.pointee = ffiString("object is not an AVAudioIONode")
        return nil
    }
    return node
}

@_cdecl("av_audio_io_node_get_presentation_latency")
public func av_audio_io_node_get_presentation_latency(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Double {
    guard let node = avaRequireIONode(ptr, outError) else { return 0 }
    return node.presentationLatency
}

@_cdecl("av_audio_io_node_is_voice_processing_enabled")
public func av_audio_io_node_is_voice_processing_enabled(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let node = avaRequireIONode(ptr, outError) else { return false }
    return node.isVoiceProcessingEnabled
}

@_cdecl("av_audio_io_node_set_voice_processing_enabled")
public func av_audio_io_node_set_voice_processing_enabled(
    _ ptr: UnsafeMutableRawPointer,
    _ enabled: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let node = avaRequireIONode(ptr, outError) else { return AVA_INVALID_ARGUMENT }
    do {
        try node.setVoiceProcessingEnabled(enabled)
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}
