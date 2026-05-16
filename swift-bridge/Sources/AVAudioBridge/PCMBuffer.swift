import AVFoundation
import Foundation

@_cdecl("av_audio_pcm_buffer_create")
public func av_audio_pcm_buffer_create(
    _ formatPtr: UnsafeMutableRawPointer,
    _ frameCapacity: UInt32,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let format = Unmanaged<AVAudioFormat>.fromOpaque(formatPtr).takeUnretainedValue()
    guard let buffer = AVAudioPCMBuffer(
        pcmFormat: format,
        frameCapacity: AVAudioFrameCount(frameCapacity)
    ) else {
        outErrorMessage?.pointee = ffiString("failed to allocate AVAudioPCMBuffer")
        return nil
    }
    return Unmanaged.passRetained(buffer).toOpaque()
}

@_cdecl("av_audio_pcm_buffer_set_frame_length")
public func av_audio_pcm_buffer_set_frame_length(
    _ bufferPtr: UnsafeMutableRawPointer,
    _ frameLength: UInt32,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    guard frameLength <= buffer.frameCapacity else {
        outErrorMessage?.pointee = ffiString("frame length exceeds frame capacity")
        return AVA_INVALID_ARGUMENT
    }
    buffer.frameLength = AVAudioFrameCount(frameLength)
    return AVA_OK
}

@_cdecl("av_audio_pcm_buffer_release")
public func av_audio_pcm_buffer_release(_ bufferPtr: UnsafeMutableRawPointer?) {
    guard let bufferPtr else { return }
    Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).release()
}

@_cdecl("av_audio_pcm_buffer_info_json")
public func av_audio_pcm_buffer_info_json(
    _ bufferPtr: UnsafeMutableRawPointer,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    let payload = PCMBufferInfoPayload(
        frameCapacity: buffer.frameCapacity,
        frameLength: buffer.frameLength,
        format: avaEncodeFormatInfo(buffer.format)
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outErrorMessage?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_pcm_buffer_copy_format")
public func av_audio_pcm_buffer_copy_format(_ bufferPtr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    return Unmanaged.passRetained(buffer.format).toOpaque()
}
