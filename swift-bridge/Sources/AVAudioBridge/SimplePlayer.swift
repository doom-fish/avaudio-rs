import AVFoundation
import Foundation

final class AudioSimplePlayerDelegateBox: NSObject, AVAudioPlayerDelegate {
    let finishCallback: AVABoolCallback?
    let decodeErrorCallback: AVAStringCallback?
    let userData: UnsafeMutableRawPointer?
    let dropUserData: AVADropCallback?

    init(
        finishCallback: AVABoolCallback?,
        decodeErrorCallback: AVAStringCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) {
        self.finishCallback = finishCallback
        self.decodeErrorCallback = decodeErrorCallback
        self.userData = userData
        self.dropUserData = dropUserData
    }

    deinit {
        if let userData, let dropUserData {
            dropUserData(userData)
        }
    }

    func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
        finishCallback?(userData, flag)
    }

    func audioPlayerDecodeErrorDidOccur(_ player: AVAudioPlayer, error: Error?) {
        let message = error.flatMap { ffiString($0.localizedDescription) }
        decodeErrorCallback?(userData, message)
    }
}

final class AudioSimplePlayerDelegateHub: NSObject, AVAudioPlayerDelegate {
    private let lock = NSLock()
    private var handler: AVAudioPlayerDelegate?
    private var subscribers: [AVAudioPlayerDelegate] = []

    func setHandler(_ newHandler: AVAudioPlayerDelegate?) {
        lock.lock()
        let previous = handler
        handler = newHandler
        lock.unlock()
        withExtendedLifetime(previous) {}
    }

    func add(_ subscriber: AVAudioPlayerDelegate) {
        lock.lock()
        subscribers.append(subscriber)
        lock.unlock()
    }

    func remove(_ subscriber: AVAudioPlayerDelegate) {
        lock.lock()
        let removed = subscribers.firstIndex { $0 === subscriber }.map { subscribers.remove(at: $0) }
        lock.unlock()
        withExtendedLifetime(removed) {}
    }

    private func listeners() -> [AVAudioPlayerDelegate] {
        lock.lock()
        defer { lock.unlock() }
        return (handler.map { [$0] } ?? []) + subscribers
    }

    func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
        for listener in listeners() {
            listener.audioPlayerDidFinishPlaying?(player, successfully: flag)
        }
    }

    func audioPlayerDecodeErrorDidOccur(_ player: AVAudioPlayer, error: Error?) {
        for listener in listeners() {
            listener.audioPlayerDecodeErrorDidOccur?(player, error: error)
        }
    }
}

final class AudioSimplePlayerBox {
    var player: AVAudioPlayer?
    let hub = AudioSimplePlayerDelegateHub()

    init(url: URL) throws {
        self.player = try AVAudioPlayer(contentsOf: url)
        self.player?.enableRate = true
        self.player?.delegate = hub
    }
}

@_cdecl("av_audio_simple_player_create_from_path")
public func av_audio_simple_player_create_from_path(
    _ pathPtr: UnsafePointer<CChar>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let path = String(cString: pathPtr)
    do {
        let box = try AudioSimplePlayerBox(url: URL(fileURLWithPath: path))
        return Unmanaged.passRetained(box).toOpaque()
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_simple_player_release")
public func av_audio_simple_player_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_simple_player_set_delegate")
public func av_audio_simple_player_set_delegate(
    _ ptr: UnsafeMutableRawPointer,
    _ finishCallback: AVABoolCallback?,
    _ decodeErrorCallback: AVAStringCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    let delegate = AudioSimplePlayerDelegateBox(
        finishCallback: finishCallback,
        decodeErrorCallback: decodeErrorCallback,
        userData: userData,
        dropUserData: dropUserData
    )
    box.hub.setHandler(delegate)
    return AVA_OK
}

@_cdecl("av_audio_simple_player_clear_delegate")
public func av_audio_simple_player_clear_delegate(_ ptr: UnsafeMutableRawPointer) {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    box.hub.setHandler(nil)
}

@_cdecl("av_audio_simple_player_play")
public func av_audio_simple_player_play(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    return box.player?.play() ?? false
}

@_cdecl("av_audio_simple_player_pause")
public func av_audio_simple_player_pause(_ ptr: UnsafeMutableRawPointer) {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    box.player?.pause()
}

@_cdecl("av_audio_simple_player_stop")
public func av_audio_simple_player_stop(_ ptr: UnsafeMutableRawPointer) {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    box.player?.stop()
}

@_cdecl("av_audio_simple_player_get_volume")
public func av_audio_simple_player_get_volume(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue().player?.volume ?? 0
}

@_cdecl("av_audio_simple_player_set_volume")
public func av_audio_simple_player_set_volume(_ ptr: UnsafeMutableRawPointer, _ volume: Float) {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    box.player?.volume = volume
}

@_cdecl("av_audio_simple_player_get_pan")
public func av_audio_simple_player_get_pan(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue().player?.pan ?? 0
}

@_cdecl("av_audio_simple_player_set_pan")
public func av_audio_simple_player_set_pan(_ ptr: UnsafeMutableRawPointer, _ pan: Float) {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    box.player?.pan = pan
}

@_cdecl("av_audio_simple_player_get_rate")
public func av_audio_simple_player_get_rate(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue().player?.rate ?? 0
}

@_cdecl("av_audio_simple_player_set_rate")
public func av_audio_simple_player_set_rate(_ ptr: UnsafeMutableRawPointer, _ rate: Float) {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    box.player?.enableRate = true
    box.player?.rate = rate
}

@_cdecl("av_audio_simple_player_get_duration")
public func av_audio_simple_player_get_duration(_ ptr: UnsafeMutableRawPointer) -> Double {
    Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue().player?.duration ?? 0
}

@_cdecl("av_audio_simple_player_get_current_time")
public func av_audio_simple_player_get_current_time(_ ptr: UnsafeMutableRawPointer) -> Double {
    Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue().player?.currentTime ?? 0
}

@_cdecl("av_audio_simple_player_set_current_time")
public func av_audio_simple_player_set_current_time(_ ptr: UnsafeMutableRawPointer, _ time: Double) {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    box.player?.currentTime = time
}

@_cdecl("av_audio_simple_player_is_playing")
public func av_audio_simple_player_is_playing(_ ptr: UnsafeMutableRawPointer) -> Bool {
    Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue().player?.isPlaying ?? false
}

@_cdecl("av_audio_simple_player_get_number_of_loops")
public func av_audio_simple_player_get_number_of_loops(_ ptr: UnsafeMutableRawPointer) -> Int32 {
    Int32(clamping: Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue().player?.numberOfLoops ?? 0)
}

@_cdecl("av_audio_simple_player_set_number_of_loops")
public func av_audio_simple_player_set_number_of_loops(_ ptr: UnsafeMutableRawPointer, _ loopCount: Int32) {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    box.player?.numberOfLoops = Int(loopCount)
}

@_cdecl("av_audio_simple_player_prepare_to_play")
public func av_audio_simple_player_prepare_to_play(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(ptr).takeUnretainedValue()
    return box.player?.prepareToPlay() ?? false
}
