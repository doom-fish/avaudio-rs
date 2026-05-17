import AVFoundation
import Foundation

@_cdecl("av_audio_session_capability_create")
public func av_audio_session_capability_create() -> UnsafeMutableRawPointer? {
    if #available(macOS 26.0, *) {
        return Unmanaged.passRetained(AVAudioSessionCapability()).toOpaque()
    }
    return nil
}

@_cdecl("av_audio_session_capability_release")
public func av_audio_session_capability_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    if #available(macOS 26.0, *) {
        Unmanaged<AVAudioSessionCapability>.fromOpaque(ptr).release()
    }
}

@_cdecl("av_audio_session_capability_is_supported")
public func av_audio_session_capability_is_supported(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr, #available(macOS 26.0, *) else { return false }
    return Unmanaged<AVAudioSessionCapability>.fromOpaque(ptr).takeUnretainedValue().isSupported
}

@_cdecl("av_audio_session_capability_is_enabled")
public func av_audio_session_capability_is_enabled(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr, #available(macOS 26.0, *) else { return false }
    return Unmanaged<AVAudioSessionCapability>.fromOpaque(ptr).takeUnretainedValue().isEnabled
}
