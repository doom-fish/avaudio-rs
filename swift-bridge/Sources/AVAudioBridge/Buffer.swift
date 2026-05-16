import AVFoundation
import Foundation

struct AudioBufferInfoPayload: Codable {
    let format: AudioFormatInfoPayload
    let bufferCount: UInt32
    let bytesPerBuffer: [UInt32]
    let channelCounts: [UInt32]
}

@_cdecl("av_audio_buffer_info_json")
public func av_audio_buffer_info_json(
    _ bufferPtr: UnsafeMutableRawPointer,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let buffer = Unmanaged<AVAudioBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    let audioBuffers = UnsafeMutableAudioBufferListPointer(buffer.mutableAudioBufferList)
    let payload = AudioBufferInfoPayload(
        format: avaEncodeFormatInfo(buffer.format),
        bufferCount: UInt32(audioBuffers.count),
        bytesPerBuffer: audioBuffers.map(\.mDataByteSize),
        channelCounts: audioBuffers.map(\.mNumberChannels)
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outErrorMessage?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}
