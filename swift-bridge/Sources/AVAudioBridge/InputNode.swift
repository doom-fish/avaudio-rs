import AVAudioObjCBridge
import AVFoundation
import Foundation

private struct AudioVoiceProcessingOtherAudioDuckingConfigurationPayload: Codable {
    let enableAdvancedDucking: Bool
    let duckingLevelRaw: Int64
}

final class InputNodeManualRenderingInputBlockBox {
    let format: AVAudioFormat
    let callback: AVAInputNodeInputBlockCallback?
    let userData: UnsafeMutableRawPointer?
    let dropUserData: AVADropCallback?
    private var current: AVAudioPCMBuffer?

    init(
        format: AVAudioFormat,
        callback: AVAInputNodeInputBlockCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) {
        self.format = format
        self.callback = callback
        self.userData = userData
        self.dropUserData = dropUserData
    }

    func provide(frameCount: AVAudioFrameCount) -> UnsafePointer<AudioBufferList>? {
        let buffer = callback?(userData, frameCount).map {
            Unmanaged<AVAudioPCMBuffer>.fromOpaque($0).takeRetainedValue()
        }
        guard let buffer, buffer.format.isEqual(format) else {
            current = nil
            return nil
        }
        current = buffer
        return buffer.audioBufferList
    }

    deinit {
        if let userData, let dropUserData {
            dropUserData(userData)
        }
    }
}

final class InputNodeSpeechActivityListenerBox {
    let callback: AVAIntCallback?
    let userData: UnsafeMutableRawPointer?
    let dropUserData: AVADropCallback?

    init(
        callback: AVAIntCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) {
        self.callback = callback
        self.userData = userData
        self.dropUserData = dropUserData
    }

    @available(macOS 14.0, *)
    func notify(_ event: AVAudioVoiceProcessingSpeechActivityEvent) {
        callback?(userData, Int64(event.rawValue))
    }

    deinit {
        if let userData, let dropUserData {
            dropUserData(userData)
        }
    }
}

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
    _ bus: UInt32,
    _ bufferSize: UInt32,
    _ formatPtr: UnsafeMutableRawPointer?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    let format = formatPtr.map { Unmanaged<AVAudioFormat>.fromOpaque($0).takeUnretainedValue() }
    var error: NSError?
    guard AVAXNodeInstallTap(node, UInt(bus), bufferSize, format, { _, _ in }, &error) else {
        avaReportObjCFailure("AVAudioNode.installTap", error, outErrorMessage)
        return AVA_CALLBACK_ERROR
    }
    return AVA_OK
}

@_cdecl("av_audio_input_node_remove_tap")
public func av_audio_input_node_remove_tap(
    _ nodePtr: UnsafeMutableRawPointer,
    _ bus: UInt32
) {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    node.removeTap(onBus: AVAudioNodeBus(bus))
}

@_cdecl("av_audio_input_node_set_manual_rendering_input_pcm_format")
public func av_audio_input_node_set_manual_rendering_input_pcm_format(
    _ nodePtr: UnsafeMutableRawPointer,
    _ formatPtr: UnsafeMutableRawPointer,
    _ callback: AVAInputNodeInputBlockCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?
) -> Bool {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    let format = Unmanaged<AVAudioFormat>.fromOpaque(formatPtr).takeUnretainedValue()
    let box = InputNodeManualRenderingInputBlockBox(
        format: format,
        callback: callback,
        userData: userData,
        dropUserData: dropUserData
    )
    guard #available(macOS 10.13, *) else {
        return false
    }
    guard callback != nil else {
        return node.setManualRenderingInputPCMFormat(format) { _ in nil }
    }
    return node.setManualRenderingInputPCMFormat(format) { frameCount in
        box.provide(frameCount: frameCount)
    }
}

@_cdecl("av_audio_input_node_get_voice_processing_bypassed")
public func av_audio_input_node_get_voice_processing_bypassed(_ nodePtr: UnsafeMutableRawPointer) -> Bool {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    guard #available(macOS 10.15, *) else {
        return false
    }
    return node.isVoiceProcessingBypassed
}

@_cdecl("av_audio_input_node_set_voice_processing_bypassed")
public func av_audio_input_node_set_voice_processing_bypassed(
    _ nodePtr: UnsafeMutableRawPointer,
    _ bypassed: Bool
) {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    guard #available(macOS 10.15, *) else {
        return
    }
    node.isVoiceProcessingBypassed = bypassed
}

@_cdecl("av_audio_input_node_get_voice_processing_agc_enabled")
public func av_audio_input_node_get_voice_processing_agc_enabled(_ nodePtr: UnsafeMutableRawPointer) -> Bool {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    guard #available(macOS 10.15, *) else {
        return false
    }
    return node.isVoiceProcessingAGCEnabled
}

@_cdecl("av_audio_input_node_set_voice_processing_agc_enabled")
public func av_audio_input_node_set_voice_processing_agc_enabled(
    _ nodePtr: UnsafeMutableRawPointer,
    _ enabled: Bool
) {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    guard #available(macOS 10.15, *) else {
        return
    }
    node.isVoiceProcessingAGCEnabled = enabled
}

@_cdecl("av_audio_input_node_get_voice_processing_input_muted")
public func av_audio_input_node_get_voice_processing_input_muted(_ nodePtr: UnsafeMutableRawPointer) -> Bool {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    guard #available(macOS 10.15, *) else {
        return false
    }
    return node.isVoiceProcessingInputMuted
}

@_cdecl("av_audio_input_node_set_voice_processing_input_muted")
public func av_audio_input_node_set_voice_processing_input_muted(
    _ nodePtr: UnsafeMutableRawPointer,
    _ muted: Bool
) {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    guard #available(macOS 10.15, *) else {
        return
    }
    node.isVoiceProcessingInputMuted = muted
}

@_cdecl("av_audio_input_node_set_muted_speech_activity_event_listener")
public func av_audio_input_node_set_muted_speech_activity_event_listener(
    _ nodePtr: UnsafeMutableRawPointer,
    _ callback: AVAIntCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?
) -> Bool {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    let box = InputNodeSpeechActivityListenerBox(
        callback: callback,
        userData: userData,
        dropUserData: dropUserData
    )
    guard #available(macOS 14.0, *) else {
        return false
    }
    guard callback != nil else {
        return node.setMutedSpeechActivityEventListener(nil)
    }
    return node.setMutedSpeechActivityEventListener { event in
        box.notify(event)
    }
}

@_cdecl("av_audio_input_node_get_other_audio_ducking_configuration_json")
public func av_audio_input_node_get_other_audio_ducking_configuration_json(
    _ nodePtr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    guard #available(macOS 14.0, *) else {
        outError?.pointee = ffiString("voice processing ducking requires macOS 14.0")
        return nil
    }
    let configuration = node.voiceProcessingOtherAudioDuckingConfiguration
    let payload = AudioVoiceProcessingOtherAudioDuckingConfigurationPayload(
        enableAdvancedDucking: configuration.enableAdvancedDucking.boolValue,
        duckingLevelRaw: Int64(configuration.duckingLevel.rawValue)
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_input_node_set_other_audio_ducking_configuration")
public func av_audio_input_node_set_other_audio_ducking_configuration(
    _ nodePtr: UnsafeMutableRawPointer,
    _ enableAdvancedDucking: Bool,
    _ duckingLevelRaw: Int64,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
    guard #available(macOS 14.0, *) else {
        outError?.pointee = ffiString("voice processing ducking requires macOS 14.0")
        return AVA_INVALID_ARGUMENT
    }
    guard let duckingLevel = AVAudioVoiceProcessingOtherAudioDuckingConfiguration.Level(rawValue: Int(duckingLevelRaw)) else {
        outError?.pointee = ffiString("invalid AVAudioVoiceProcessingOtherAudioDuckingConfiguration.Level")
        return AVA_INVALID_ARGUMENT
    }
    node.voiceProcessingOtherAudioDuckingConfiguration = AVAudioVoiceProcessingOtherAudioDuckingConfiguration(
        enableAdvancedDucking: ObjCBool(enableAdvancedDucking),
        duckingLevel: duckingLevel
    )
    return AVA_OK
}
