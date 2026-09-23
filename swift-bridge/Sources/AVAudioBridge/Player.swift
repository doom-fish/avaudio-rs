import AVAudioObjCBridge
import AVFoundation
import Foundation

final class PlayerCompletionBox {
    private let simpleCallback: AVASimpleCallback?
    private let typedCallback: AVAIntCallback?
    private let userData: UnsafeMutableRawPointer?
    private let dropUserData: AVADropCallback?
    private let lock = NSLock()
    private var fired = false

    init(
        simpleCallback: AVASimpleCallback?,
        typedCallback: AVAIntCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) {
        self.simpleCallback = simpleCallback
        self.typedCallback = typedCallback
        self.userData = userData
        self.dropUserData = dropUserData
    }

    func fire(_ type: AVAudioPlayerNodeCompletionCallbackType) {
        lock.lock()
        let first = !fired
        fired = true
        lock.unlock()
        guard first else { return }
        if let typedCallback {
            typedCallback(userData, Int64(type.rawValue))
        } else {
            simpleCallback?(userData)
        }
    }

    deinit {
        if let userData, let dropUserData {
            dropUserData(userData)
        }
    }
}

func avaPlayerCompletionBox(
    simpleCallback: AVASimpleCallback?,
    typedCallback: AVAIntCallback?,
    userData: UnsafeMutableRawPointer?,
    dropUserData: AVADropCallback?
) -> PlayerCompletionBox? {
    guard simpleCallback != nil || typedCallback != nil || userData != nil || dropUserData != nil else {
        return nil
    }
    return PlayerCompletionBox(
        simpleCallback: simpleCallback,
        typedCallback: typedCallback,
        userData: userData,
        dropUserData: dropUserData
    )
}

func avaScheduleBuffer(
    _ node: AVAudioPlayerNode,
    _ buffer: AVAudioPCMBuffer,
    _ when: AVAudioTime?,
    _ options: AVAudioPlayerNodeBufferOptions,
    _ callbackType: AVAudioPlayerNodeCompletionCallbackType,
    _ completion: ((AVAudioPlayerNodeCompletionCallbackType) -> Void)?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    var error: NSError?
    guard AVAXPlayerScheduleBuffer(node, buffer, when, options, callbackType, completion, &error) else {
        avaReportObjCFailure("AVAudioPlayerNode.scheduleBuffer", error, outErrorMessage)
        return AVA_PLAYER_ERROR
    }
    return AVA_OK
}

func avaScheduleFile(
    _ node: AVAudioPlayerNode,
    _ file: AVAudioFile,
    _ when: AVAudioTime?,
    _ callbackType: AVAudioPlayerNodeCompletionCallbackType,
    _ completion: ((AVAudioPlayerNodeCompletionCallbackType) -> Void)?,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    var error: NSError?
    guard AVAXPlayerScheduleFile(node, file, when, callbackType, completion, &error) else {
        avaReportObjCFailure("AVAudioPlayerNode.scheduleFile", error, outErrorMessage)
        return AVA_PLAYER_ERROR
    }
    return AVA_OK
}

private func avaFire(_ box: PlayerCompletionBox?) -> ((AVAudioPlayerNodeCompletionCallbackType) -> Void)? {
    guard let box else { return nil }
    return { type in box.fire(type) }
}

final class AudioPlayerNodeBox {
    let node = AVAudioPlayerNode()
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
public func av_audio_player_node_play(
    _ playerPtr: UnsafeMutableRawPointer,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    var error: NSError?
    guard AVAXPlayerPlay(player.node, &error) else {
        avaReportObjCFailure("AVAudioPlayerNode.play", error, outErrorMessage)
        return AVA_PLAYER_ERROR
    }
    return AVA_OK
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
    let completion = avaPlayerCompletionBox(
        simpleCallback: callback,
        typedCallback: nil,
        userData: userData,
        dropUserData: dropUserData
    )
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    return avaScheduleBuffer(player.node, buffer, nil, [], .dataConsumed, avaFire(completion), outErrorMessage)
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
    let completion = avaPlayerCompletionBox(
        simpleCallback: callback,
        typedCallback: nil,
        userData: userData,
        dropUserData: dropUserData
    )
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let file = Unmanaged<AVAudioFile>.fromOpaque(filePtr).takeUnretainedValue()
    return avaScheduleFile(player.node, file, nil, .dataConsumed, avaFire(completion), outErrorMessage)
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
    let completion = avaPlayerCompletionBox(
        simpleCallback: callback,
        typedCallback: nil,
        userData: userData,
        dropUserData: dropUserData
    )
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    let when = whenPtr.map { Unmanaged<AVAudioTime>.fromOpaque($0).takeUnretainedValue() }
    let options = AVAudioPlayerNodeBufferOptions(rawValue: optionsRaw)
    return avaScheduleBuffer(player.node, buffer, when, options, .dataConsumed, avaFire(completion), outErrorMessage)
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
    let completion = avaPlayerCompletionBox(
        simpleCallback: nil,
        typedCallback: callback,
        userData: userData,
        dropUserData: dropUserData
    )
    guard let callbackType = AVAudioPlayerNodeCompletionCallbackType(rawValue: Int(callbackTypeRaw)) else {
        outErrorMessage?.pointee = ffiString("invalid AVAudioPlayerNodeCompletionCallbackType")
        return AVA_INVALID_ARGUMENT
    }
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    let when = whenPtr.map { Unmanaged<AVAudioTime>.fromOpaque($0).takeUnretainedValue() }
    let options = AVAudioPlayerNodeBufferOptions(rawValue: optionsRaw)
    return avaScheduleBuffer(player.node, buffer, when, options, callbackType, avaFire(completion), outErrorMessage)
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
    let completion = avaPlayerCompletionBox(
        simpleCallback: nil,
        typedCallback: callback,
        userData: userData,
        dropUserData: dropUserData
    )
    guard let callbackType = AVAudioPlayerNodeCompletionCallbackType(rawValue: Int(callbackTypeRaw)) else {
        outErrorMessage?.pointee = ffiString("invalid AVAudioPlayerNodeCompletionCallbackType")
        return AVA_INVALID_ARGUMENT
    }
    let player = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerPtr).takeUnretainedValue()
    let file = Unmanaged<AVAudioFile>.fromOpaque(filePtr).takeUnretainedValue()
    let when = whenPtr.map { Unmanaged<AVAudioTime>.fromOpaque($0).takeUnretainedValue() }
    return avaScheduleFile(player.node, file, when, callbackType, avaFire(completion), outErrorMessage)
}
