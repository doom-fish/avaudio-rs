import AVFoundation
import Foundation

struct AudioCompressedBufferInfoPayload: Codable {
    let packetCapacity: UInt32
    let packetCount: UInt32
    let maximumPacketSize: Int
    let byteCapacity: UInt32
    let byteLength: UInt32
}

@_cdecl("av_audio_compressed_buffer_create")
public func av_audio_compressed_buffer_create(
    _ formatPtr: UnsafeMutableRawPointer,
    _ packetCapacity: UInt32,
    _ maximumPacketSize: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let format = Unmanaged<AVAudioFormat>.fromOpaque(formatPtr).takeUnretainedValue()
    let buffer = AVAudioCompressedBuffer(
        format: format,
        packetCapacity: AVAudioPacketCount(packetCapacity),
        maximumPacketSize: maximumPacketSize
    )
    return Unmanaged.passRetained(buffer).toOpaque()
}

@_cdecl("av_audio_compressed_buffer_release")
public func av_audio_compressed_buffer_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioCompressedBuffer>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_compressed_buffer_info_json")
public func av_audio_compressed_buffer_info_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let buffer = Unmanaged<AVAudioCompressedBuffer>.fromOpaque(ptr).takeUnretainedValue()
    let payload = AudioCompressedBufferInfoPayload(
        packetCapacity: buffer.packetCapacity,
        packetCount: buffer.packetCount,
        maximumPacketSize: buffer.maximumPacketSize,
        byteCapacity: buffer.byteCapacity,
        byteLength: buffer.byteLength
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_compressed_buffer_set_packet_count")
public func av_audio_compressed_buffer_set_packet_count(
    _ ptr: UnsafeMutableRawPointer,
    _ packetCount: UInt32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let buffer = Unmanaged<AVAudioCompressedBuffer>.fromOpaque(ptr).takeUnretainedValue()
    guard packetCount <= buffer.packetCapacity else {
        outError?.pointee = ffiString("packet count exceeds packet capacity")
        return AVA_INVALID_ARGUMENT
    }
    buffer.packetCount = AVAudioPacketCount(packetCount)
    return AVA_OK
}

@_cdecl("av_audio_compressed_buffer_set_byte_length")
public func av_audio_compressed_buffer_set_byte_length(
    _ ptr: UnsafeMutableRawPointer,
    _ byteLength: UInt32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let buffer = Unmanaged<AVAudioCompressedBuffer>.fromOpaque(ptr).takeUnretainedValue()
    guard byteLength <= buffer.byteCapacity else {
        outError?.pointee = ffiString("byte length exceeds byte capacity")
        return AVA_INVALID_ARGUMENT
    }
    buffer.byteLength = byteLength
    return AVA_OK
}

@_cdecl("av_audio_compressed_buffer_copy_format")
public func av_audio_compressed_buffer_copy_format(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let buffer = Unmanaged<AVAudioCompressedBuffer>.fromOpaque(ptr).takeUnretainedValue()
    return Unmanaged.passRetained(buffer.format).toOpaque()
}
