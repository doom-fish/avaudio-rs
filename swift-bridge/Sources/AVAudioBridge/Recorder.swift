import AVFoundation
import Foundation

final class AudioRecorderBox {
    var recorder: AVAudioRecorder?

    init(url: URL, sampleRate: Double, channels: Int, bitDepth: Int) throws {
        let settings: [String: Any] = [
            AVFormatIDKey: kAudioFormatLinearPCM,
            AVSampleRateKey: sampleRate,
            AVNumberOfChannelsKey: channels,
            AVLinearPCMBitDepthKey: bitDepth,
            AVLinearPCMIsBigEndianKey: false,
            AVLinearPCMIsFloatKey: bitDepth == 32
        ]
        self.recorder = try AVAudioRecorder(url: url, settings: settings)
    }
}

@_cdecl("av_audio_recorder_create")
public func av_audio_recorder_create(
    _ pathPtr: UnsafePointer<CChar>,
    _ sampleRate: Double,
    _ channels: Int32,
    _ bitDepth: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let path = String(cString: pathPtr)
    do {
        let box = try AudioRecorderBox(
            url: URL(fileURLWithPath: path),
            sampleRate: sampleRate,
            channels: Int(channels),
            bitDepth: Int(bitDepth)
        )
        return Unmanaged.passRetained(box).toOpaque()
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_recorder_release")
public func av_audio_recorder_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AudioRecorderBox>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_recorder_record")
public func av_audio_recorder_record(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let box = Unmanaged<AudioRecorderBox>.fromOpaque(ptr).takeUnretainedValue()
    return box.recorder?.record() ?? false
}

@_cdecl("av_audio_recorder_stop")
public func av_audio_recorder_stop(_ ptr: UnsafeMutableRawPointer) {
    let box = Unmanaged<AudioRecorderBox>.fromOpaque(ptr).takeUnretainedValue()
    box.recorder?.stop()
}

@_cdecl("av_audio_recorder_pause")
public func av_audio_recorder_pause(_ ptr: UnsafeMutableRawPointer) {
    let box = Unmanaged<AudioRecorderBox>.fromOpaque(ptr).takeUnretainedValue()
    box.recorder?.pause()
}

@_cdecl("av_audio_recorder_is_recording")
public func av_audio_recorder_is_recording(_ ptr: UnsafeMutableRawPointer) -> Bool {
    Unmanaged<AudioRecorderBox>.fromOpaque(ptr).takeUnretainedValue().recorder?.isRecording ?? false
}

@_cdecl("av_audio_recorder_current_time")
public func av_audio_recorder_current_time(_ ptr: UnsafeMutableRawPointer) -> Double {
    Unmanaged<AudioRecorderBox>.fromOpaque(ptr).takeUnretainedValue().recorder?.currentTime ?? 0
}

@_cdecl("av_audio_recorder_set_metering_enabled")
public func av_audio_recorder_set_metering_enabled(_ ptr: UnsafeMutableRawPointer, _ enabled: Bool) {
    let box = Unmanaged<AudioRecorderBox>.fromOpaque(ptr).takeUnretainedValue()
    box.recorder?.isMeteringEnabled = enabled
}

@_cdecl("av_audio_recorder_update_meters")
public func av_audio_recorder_update_meters(_ ptr: UnsafeMutableRawPointer) {
    let box = Unmanaged<AudioRecorderBox>.fromOpaque(ptr).takeUnretainedValue()
    box.recorder?.updateMeters()
}

@_cdecl("av_audio_recorder_average_power")
public func av_audio_recorder_average_power(_ ptr: UnsafeMutableRawPointer, _ channel: Int32) -> Float {
    let box = Unmanaged<AudioRecorderBox>.fromOpaque(ptr).takeUnretainedValue()
    return box.recorder?.averagePower(forChannel: Int(channel)) ?? 0
}

@_cdecl("av_audio_recorder_peak_power")
public func av_audio_recorder_peak_power(_ ptr: UnsafeMutableRawPointer, _ channel: Int32) -> Float {
    let box = Unmanaged<AudioRecorderBox>.fromOpaque(ptr).takeUnretainedValue()
    return box.recorder?.peakPower(forChannel: Int(channel)) ?? 0
}

@_cdecl("av_audio_recorder_delete_recording")
public func av_audio_recorder_delete_recording(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let box = Unmanaged<AudioRecorderBox>.fromOpaque(ptr).takeUnretainedValue()
    return box.recorder?.deleteRecording() ?? false
}
