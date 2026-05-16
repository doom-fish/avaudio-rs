import AVFoundation
import Foundation

final class SourceNodeRenderCallbackBox {
    let callback: AVASourceNodeRenderCallback?
    let userData: UnsafeMutableRawPointer?
    let dropUserData: AVADropCallback?

    init(
        callback: AVASourceNodeRenderCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) {
        self.callback = callback
        self.userData = userData
        self.dropUserData = dropUserData
    }

    func render(
        isSilence: UnsafeMutablePointer<ObjCBool>?,
        timestamp: UnsafePointer<AudioTimeStamp>?,
        frameCount: AVAudioFrameCount,
        outputData: UnsafeMutablePointer<AudioBufferList>?
    ) -> OSStatus {
        var rustSilence = isSilence?.pointee.boolValue ?? false
        let status = callback?(
            userData,
            &rustSilence,
            timestamp.map(UnsafeRawPointer.init),
            frameCount,
            outputData.map(UnsafeMutableRawPointer.init)
        ) ?? AVA_OK
        if let isSilence {
            isSilence.pointee = ObjCBool(rustSilence)
        }
        return OSStatus(status)
    }

    deinit {
        if let userData, let dropUserData {
            dropUserData(userData)
        }
    }
}

final class SinkNodeReceiverCallbackBox {
    let callback: AVASinkNodeReceiverCallback?
    let userData: UnsafeMutableRawPointer?
    let dropUserData: AVADropCallback?

    init(
        callback: AVASinkNodeReceiverCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) {
        self.callback = callback
        self.userData = userData
        self.dropUserData = dropUserData
    }

    func receive(
        timestamp: UnsafePointer<AudioTimeStamp>?,
        frameCount: AVAudioFrameCount,
        inputData: UnsafePointer<AudioBufferList>?
    ) -> OSStatus {
        let status = callback?(
            userData,
            timestamp.map(UnsafeRawPointer.init),
            frameCount,
            inputData.map(UnsafeRawPointer.init)
        ) ?? AVA_OK
        return OSStatus(status)
    }

    deinit {
        if let userData, let dropUserData {
            dropUserData(userData)
        }
    }
}

@_cdecl("av_audio_source_node_create")
public func av_audio_source_node_create(
    _ callback: AVASourceNodeRenderCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 10.15, *) else {
        outError?.pointee = ffiString("AVAudioSourceNode requires macOS 10.15")
        return nil
    }
    guard let callback else {
        outError?.pointee = ffiString("source-node callback must not be nil")
        return nil
    }
    let box = SourceNodeRenderCallbackBox(callback: callback, userData: userData, dropUserData: dropUserData)
    let node = AVAudioSourceNode(renderBlock: { isSilence, timestamp, frameCount, outputData in
        box.render(isSilence: isSilence, timestamp: timestamp, frameCount: frameCount, outputData: outputData)
    })
    return Unmanaged.passRetained(node).toOpaque()
}

@_cdecl("av_audio_source_node_create_with_format")
public func av_audio_source_node_create_with_format(
    _ formatPtr: UnsafeMutableRawPointer,
    _ callback: AVASourceNodeRenderCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 10.15, *) else {
        outError?.pointee = ffiString("AVAudioSourceNode requires macOS 10.15")
        return nil
    }
    guard let callback else {
        outError?.pointee = ffiString("source-node callback must not be nil")
        return nil
    }
    let format = Unmanaged<AVAudioFormat>.fromOpaque(formatPtr).takeUnretainedValue()
    let box = SourceNodeRenderCallbackBox(callback: callback, userData: userData, dropUserData: dropUserData)
    let node = AVAudioSourceNode(format: format, renderBlock: { isSilence, timestamp, frameCount, outputData in
        box.render(isSilence: isSilence, timestamp: timestamp, frameCount: frameCount, outputData: outputData)
    })
    return Unmanaged.passRetained(node).toOpaque()
}

@_cdecl("av_audio_source_node_release")
public func av_audio_source_node_release(_ ptr: UnsafeMutableRawPointer?) {
    av_audio_node_release(ptr)
}

@_cdecl("av_audio_sink_node_create")
public func av_audio_sink_node_create(
    _ callback: AVASinkNodeReceiverCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 10.15, *) else {
        outError?.pointee = ffiString("AVAudioSinkNode requires macOS 10.15")
        return nil
    }
    guard let callback else {
        outError?.pointee = ffiString("sink-node callback must not be nil")
        return nil
    }
    let box = SinkNodeReceiverCallbackBox(callback: callback, userData: userData, dropUserData: dropUserData)
    let node = AVAudioSinkNode(receiverBlock: { timestamp, frameCount, inputData in
        box.receive(timestamp: timestamp, frameCount: frameCount, inputData: inputData)
    })
    return Unmanaged.passRetained(node).toOpaque()
}

@_cdecl("av_audio_sink_node_release")
public func av_audio_sink_node_release(_ ptr: UnsafeMutableRawPointer?) {
    av_audio_node_release(ptr)
}
