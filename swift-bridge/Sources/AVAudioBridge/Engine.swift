import AVFoundation
import Foundation

struct AudioEngineManualRenderingInfoPayload: Codable {
    let isInManualRenderingMode: Bool
    let manualRenderingModeRaw: Int64
    let manualRenderingMaximumFrameCount: UInt32
    let manualRenderingSampleTime: Int64
}

private func avaEncodeManualRenderingInfo(_ engine: AVAudioEngine) -> AudioEngineManualRenderingInfoPayload {
    AudioEngineManualRenderingInfoPayload(
        isInManualRenderingMode: engine.isInManualRenderingMode,
        manualRenderingModeRaw: Int64(engine.manualRenderingMode.rawValue),
        manualRenderingMaximumFrameCount: engine.manualRenderingMaximumFrameCount,
        manualRenderingSampleTime: engine.manualRenderingSampleTime
    )
}

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

@_cdecl("av_audio_engine_enable_manual_rendering_mode")
public func av_audio_engine_enable_manual_rendering_mode(
    _ enginePtr: UnsafeMutableRawPointer,
    _ modeRaw: Int64,
    _ formatPtr: UnsafeMutableRawPointer,
    _ maximumFrameCount: UInt32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let format = Unmanaged<AVAudioFormat>.fromOpaque(formatPtr).takeUnretainedValue()
    guard let mode = AVAudioEngineManualRenderingMode(rawValue: Int(modeRaw)) else {
        outError?.pointee = ffiString("invalid manual rendering mode")
        return AVA_INVALID_ARGUMENT
    }
    do {
        try engine.enableManualRenderingMode(mode, format: format, maximumFrameCount: maximumFrameCount)
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_ENGINE_ERROR
    }
}

@_cdecl("av_audio_engine_disable_manual_rendering_mode")
public func av_audio_engine_disable_manual_rendering_mode(_ enginePtr: UnsafeMutableRawPointer) {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    engine.disableManualRenderingMode()
}

@_cdecl("av_audio_engine_manual_rendering_info_json")
public func av_audio_engine_manual_rendering_info_json(
    _ enginePtr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(avaEncodeManualRenderingInfo(engine)))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_engine_copy_manual_rendering_format")
public func av_audio_engine_copy_manual_rendering_format(_ enginePtr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    return Unmanaged.passRetained(engine.manualRenderingFormat).toOpaque()
}

@_cdecl("av_audio_engine_render_offline")
public func av_audio_engine_render_offline(
    _ enginePtr: UnsafeMutableRawPointer,
    _ numberOfFrames: UInt32,
    _ bufferPtr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int64 {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    do {
        let status = try engine.renderOffline(numberOfFrames, to: buffer)
        return Int64(status.rawValue)
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return Int64.min
    }
}

@_cdecl("av_audio_engine_manual_rendering_block_render")
public func av_audio_engine_manual_rendering_block_render(
    _ enginePtr: UnsafeMutableRawPointer,
    _ numberOfFrames: UInt32,
    _ bufferPtr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int64 {
    let engine = Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue()
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    var renderError: OSStatus = noErr
    let status = engine.manualRenderingBlock(numberOfFrames, buffer.mutableAudioBufferList, &renderError)
    if status == .error, renderError != noErr {
        outError?.pointee = ffiString("manual rendering block failed with OSStatus \(renderError)")
    }
    return Int64(status.rawValue)
}

@_cdecl("av_audio_engine_configuration_change_notification_name")
public func av_audio_engine_configuration_change_notification_name(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    ffiString(NSNotification.Name.AVAudioEngineConfigurationChange.rawValue)
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
