import AVFoundation
import Foundation

@_cdecl("av_audio_input_node_release")
public func av_audio_input_node_release(_ ptr: UnsafeMutableRawPointer?) {
    av_audio_node_release(ptr)
}

@_cdecl("av_audio_input_node_output_format_json")
public func av_audio_input_node_output_format_json(
    _ nodePtr: UnsafeMutableRawPointer,
    _ bus: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(avaEncodeFormatInfo(node.outputFormat(forBus: AVAudioNodeBus(bus)))))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_input_node_input_format_json")
public func av_audio_input_node_input_format_json(
    _ nodePtr: UnsafeMutableRawPointer,
    _ bus: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(avaEncodeFormatInfo(node.inputFormat(forBus: AVAudioNodeBus(bus)))))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_input_node_install_tap_scaffold")
public func av_audio_input_node_install_tap_scaffold(
    _ nodePtr: UnsafeMutableRawPointer,
    _ bus: Int,
    _ bufferSize: UInt32,
    _ formatPtr: UnsafeMutableRawPointer?
) -> Int32 {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    let audioBus = AVAudioNodeBus(bus)
    let format = formatPtr.map { Unmanaged<AVAudioFormat>.fromOpaque($0).takeUnretainedValue() }
    node.removeTap(onBus: audioBus)
    node.installTap(onBus: audioBus, bufferSize: AVAudioFrameCount(bufferSize), format: format) { _, _ in }
    return AVA_OK
}

@_cdecl("av_audio_input_node_remove_tap")
public func av_audio_input_node_remove_tap(
    _ nodePtr: UnsafeMutableRawPointer,
    _ bus: Int
) {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    node.removeTap(onBus: AVAudioNodeBus(bus))
}
