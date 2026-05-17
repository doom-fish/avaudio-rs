import AVFoundation
import Foundation

let AVA_OK: Int32 = 0
let AVA_INVALID_ARGUMENT: Int32 = -1
let AVA_FORMAT_ERROR: Int32 = -2
let AVA_FILE_ERROR: Int32 = -3
let AVA_ENGINE_ERROR: Int32 = -4
let AVA_PLAYER_ERROR: Int32 = -5
let AVA_CALLBACK_ERROR: Int32 = -6
let AVA_OPERATION_FAILED: Int32 = -7

public typealias AVASimpleCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias AVADropCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias AVABoolCallback = @convention(c) (UnsafeMutableRawPointer?, Bool) -> Void
public typealias AVASourceNodeRenderCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<Bool>?,
    UnsafeRawPointer?,
    UInt32,
    UnsafeMutableRawPointer?
) -> Int32
public typealias AVASinkNodeReceiverCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeRawPointer?,
    UInt32,
    UnsafeRawPointer?
) -> Int32
public typealias AVASequencerUserCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<UInt8>?,
    Int,
    Double
) -> Void
public typealias AVAMusicTrackEnumerationCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    Double,
    UnsafeMutablePointer<Double>?,
    UnsafeMutablePointer<Bool>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32

@_cdecl("ava_string_free")
public func ava_string_free(_ str: UnsafeMutablePointer<CChar>?) {
    guard let str else { return }
    free(str)
}

@_cdecl("ava_buffer_free")
public func ava_buffer_free(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    free(ptr)
}

@_cdecl("av_audio_node_release")
public func av_audio_node_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioNode>.fromOpaque(ptr).release()
}

func ffiString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

enum BridgeError: LocalizedError {
    case message(String)

    var errorDescription: String? {
        switch self {
        case .message(let message):
            return message
        }
    }
}

struct AudioFormatInfoPayload: Codable {
    let commonFormat: Int32
    let sampleRate: Double
    let channelCount: UInt32
    let isInterleaved: Bool
}

struct AudioFileInfoPayload: Codable {
    let lengthFrames: Int64
    let processingFormat: AudioFormatInfoPayload
    let fileFormat: AudioFormatInfoPayload
}

struct PCMBufferInfoPayload: Codable {
    let frameCapacity: UInt32
    let frameLength: UInt32
    let format: AudioFormatInfoPayload
}

struct AudioEngineInfoPayload: Codable {
    let isRunning: Bool
}

struct AudioPlayerNodeInfoPayload: Codable {
    let isPlaying: Bool
}

func avaEncodeJSON<T: Encodable>(_ value: T) throws -> String {
    let data = try JSONEncoder().encode(value)
    guard let string = String(data: data, encoding: .utf8) else {
        throw BridgeError.message("failed to UTF-8 encode JSON payload")
    }
    return string
}

func avaEncodeFormatInfo(_ format: AVAudioFormat) -> AudioFormatInfoPayload {
    AudioFormatInfoPayload(
        commonFormat: Int32(format.commonFormat.rawValue),
        sampleRate: format.sampleRate,
        channelCount: format.channelCount,
        isInterleaved: format.isInterleaved
    )
}

final class CompletionCallbackBox {
    let callback: AVASimpleCallback?
    let userData: UnsafeMutableRawPointer?
    let dropUserData: AVADropCallback?
    private var disposed = false

    init(
        callback: AVASimpleCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) {
        self.callback = callback
        self.userData = userData
        self.dropUserData = dropUserData
    }

    func fire() {
        callback?(userData)
        dispose()
    }

    func dispose() {
        guard !disposed else { return }
        disposed = true
        if let userData, let dropUserData {
            dropUserData(userData)
        }
    }
}
