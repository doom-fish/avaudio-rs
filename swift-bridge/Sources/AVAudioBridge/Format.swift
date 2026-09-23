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

@_cdecl("av_audio_format_create_with_common_format")
public func av_audio_format_create_with_common_format(
    _ commonFormatRaw: Int32,
    _ sampleRate: Double,
    _ channelCount: UInt32,
    _ interleaved: Bool,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let commonFormat = AVAudioCommonFormat(rawValue: UInt(clamping: commonFormatRaw)),
          commonFormat != .otherFormat else {
        outErrorMessage?.pointee = ffiString("unsupported AVAudioCommonFormat \(commonFormatRaw)")
        return nil
    }
    guard let format = AVAudioFormat(
        commonFormat: commonFormat,
        sampleRate: sampleRate,
        channels: AVAudioChannelCount(channelCount),
        interleaved: interleaved
    ) else {
        outErrorMessage?.pointee = ffiString("failed to create AVAudioFormat")
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
