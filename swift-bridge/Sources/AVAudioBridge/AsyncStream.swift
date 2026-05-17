import AVFoundation
import Foundation

public typealias AVAStreamEventCallback = @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void

final class ConfigChangeBridge: NSObject {
    let onEvent: AVAStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    var observer: NSObjectProtocol?

    init(
        enginePtr: UnsafeMutableRawPointer,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        super.init()
        let nc = NotificationCenter.default
        observer = nc.addObserver(
            forName: .AVAudioEngineConfigurationChange,
            object: Unmanaged<AVAudioEngine>.fromOpaque(enginePtr).takeUnretainedValue(),
            queue: nil
        ) { [weak self] _ in
            guard let self else { return }
            self.onEvent(0, nil, self.ctx)
        }
    }

    deinit {
        if let observer {
            NotificationCenter.default.removeObserver(observer)
        }
    }
}

@_cdecl("ava_engine_config_change_subscribe")
public func ava_engine_config_change_subscribe(
    _ enginePtr: UnsafeMutableRawPointer,
    _ onEvent: AVAStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    let bridge = ConfigChangeBridge(enginePtr: enginePtr, onEvent: onEvent, ctx: ctx)
    return Unmanaged.passRetained(bridge).toOpaque()
}

@_cdecl("ava_engine_config_change_unsubscribe")
public func ava_engine_config_change_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<ConfigChangeBridge>.fromOpaque(handle).release()
}

final class PlayerNodeStreamBridge: NSObject {
    let node: AVAudioPlayerNode
    let onEvent: AVAStreamEventCallback
    let ctx: UnsafeMutableRawPointer

    init(
        playerBoxPtr: UnsafeMutableRawPointer,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer
    ) {
        self.node = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerBoxPtr).takeUnretainedValue().node
        self.onEvent = onEvent
        self.ctx = ctx
        super.init()
    }

    func scheduleBuffer(bufferPtr: UnsafeMutableRawPointer, options: UInt) {
        let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
        let opts = AVAudioPlayerNodeBufferOptions(rawValue: options)
        node.scheduleBuffer(buffer, at: nil, options: opts, completionCallbackType: .dataPlayedBack) { [weak self] cbType in
            guard let self else { return }
            self.onEvent(Int32(cbType.rawValue), nil, self.ctx)
        }
    }

    func scheduleFile(filePtr: UnsafeMutableRawPointer) {
        let file = Unmanaged<AVAudioFile>.fromOpaque(filePtr).takeUnretainedValue()
        node.scheduleFile(file, at: nil, completionCallbackType: .dataPlayedBack) { [weak self] cbType in
            guard let self else { return }
            self.onEvent(Int32(cbType.rawValue), nil, self.ctx)
        }
    }
}

@_cdecl("ava_player_node_stream_subscribe")
public func ava_player_node_stream_subscribe(
    _ playerBoxPtr: UnsafeMutableRawPointer,
    _ onEvent: AVAStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    let bridge = PlayerNodeStreamBridge(playerBoxPtr: playerBoxPtr, onEvent: onEvent, ctx: ctx)
    return Unmanaged.passRetained(bridge).toOpaque()
}

@_cdecl("ava_player_node_stream_schedule_buffer")
public func ava_player_node_stream_schedule_buffer(
    _ handle: UnsafeMutableRawPointer,
    _ bufferPtr: UnsafeMutableRawPointer,
    _ options: UInt,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let bridge = Unmanaged<PlayerNodeStreamBridge>.fromOpaque(handle).takeUnretainedValue()
    bridge.scheduleBuffer(bufferPtr: bufferPtr, options: options)
    _ = outError
    return AVA_OK
}

@_cdecl("ava_player_node_stream_schedule_file")
public func ava_player_node_stream_schedule_file(
    _ handle: UnsafeMutableRawPointer,
    _ filePtr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let bridge = Unmanaged<PlayerNodeStreamBridge>.fromOpaque(handle).takeUnretainedValue()
    bridge.scheduleFile(filePtr: filePtr)
    _ = outError
    return AVA_OK
}

@_cdecl("ava_player_node_stream_unsubscribe")
public func ava_player_node_stream_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<PlayerNodeStreamBridge>.fromOpaque(handle).release()
}

final class RecorderStreamBridge: NSObject, AVAudioRecorderDelegate {
    let onEvent: AVAStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    weak var recorder: AVAudioRecorder?

    init(
        recorderBoxPtr: UnsafeMutableRawPointer,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        let box = Unmanaged<AudioRecorderBox>.fromOpaque(recorderBoxPtr).takeUnretainedValue()
        self.recorder = box.recorder
        super.init()
        box.delegateBox = nil
        box.recorder?.delegate = self
    }

    deinit {
        if (recorder?.delegate as AnyObject?) === self {
            recorder?.delegate = nil
        }
    }

    func audioRecorderDidFinishRecording(_ recorder: AVAudioRecorder, successfully flag: Bool) {
        onEvent(flag ? 1 : 0, nil, ctx)
    }

    func audioRecorderEncodeErrorDidOccur(_ recorder: AVAudioRecorder, error: Error?) {
        if let error {
            error.localizedDescription.withCString { ptr in
                onEvent(2, UnsafeRawPointer(ptr), ctx)
            }
        } else {
            onEvent(2, nil, ctx)
        }
    }
}

@_cdecl("ava_recorder_stream_subscribe")
public func ava_recorder_stream_subscribe(
    _ recorderBoxPtr: UnsafeMutableRawPointer,
    _ onEvent: AVAStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    let bridge = RecorderStreamBridge(recorderBoxPtr: recorderBoxPtr, onEvent: onEvent, ctx: ctx)
    return Unmanaged.passRetained(bridge).toOpaque()
}

@_cdecl("ava_recorder_stream_unsubscribe")
public func ava_recorder_stream_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<RecorderStreamBridge>.fromOpaque(handle).release()
}

final class SimplePlayerStreamBridge: NSObject, AVAudioPlayerDelegate {
    let onEvent: AVAStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    weak var player: AVAudioPlayer?

    init(
        playerBoxPtr: UnsafeMutableRawPointer,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        let box = Unmanaged<AudioSimplePlayerBox>.fromOpaque(playerBoxPtr).takeUnretainedValue()
        self.player = box.player
        super.init()
        box.delegateBox = nil
        box.player?.delegate = self
    }

    deinit {
        if (player?.delegate as AnyObject?) === self {
            player?.delegate = nil
        }
    }

    func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
        onEvent(flag ? 1 : 0, nil, ctx)
    }

    func audioPlayerDecodeErrorDidOccur(_ player: AVAudioPlayer, error: Error?) {
        if let error {
            error.localizedDescription.withCString { ptr in
                onEvent(2, UnsafeRawPointer(ptr), ctx)
            }
        } else {
            onEvent(2, nil, ctx)
        }
    }
}

@_cdecl("ava_simple_player_stream_subscribe")
public func ava_simple_player_stream_subscribe(
    _ playerBoxPtr: UnsafeMutableRawPointer,
    _ onEvent: AVAStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    let bridge = SimplePlayerStreamBridge(playerBoxPtr: playerBoxPtr, onEvent: onEvent, ctx: ctx)
    return Unmanaged.passRetained(bridge).toOpaque()
}

@_cdecl("ava_simple_player_stream_unsubscribe")
public func ava_simple_player_stream_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<SimplePlayerStreamBridge>.fromOpaque(handle).release()
}

struct TapEventPayload {
    var frameLength: UInt32
    var channelCount: UInt32
    var sampleRate: Double
}

final class TapBridge: NSObject {
    let node: AVAudioNode
    let bus: AVAudioNodeBus
    let onEvent: AVAStreamEventCallback
    let ctx: UnsafeMutableRawPointer

    init(
        nodePtr: UnsafeMutableRawPointer,
        bus: Int,
        bufferSize: UInt32,
        formatPtr: UnsafeMutableRawPointer?,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer
    ) {
        self.node = Unmanaged<AVAudioNode>.fromOpaque(nodePtr).takeUnretainedValue()
        self.bus = AVAudioNodeBus(bus)
        self.onEvent = onEvent
        self.ctx = ctx
        super.init()
        let format = formatPtr.map { Unmanaged<AVAudioFormat>.fromOpaque($0).takeUnretainedValue() }
        node.removeTap(onBus: self.bus)
        node.installTap(onBus: self.bus, bufferSize: AVAudioFrameCount(bufferSize), format: format) { [weak self] buffer, _ in
            guard let self else { return }
            var payload = TapEventPayload(
                frameLength: buffer.frameLength,
                channelCount: buffer.format.channelCount,
                sampleRate: buffer.format.sampleRate
            )
            withUnsafePointer(to: &payload) { ptr in
                self.onEvent(0, UnsafeRawPointer(ptr), self.ctx)
            }
        }
    }

    deinit {
        node.removeTap(onBus: bus)
    }
}

@_cdecl("ava_node_tap_subscribe")
public func ava_node_tap_subscribe(
    _ nodePtr: UnsafeMutableRawPointer,
    _ bus: Int,
    _ bufferSize: UInt32,
    _ formatPtr: UnsafeMutableRawPointer?,
    _ onEvent: AVAStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer {
    let bridge = TapBridge(
        nodePtr: nodePtr,
        bus: bus,
        bufferSize: bufferSize,
        formatPtr: formatPtr,
        onEvent: onEvent,
        ctx: ctx
    )
    return Unmanaged.passRetained(bridge).toOpaque()
}

@_cdecl("ava_node_tap_unsubscribe")
public func ava_node_tap_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<TapBridge>.fromOpaque(handle).release()
}
