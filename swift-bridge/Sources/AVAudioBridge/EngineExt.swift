import AVAudioObjCBridge
import AVFoundation
import Foundation

func avaReportObjCFailure(
    _ operation: String,
    _ error: NSError?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    let reason = error?.localizedDescription ?? "unknown failure"
    outErrorMessage?.pointee = ffiString("\(operation) failed: \(reason)")
}

@_cdecl("av_audio_engine_attach_node")
public func av_audio_engine_attach_node(
    _ enginePtr: UnsafeMutableRawPointer,
    _ nodePtr: UnsafeMutableRawPointer,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let node = Unmanaged<AVAudioNode>.fromOpaque(nodePtr).takeUnretainedValue()
    var error: NSError?
    guard AVAXEngineAttach(engine, node, &error) else {
        avaReportObjCFailure("AVAudioEngine.attach", error, outErrorMessage)
        return AVA_ENGINE_ERROR
    }
    return AVA_OK
}

@_cdecl("av_audio_engine_connect_nodes")
public func av_audio_engine_connect_nodes(
    _ enginePtr: UnsafeMutableRawPointer,
    _ fromNodePtr: UnsafeMutableRawPointer,
    _ toNodePtr: UnsafeMutableRawPointer,
    _ formatPtr: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let fromNode = Unmanaged<AVAudioNode>.fromOpaque(fromNodePtr).takeUnretainedValue()
    let toNode = Unmanaged<AVAudioNode>.fromOpaque(toNodePtr).takeUnretainedValue()
    let format = formatPtr.map { Unmanaged<AVAudioFormat>.fromOpaque($0).takeUnretainedValue() }
    var error: NSError?
    guard AVAXEngineConnect(engine, fromNode, toNode, format, &error) else {
        avaReportObjCFailure("AVAudioEngine.connect", error, outErrorMessage)
        return AVA_ENGINE_ERROR
    }
    return AVA_OK
}

@_cdecl("av_audio_engine_connect_node_to_main_mixer")
public func av_audio_engine_connect_node_to_main_mixer(
    _ enginePtr: UnsafeMutableRawPointer,
    _ nodePtr: UnsafeMutableRawPointer,
    _ formatPtr: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let node = Unmanaged<AVAudioNode>.fromOpaque(nodePtr).takeUnretainedValue()
    let format = formatPtr.map { Unmanaged<AVAudioFormat>.fromOpaque($0).takeUnretainedValue() }
    var error: NSError?
    guard AVAXEngineConnect(engine, node, engine.mainMixerNode, format, &error) else {
        avaReportObjCFailure("AVAudioEngine.connect", error, outErrorMessage)
        return AVA_ENGINE_ERROR
    }
    return AVA_OK
}

@_cdecl("av_audio_engine_get_main_mixer_node")
public func av_audio_engine_get_main_mixer_node(
    _ enginePtr: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    return Unmanaged.passRetained(engine.mainMixerNode).toOpaque()
}

@_cdecl("av_audio_engine_get_input_node")
public func av_audio_engine_get_input_node(
    _ enginePtr: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    return Unmanaged.passRetained(engine.inputNode).toOpaque()
}

@_cdecl("av_audio_engine_get_output_node")
public func av_audio_engine_get_output_node(
    _ enginePtr: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    return Unmanaged.passRetained(engine.outputNode).toOpaque()
}
