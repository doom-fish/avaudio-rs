import AVFoundation
import Foundation

final class AudioPlayerNodeBox {
    let node = AVAudioPlayerNode()
    private var nextCompletionId = 0
    private var pendingCompletions: [Int: CompletionCallbackBox] = [:]

    deinit {
        pendingCompletions.values.forEach { $0.dispose() }
        pendingCompletions.removeAll()
    }

    func addCompletion(
        callback: AVASimpleCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) -> (() -> Void)? {
        guard callback != nil || dropUserData != nil || userData != nil else {
            return nil
        }
        let id = nextCompletionId
        nextCompletionId += 1
        let box = CompletionCallbackBox(callback: callback, userData: userData, dropUserData: dropUserData)
        pendingCompletions[id] = box
        return { [weak self] in
            guard let self else {
                box.fire()
                return
            }
            self.pendingCompletions.removeValue(forKey: id)?.fire()
        }
    }
}

@_cdecl("av_audio_player_node_create")
public func av_audio_player_node_create(
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AudioPlayerNodeBox()).toOpaque()
}

@_cdecl("av_audio_player_node_release")
public func av_audio_player_node_release(_ playerPtr: UnsafeMutableRawPointer?) {
    guard let playerPtr else { return }
    Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).release()
}

@_cdecl("av_audio_player_node_get_node_unretained")
public func av_audio_player_node_get_node_unretained(
    _ playerPtr: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    return Unmanaged.passUnretained(player.node).toOpaque()
}

@_cdecl("av_audio_player_node_info_json")
public func av_audio_player_node_info_json(
    _ playerPtr: UnsafeMutableRawPointer,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(AudioPlayerNodeInfoPayload(isPlaying: player.node.isPlaying)))
    } catch {
        outErrorMessage?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_player_node_play")
public func av_audio_player_node_play(_ playerPtr: UnsafeMutableRawPointer) {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    player.node.play()
}

@_cdecl("av_audio_player_node_pause")
public func av_audio_player_node_pause(_ playerPtr: UnsafeMutableRawPointer) {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    player.node.pause()
}

@_cdecl("av_audio_player_node_stop")
public func av_audio_player_node_stop(_ playerPtr: UnsafeMutableRawPointer) {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    player.node.stop()
}

@_cdecl("av_audio_player_node_schedule_buffer")
public func av_audio_player_node_schedule_buffer(
    _ playerPtr: UnsafeMutableRawPointer,
    _ bufferPtr: UnsafeMutableRawPointer,
    _ callback: AVASimpleCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    let completion = player.addCompletion(callback: callback, userData: userData, dropUserData: dropUserData)
    player.node.scheduleBuffer(buffer, completionHandler: completion)
    return AVA_OK
}

@_cdecl("av_audio_player_node_schedule_file")
public func av_audio_player_node_schedule_file(
    _ playerPtr: UnsafeMutableRawPointer,
    _ filePtr: UnsafeMutableRawPointer,
    _ callback: AVASimpleCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let file = Unmanaged<AVAudioFile>.fromOpaque(filePtr).takeUnretainedValue()
    let completion = player.addCompletion(callback: callback, userData: userData, dropUserData: dropUserData)
    player.node.scheduleFile(file, at: nil, completionHandler: completion)
    return AVA_OK
}
