import AVFoundation
import Foundation

@_cdecl("av_audio_file_open_for_reading")
public func av_audio_file_open_for_reading(
    _ pathPtr: UnsafePointer<CChar>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let path = String(cString: pathPtr)
    do {
        let file = try AVAudioFile(forReading: URL(fileURLWithPath: path))
        return Unmanaged.passRetained(file).toOpaque()
    } catch {
        outErrorMessage?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_file_release")
public func av_audio_file_release(_ filePtr: UnsafeMutableRawPointer?) {
    guard let filePtr else { return }
    Unmanaged<AVAudioFile>.fromOpaque(filePtr).release()
}

@_cdecl("av_audio_file_info_json")
public func av_audio_file_info_json(
    _ filePtr: UnsafeMutableRawPointer,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let file = Unmanaged<AVAudioFile>.fromOpaque(filePtr).takeUnretainedValue()
    let payload = AudioFileInfoPayload(
        lengthFrames: file.length,
        processingFormat: avaEncodeFormatInfo(file.processingFormat),
        fileFormat: avaEncodeFormatInfo(file.fileFormat)
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outErrorMessage?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_file_copy_processing_format")
public func av_audio_file_copy_processing_format(_ filePtr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let file = Unmanaged<AVAudioFile>.fromOpaque(filePtr).takeUnretainedValue()
    return Unmanaged.passRetained(file.processingFormat).toOpaque()
}

@_cdecl("av_audio_file_copy_file_format")
public func av_audio_file_copy_file_format(_ filePtr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let file = Unmanaged<AVAudioFile>.fromOpaque(filePtr).takeUnretainedValue()
    return Unmanaged.passRetained(file.fileFormat).toOpaque()
}

@_cdecl("av_audio_file_read_pcm_buffer")
public func av_audio_file_read_pcm_buffer(
    _ filePtr: UnsafeMutableRawPointer,
    _ frameCount: UInt32,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let file = Unmanaged<AVAudioFile>.fromOpaque(filePtr).takeUnretainedValue()
    guard let buffer = AVAudioPCMBuffer(
        pcmFormat: file.processingFormat,
        frameCapacity: AVAudioFrameCount(frameCount)
    ) else {
        outErrorMessage?.pointee = ffiString("failed to allocate AVAudioPCMBuffer")
        return nil
    }
    do {
        try file.read(into: buffer, frameCount: AVAudioFrameCount(frameCount))
        return Unmanaged.passRetained(buffer).toOpaque()
    } catch {
        outErrorMessage?.pointee = ffiString(error.localizedDescription)
        return nil
    }
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
