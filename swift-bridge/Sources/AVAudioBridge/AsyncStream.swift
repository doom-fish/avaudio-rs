import AVAudioObjCBridge
import AVFoundation
import Foundation

public typealias AVAStreamEventCallback = @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void

final class ConfigChangeBridge: NSObject {
    let onEvent: AVAStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    let releaseContext: AVADropCallback
    var observer: NSObjectProtocol?

    init(
        enginePtr: UnsafeMutableRawPointer,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer,
        releaseContext: @escaping AVADropCallback
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        self.releaseContext = releaseContext
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
        releaseContext(ctx)
    }
}

@_cdecl("ava_engine_config_change_subscribe")
public func ava_engine_config_change_subscribe(
    _ enginePtr: UnsafeMutableRawPointer,
    _ onEvent: AVAStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ releaseContext: AVADropCallback
) -> UnsafeMutableRawPointer {
    let bridge = ConfigChangeBridge(
        enginePtr: enginePtr,
        onEvent: onEvent,
        ctx: ctx,
        releaseContext: releaseContext
    )
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
    let releaseContext: AVADropCallback

    init(
        playerBoxPtr: UnsafeMutableRawPointer,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer,
        releaseContext: @escaping AVADropCallback
    ) {
        self.node = Unmanaged<AudioPlayerNodeBox>.fromOpaque(playerBoxPtr).takeUnretainedValue().node
        self.onEvent = onEvent
        self.ctx = ctx
        self.releaseContext = releaseContext
        super.init()
    }

    deinit {
        releaseContext(ctx)
    }

    func completion() -> (AVAudioPlayerNodeCompletionCallbackType) -> Void {
        { [weak self] cbType in
            guard let self else { return }
            self.onEvent(Int32(cbType.rawValue), nil, self.ctx)
        }
    }
}

@_cdecl("ava_player_node_stream_subscribe")
public func ava_player_node_stream_subscribe(
    _ playerBoxPtr: UnsafeMutableRawPointer,
    _ onEvent: AVAStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ releaseContext: AVADropCallback
) -> UnsafeMutableRawPointer {
    let bridge = PlayerNodeStreamBridge(
        playerBoxPtr: playerBoxPtr,
        onEvent: onEvent,
        ctx: ctx,
        releaseContext: releaseContext
    )
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
    let buffer = Unmanaged<AVAudioPCMBuffer>.fromOpaque(bufferPtr).takeUnretainedValue()
    return avaScheduleBuffer(
        bridge.node,
        buffer,
        nil,
        AVAudioPlayerNodeBufferOptions(rawValue: options),
        .dataPlayedBack,
        bridge.completion(),
        outError
    )
}

@_cdecl("ava_player_node_stream_schedule_file")
public func ava_player_node_stream_schedule_file(
    _ handle: UnsafeMutableRawPointer,
    _ filePtr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let bridge = Unmanaged<PlayerNodeStreamBridge>.fromOpaque(handle).takeUnretainedValue()
    let file = Unmanaged<AVAudioFile>.fromOpaque(filePtr).takeUnretainedValue()
    return avaScheduleFile(bridge.node, file, nil, .dataPlayedBack, bridge.completion(), outError)
}

@_cdecl("ava_player_node_stream_unsubscribe")
public func ava_player_node_stream_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<PlayerNodeStreamBridge>.fromOpaque(handle).release()
}

final class RecorderStreamBridge: NSObject, AVAudioRecorderDelegate {
    let onEvent: AVAStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    let releaseContext: AVADropCallback
    weak var recorder: AVAudioRecorder?

    init(
        recorderBoxPtr: UnsafeMutableRawPointer,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer,
        releaseContext: @escaping AVADropCallback
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        self.releaseContext = releaseContext
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
        releaseContext(ctx)
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
    _ ctx: UnsafeMutableRawPointer,
    _ releaseContext: AVADropCallback
) -> UnsafeMutableRawPointer {
    let bridge = RecorderStreamBridge(
        recorderBoxPtr: recorderBoxPtr,
        onEvent: onEvent,
        ctx: ctx,
        releaseContext: releaseContext
    )
    return Unmanaged.passRetained(bridge).toOpaque()
}

@_cdecl("ava_recorder_stream_unsubscribe")
public func ava_recorder_stream_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<RecorderStreamBridge>.fromOpaque(handle).release()
}

final class SimplePlayerStreamBridge: NSObject, AVAudioPlayerDelegate {
    let onEvent: AVAStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    let releaseContext: AVADropCallback
    weak var player: AVAudioPlayer?

    init(
        playerBoxPtr: UnsafeMutableRawPointer,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer,
        releaseContext: @escaping AVADropCallback
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        self.releaseContext = releaseContext
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
        releaseContext(ctx)
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
    _ ctx: UnsafeMutableRawPointer,
    _ releaseContext: AVADropCallback
) -> UnsafeMutableRawPointer {
    let bridge = SimplePlayerStreamBridge(
        playerBoxPtr: playerBoxPtr,
        onEvent: onEvent,
        ctx: ctx,
        releaseContext: releaseContext
    )
    return Unmanaged.passRetained(bridge).toOpaque()
}

@_cdecl("ava_simple_player_stream_unsubscribe")
public func ava_simple_player_stream_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<SimplePlayerStreamBridge>.fromOpaque(handle).release()
}

final class MutedSpeechActivityStreamBridge: NSObject {
    let node: AVAudioInputNode
    let onEvent: AVAStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    private let stateLock = NSLock()
    private var active = true

    init?(
        nodePtr: UnsafeMutableRawPointer,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer,
        outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
    ) {
        self.node = Unmanaged<AVAudioInputNode>.fromOpaque(nodePtr).takeUnretainedValue()
        self.onEvent = onEvent
        self.ctx = ctx
        super.init()

        guard #available(macOS 14.0, *) else {
            outError?.pointee = ffiString("muted speech activity listener requires macOS 14.0")
            return nil
        }

        let ok = node.setMutedSpeechActivityEventListener { [weak self] event in
            guard let self else { return }
            self.stateLock.lock()
            defer { self.stateLock.unlock() }
            guard self.active else { return }
            self.onEvent(Int32(event.rawValue), nil, self.ctx)
        }
        if !ok {
            outError?.pointee = ffiString("failed to install muted speech activity listener")
            return nil
        }
    }

    func cancel() {
        stateLock.lock()
        let wasActive = active
        active = false
        stateLock.unlock()

        guard wasActive else { return }
        if #available(macOS 14.0, *) {
            _ = node.setMutedSpeechActivityEventListener(nil)
        }
    }

    deinit {
        cancel()
    }
}

@_cdecl("ava_input_node_speech_activity_subscribe")
public func ava_input_node_speech_activity_subscribe(
    _ nodePtr: UnsafeMutableRawPointer,
    _ onEvent: AVAStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let bridge = MutedSpeechActivityStreamBridge(
        nodePtr: nodePtr,
        onEvent: onEvent,
        ctx: ctx,
        outError: outError
    )
    return bridge.map { Unmanaged.passRetained($0).toOpaque() }
}

@_cdecl("ava_input_node_speech_activity_unsubscribe")
public func ava_input_node_speech_activity_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    let bridge = Unmanaged<MutedSpeechActivityStreamBridge>.fromOpaque(handle).takeUnretainedValue()
    bridge.cancel()
    Unmanaged<MutedSpeechActivityStreamBridge>.fromOpaque(handle).release()
}

final class TapBridge: NSObject {
    let node: AVAudioNode
    let bus: UInt32
    let onEvent: AVAStreamEventCallback
    let ctx: UnsafeMutableRawPointer
    let releaseContext: AVADropCallback
    private var installed = false

    init(
        node: AVAudioNode,
        bus: UInt32,
        onEvent: @escaping AVAStreamEventCallback,
        ctx: UnsafeMutableRawPointer,
        releaseContext: @escaping AVADropCallback
    ) {
        self.node = node
        self.bus = bus
        self.onEvent = onEvent
        self.ctx = ctx
        self.releaseContext = releaseContext
        super.init()
    }

    func install(bufferSize: UInt32, format: AVAudioFormat?, error: inout NSError?) -> Bool {
        installed = AVAXNodeInstallTap(node, UInt(bus), bufferSize, format, { [weak self] buffer, _ in
            guard let self else { return }
            self.onEvent(0, UnsafeRawPointer(Unmanaged.passRetained(buffer).toOpaque()), self.ctx)
        }, &error)
        return installed
    }

    func cancel() {
        guard installed else { return }
        installed = false
        node.removeTap(onBus: AVAudioNodeBus(bus))
    }

    deinit {
        cancel()
        releaseContext(ctx)
    }
}

@_cdecl("ava_node_tap_subscribe")
public func ava_node_tap_subscribe(
    _ nodePtr: UnsafeMutableRawPointer,
    _ bus: UInt32,
    _ bufferSize: UInt32,
    _ formatPtr: UnsafeMutableRawPointer?,
    _ onEvent: AVAStreamEventCallback,
    _ ctx: UnsafeMutableRawPointer,
    _ releaseContext: AVADropCallback,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let bridge = TapBridge(
        node: Unmanaged<AVAudioNode>.fromOpaque(nodePtr).takeUnretainedValue(),
        bus: bus,
        onEvent: onEvent,
        ctx: ctx,
        releaseContext: releaseContext
    )
    let format = formatPtr.map { Unmanaged<AVAudioFormat>.fromOpaque($0).takeUnretainedValue() }
    var error: NSError?
    guard bridge.install(bufferSize: bufferSize, format: format, error: &error) else {
        avaReportObjCFailure("AVAudioNode.installTap", error, outError)
        return nil
    }
    return Unmanaged.passRetained(bridge).toOpaque()
}

@_cdecl("ava_node_tap_unsubscribe")
public func ava_node_tap_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    let bridge = Unmanaged<TapBridge>.fromOpaque(handle).takeUnretainedValue()
    bridge.cancel()
    Unmanaged<TapBridge>.fromOpaque(handle).release()
}
