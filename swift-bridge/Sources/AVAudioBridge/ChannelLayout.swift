import AVFoundation
import CoreAudioTypes
import Foundation

struct AudioChannelLayoutInfoPayload: Codable {
    let layoutTag: UInt32
    let channelCount: UInt32
}

@_cdecl("av_audio_channel_layout_create_with_layout_tag")
public func av_audio_channel_layout_create_with_layout_tag(
    _ layoutTag: UInt32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let layout = AVAudioChannelLayout(layoutTag: AudioChannelLayoutTag(layoutTag)) else {
        outError?.pointee = ffiString("failed to create AVAudioChannelLayout")
        return nil
    }
    return Unmanaged.passRetained(layout).toOpaque()
}

@_cdecl("av_audio_channel_layout_release")
public func av_audio_channel_layout_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioChannelLayout>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_channel_layout_info_json")
public func av_audio_channel_layout_info_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let layout = Unmanaged<AVAudioChannelLayout>.fromOpaque(ptr).takeUnretainedValue()
    let payload = AudioChannelLayoutInfoPayload(
        layoutTag: layout.layoutTag,
        channelCount: layout.channelCount
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_channel_layout_is_equal")
public func av_audio_channel_layout_is_equal(
    _ lhsPtr: UnsafeMutableRawPointer,
    _ rhsPtr: UnsafeMutableRawPointer
) -> Bool {
    let lhs = Unmanaged<AVAudioChannelLayout>.fromOpaque(lhsPtr).takeUnretainedValue()
    let rhs = Unmanaged<AVAudioChannelLayout>.fromOpaque(rhsPtr).takeUnretainedValue()
    return lhs.isEqual(rhs)
}
