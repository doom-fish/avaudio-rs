import AVFoundation
import Foundation

@_cdecl("av_audio_output_node_release")
public func av_audio_output_node_release(_ ptr: UnsafeMutableRawPointer?) {
    av_audio_node_release(ptr)
}

@_cdecl("av_audio_output_node_output_format_json")
public func av_audio_output_node_output_format_json(
    _ nodePtr: UnsafeMutableRawPointer,
    _ bus: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let node = Unmanaged<AVAudioOutputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(avaEncodeFormatInfo(node.outputFormat(forBus: AVAudioNodeBus(bus)))))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}
