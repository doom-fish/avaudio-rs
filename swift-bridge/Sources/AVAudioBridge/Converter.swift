import AVFoundation
import Foundation

struct ConverterInfoPayload: Codable {
    let inputFormat: AudioFormatInfoPayload
    let outputFormat: AudioFormatInfoPayload
}

final class AudioConverterBox {
    let converter: AVAudioConverter

    init(converter: AVAudioConverter) {
        self.converter = converter
    }
}

@_cdecl("av_audio_converter_create")
public func av_audio_converter_create(
    _ inputFormatPtr: UnsafeMutableRawPointer,
    _ outputFormatPtr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let inputFormat = Unmanaged<AVAudioFormat>.fromOpaque(inputFormatPtr).takeUnretainedValue()
    let outputFormat = Unmanaged<AVAudioFormat>.fromOpaque(outputFormatPtr).takeUnretainedValue()
    guard let converter = AVAudioConverter(from: inputFormat, to: outputFormat) else {
        outError?.pointee = ffiString("failed to create AVAudioConverter")
        return nil
    }
    return Unmanaged.passRetained(AudioConverterBox(converter: converter)).toOpaque()
}

@_cdecl("av_audio_converter_release")
public func av_audio_converter_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AudioConverterBox>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_converter_info_json")
public func av_audio_converter_info_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let box = Unmanaged<AudioConverterBox>.fromOpaque(ptr).takeUnretainedValue()
    let payload = ConverterInfoPayload(
        inputFormat: avaEncodeFormatInfo(box.converter.inputFormat),
        outputFormat: avaEncodeFormatInfo(box.converter.outputFormat)
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_converter_convert_buffer")
public func av_audio_converter_convert_buffer(
    _ ptr: UnsafeMutableRawPointer,
    _ inputBufferPtr: UnsafeMutableRawPointer,
    _ outputBufferPtr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let box = Unmanaged<AudioConverterBox>.fromOpaque(ptr).takeUnretainedValue()
    let inputBuffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(inputBufferPtr).takeUnretainedValue()
    let outputBuffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(outputBufferPtr).takeUnretainedValue()
    var inputConsumed = false
    var nsError: NSError?
    outputBuffer.frameLength = 0
    let status = box.converter.convert(to: outputBuffer, error: &nsError) { _, outStatus in
        if inputConsumed {
            outStatus.pointee = .noDataNow
            return nil
        }
        inputConsumed = true
        outStatus.pointee = .haveData
        return inputBuffer
    }
    if let nsError {
        outError?.pointee = ffiString(nsError.localizedDescription)
        return AVA_OPERATION_FAILED
    }
    switch status {
    case .haveData, .inputRanDry, .endOfStream:
        return AVA_OK
    case .error:
        outError?.pointee = ffiString("converter failed without a detailed NSError")
        return AVA_OPERATION_FAILED
    @unknown default:
        outError?.pointee = ffiString("converter returned an unknown output status")
        return AVA_OPERATION_FAILED
    }
}
