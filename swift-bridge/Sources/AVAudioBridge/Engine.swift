import AVFoundation
import Foundation

@_cdecl("av_audio_engine_create")
public func av_audio_engine_create(
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioEngine()).toOpaque()
}

@_cdecl("av_audio_engine_release")
public func av_audio_engine_release(_ enginePtr: UnsafeMutableRawPointer?) {
    guard let enginePtr else { return }
    Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).release()
}

@_cdecl("av_audio_engine_info_json")
public func av_audio_engine_info_json(
    _ enginePtr: UnsafeMutableRawPointer,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(AudioEngineInfoPayload(isRunning: engine.isRunning)))
    } catch {
        outErrorMessage?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_engine_prepare")
public func av_audio_engine_prepare(_ enginePtr: UnsafeMutableRawPointer) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    engine.prepare()
}

@_cdecl("av_audio_engine_start")
public func av_audio_engine_start(
    _ enginePtr: UnsafeMutableRawPointer,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    do {
        try engine.start()
        return AVA_OK
    } catch {
        outErrorMessage?.pointee = ffiString(error.localizedDescription)
        return AVA_ENGINE_ERROR
    }
}

@_cdecl("av_audio_engine_stop")
public func av_audio_engine_stop(_ enginePtr: UnsafeMutableRawPointer) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    engine.stop()
}

@_cdecl("av_audio_engine_reset")
public func av_audio_engine_reset(_ enginePtr: UnsafeMutableRawPointer) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    engine.reset()
}

@_cdecl("av_audio_engine_attach_player_node")
public func av_audio_engine_attach_player_node(
    _ enginePtr: UnsafeMutableRawPointer,
    _ playerPtr: UnsafeMutableRawPointer
) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    engine.attach(player.node)
}

@_cdecl("av_audio_engine_connect_player_to_main_mixer")
public func av_audio_engine_connect_player_to_main_mixer(
    _ enginePtr: UnsafeMutableRawPointer,
    _ playerPtr: UnsafeMutableRawPointer,
    _ formatPtr: UnsafeMutableRawPointer?
) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let format = formatPtr.map { Unmanaged<AVAudioFormat>.fromOpaque($0).takeUnretainedValue() }
    engine.connect(player.node, to: engine.mainMixerNode, format: format)
}

@_cdecl("av_audio_engine_copy_main_mixer_output_format")
public func av_audio_engine_copy_main_mixer_output_format(
    _ enginePtr: UnsafeMutableRawPointer,
    _ bus: Int
) -> UnsafeMutableRawPointer? {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let format = engine.mainMixerNode.outputFormat(forBus: AVAudioNodeBus(bus))
    return Unmanaged.passRetained(format).toOpaque()
}
