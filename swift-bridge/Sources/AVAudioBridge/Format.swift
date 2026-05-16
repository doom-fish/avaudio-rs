import AVFoundation
import Foundation

@_cdecl("av_audio_format_create_standard")
public func av_audio_format_create_standard(
    _ sampleRate: Double,
    _ channelCount: UInt32,
    _ interleaved: Bool,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let format = AVAudioFormat(
        commonFormat: .pcmFormatFloat32,
        sampleRate: sampleRate,
        channels: AVAudioChannelCount(channelCount),
        interleaved: interleaved
    ) else {
        outErrorMessage?.pointee = ffiString("failed to create standard AVAudioFormat")
        return nil
    }
    return Unmanaged.passRetained(format).toOpaque()
}

@_cdecl("av_audio_format_release")
public func av_audio_format_release(_ formatPtr: UnsafeMutableRawPointer?) {
    guard let formatPtr else { return }
    Unmanaged<AVAudioFormat>.fromOpaque(formatPtr).release()
}

@_cdecl("av_audio_format_info_json")
public func av_audio_format_info_json(
    _ formatPtr: UnsafeMutableRawPointer,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let format = Unmanaged<AVAudioFormat>.fromOpaque(formatPtr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(avaEncodeFormatInfo(format)))
    } catch {
        outErrorMessage?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}
