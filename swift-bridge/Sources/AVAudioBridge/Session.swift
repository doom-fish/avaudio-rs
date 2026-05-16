import AVFoundation
import Foundation

@_cdecl("av_audio_session_get_sample_rate")
public func av_audio_session_get_sample_rate() -> Double {
    #if os(macOS)
    return 48_000.0
    #else
    return AVAudioSession.sharedInstance().sampleRate
    #endif
}

@_cdecl("av_audio_session_get_output_volume")
public func av_audio_session_get_output_volume() -> Float {
    #if os(macOS)
    return 1.0
    #else
    return AVAudioSession.sharedInstance().outputVolume
    #endif
}

@_cdecl("av_audio_session_is_other_audio_playing")
public func av_audio_session_is_other_audio_playing() -> Bool {
    #if os(macOS)
    return false
    #else
    return AVAudioSession.sharedInstance().isOtherAudioPlaying
    #endif
}
