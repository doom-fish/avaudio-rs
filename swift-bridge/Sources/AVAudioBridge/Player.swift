import AVFoundation
import Foundation

final class TypedCompletionCallbackBox {
    let callback: AVAIntCallback?
    let userData: UnsafeMutableRawPointer?
    let dropUserData: AVADropCallback?
    private var disposed = false

    init(
        callback: AVAIntCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) {
        self.callback = callback
        self.userData = userData
        self.dropUserData = dropUserData
    }

    func fire(_ value: Int64) {
        callback?(userData, value)
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

final class AudioPlayerNodeBox {
    let node = AVAudioPlayerNode()
    private var nextCompletionId = 0
    private var pendingCompletions: [Int: CompletionCallbackBox] = [:]
    private var pendingTypedCompletions: [Int: TypedCompletionCallbackBox] = [:]

    deinit {
        pendingCompletions.values.forEach { $0.dispose() }
        pendingCompletions.removeAll()
        pendingTypedCompletions.values.forEach { $0.dispose() }
        pendingTypedCompletions.removeAll()
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

    func addTypedCompletion(
        callback: AVAIntCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) -> ((AVAudioPlayerNodeCompletionCallbackType) -> Void)? {
        guard callback != nil || dropUserData != nil || userData != nil else {
            return nil
        }
        let id = nextCompletionId
        nextCompletionId += 1
        let box = TypedCompletionCallbackBox(callback: callback, userData: userData, dropUserData: dropUserData)
        pendingTypedCompletions[id] = box
        return { [weak self] value in
            guard let self else {
                box.fire(Int64(value.rawValue))
                return
            }
            self.pendingTypedCompletions.removeValue(forKey: id)?.fire(Int64(value.rawValue))
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

@_cdecl("av_audio_player_node_schedule_buffer_with_options")
public func av_audio_player_node_schedule_buffer_with_options(
    _ playerPtr: UnsafeMutableRawPointer,
    _ bufferPtr: UnsafeMutableRawPointer,
    _ whenPtr: UnsafeMutableRawPointer?,
    _ optionsRaw: UInt,
    _ callback: AVASimpleCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    let when = whenPtr.map { Unmanaged<AVAudioTime>.fromOpaque($0).takeUnretainedValue() }
    let options = AVAudioPlayerNodeBufferOptions(rawValue: optionsRaw)
    let completion = player.addCompletion(callback: callback, userData: userData, dropUserData: dropUserData)
    player.node.scheduleBuffer(buffer, at: when, options: options, completionHandler: completion)
    return AVA_OK
}

@_cdecl("av_audio_player_node_schedule_buffer_with_callback_type")
public func av_audio_player_node_schedule_buffer_with_callback_type(
    _ playerPtr: UnsafeMutableRawPointer,
    _ bufferPtr: UnsafeMutableRawPointer,
    _ whenPtr: UnsafeMutableRawPointer?,
    _ optionsRaw: UInt,
    _ callbackTypeRaw: Int64,
    _ callback: AVAIntCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    let when = whenPtr.map { Unmanaged<AVAudioTime>.fromOpaque($0).takeUnretainedValue() }
    let options = AVAudioPlayerNodeBufferOptions(rawValue: optionsRaw)
    guard let callbackType = AVAudioPlayerNodeCompletionCallbackType(rawValue: Int(callbackTypeRaw)) else {
        outErrorMessage?.pointee = ffiString("invalid AVAudioPlayerNodeCompletionCallbackType")
        return AVA_INVALID_ARGUMENT
    }
    let completion = player.addTypedCompletion(callback: callback, userData: userData, dropUserData: dropUserData)
    player.node.scheduleBuffer(
        buffer,
        at: when,
        options: options,
        completionCallbackType: callbackType,
        completionHandler: completion
    )
    return AVA_OK
}

@_cdecl("av_audio_player_node_schedule_file_with_callback_type")
public func av_audio_player_node_schedule_file_with_callback_type(
    _ playerPtr: UnsafeMutableRawPointer,
    _ filePtr: UnsafeMutableRawPointer,
    _ whenPtr: UnsafeMutableRawPointer?,
    _ callbackTypeRaw: Int64,
    _ callback: AVAIntCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let file = Unmanaged<AVAudioFile>.fromOpaque(filePtr).takeUnretainedValue()
    let when = whenPtr.map { Unmanaged<AVAudioTime>.fromOpaque($0).takeUnretainedValue() }
    guard let callbackType = AVAudioPlayerNodeCompletionCallbackType(rawValue: Int(callbackTypeRaw)) else {
        outErrorMessage?.pointee = ffiString("invalid AVAudioPlayerNodeCompletionCallbackType")
        return AVA_INVALID_ARGUMENT
    }
    let completion = player.addTypedCompletion(callback: callback, userData: userData, dropUserData: dropUserData)
    player.node.scheduleFile(
        file,
        at: when,
        completionCallbackType: callbackType,
        completionHandler: completion
    )
    return AVA_OK
}
