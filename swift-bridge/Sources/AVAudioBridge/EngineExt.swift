import AVFoundation
import Foundation

@_cdecl("av_audio_engine_attach_node")
public func av_audio_engine_attach_node(
    _ enginePtr: UnsafeMutableRawPointer,
    _ nodePtr: UnsafeMutableRawPointer
) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let node = Unmanaged<AVAudioNode>.fromOpaque(nodePtr).takeUnretainedValue()
    engine.attach(node)
}

@_cdecl("av_audio_engine_connect_nodes")
public func av_audio_engine_connect_nodes(
    _ enginePtr: UnsafeMutableRawPointer,
    _ fromNodePtr: UnsafeMutableRawPointer,
    _ toNodePtr: UnsafeMutableRawPointer,
    _ formatPtr: UnsafeMutableRawPointer?
) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let fromNode = Unmanaged<AVAudioNode>.fromOpaque(fromNodePtr).takeUnretainedValue()
    let toNode = Unmanaged<AVAudioNode>.fromOpaque(toNodePtr).takeUnretainedValue()
    let format = formatPtr.map { Unmanaged<AVAudioFormat>.fromOpaque($0).takeUnretainedValue() }
    engine.connect(fromNode, to: toNode, format: format)
}

@_cdecl("av_audio_engine_connect_node_to_main_mixer")
public func av_audio_engine_connect_node_to_main_mixer(
    _ enginePtr: UnsafeMutableRawPointer,
    _ nodePtr: UnsafeMutableRawPointer,
    _ formatPtr: UnsafeMutableRawPointer?
) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let node = Unmanaged<AVAudioNode>.fromOpaque(nodePtr).takeUnretainedValue()
    let format = formatPtr.map { Unmanaged<AVAudioFormat>.fromOpaque($0).takeUnretainedValue() }
    engine.connect(node, to: engine.mainMixerNode, format: format)
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
