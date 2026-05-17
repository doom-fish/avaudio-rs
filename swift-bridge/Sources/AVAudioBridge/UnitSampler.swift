import AVFoundation
import Foundation

@_cdecl("av_audio_unit_sampler_create")
public func av_audio_unit_sampler_create() -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioUnitSampler()).toOpaque()
}

@_cdecl("av_audio_unit_sampler_release")
public func av_audio_unit_sampler_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_unit_sampler_load_instrument")
public func av_audio_unit_sampler_load_instrument(
    _ ptr: UnsafeMutableRawPointer,
    _ pathPtr: UnsafePointer<CChar>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let sampler = Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue()
    let path = String(cString: pathPtr)
    do {
        try sampler.loadInstrument(at: URL(fileURLWithPath: path))
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}

@_cdecl("av_audio_unit_sampler_load_audio_files")
public func av_audio_unit_sampler_load_audio_files(
    _ ptr: UnsafeMutableRawPointer,
    _ pathsPtr: UnsafePointer<UnsafePointer<CChar>?>,
    _ pathCount: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let sampler = Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue()
    let urls = (0..<pathCount).compactMap { index -> URL? in
        guard let raw = pathsPtr[index] else { return nil }
        return URL(fileURLWithPath: String(cString: raw))
    }
    do {
        try sampler.loadAudioFiles(at: urls)
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}

@_cdecl("av_audio_unit_sampler_load_sound_bank_instrument")
public func av_audio_unit_sampler_load_sound_bank_instrument(
    _ ptr: UnsafeMutableRawPointer,
    _ pathPtr: UnsafePointer<CChar>,
    _ program: Int32,
    _ bankMSB: Int32,
    _ bankLSB: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let sampler = Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue()
    let path = String(cString: pathPtr)
    do {
        try sampler.loadSoundBankInstrument(
            at: URL(fileURLWithPath: path),
            program: UInt8(clamping: program),
            bankMSB: UInt8(clamping: bankMSB),
            bankLSB: UInt8(clamping: bankLSB)
        )
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}

@_cdecl("av_audio_unit_sampler_get_stereo_pan")
public func av_audio_unit_sampler_get_stereo_pan(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue().stereoPan
}

@_cdecl("av_audio_unit_sampler_set_stereo_pan")
public func av_audio_unit_sampler_set_stereo_pan(_ ptr: UnsafeMutableRawPointer, _ stereoPan: Float) {
    let sampler = Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue()
    sampler.stereoPan = stereoPan
}

@_cdecl("av_audio_unit_sampler_get_overall_gain")
public func av_audio_unit_sampler_get_overall_gain(_ ptr: UnsafeMutableRawPointer) -> Float {
    let sampler = Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue()
    if #available(macOS 12.0, *) {
        return sampler.overallGain
    }
    return sampler.masterGain
}

@_cdecl("av_audio_unit_sampler_set_overall_gain")
public func av_audio_unit_sampler_set_overall_gain(_ ptr: UnsafeMutableRawPointer, _ overallGain: Float) {
    let sampler = Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue()
    if #available(macOS 12.0, *) {
        sampler.overallGain = overallGain
    } else {
        sampler.masterGain = overallGain
    }
}

@_cdecl("av_audio_unit_sampler_get_master_gain")
public func av_audio_unit_sampler_get_master_gain(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue().masterGain
}

@_cdecl("av_audio_unit_sampler_set_master_gain")
public func av_audio_unit_sampler_set_master_gain(_ ptr: UnsafeMutableRawPointer, _ masterGain: Float) {
    let sampler = Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue()
    sampler.masterGain = masterGain
}

@_cdecl("av_audio_unit_sampler_get_global_tuning")
public func av_audio_unit_sampler_get_global_tuning(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue().globalTuning
}

@_cdecl("av_audio_unit_sampler_set_global_tuning")
public func av_audio_unit_sampler_set_global_tuning(_ ptr: UnsafeMutableRawPointer, _ globalTuning: Float) {
    let sampler = Unmanaged<AVAudioUnitSampler>.fromOpaque(ptr).takeUnretainedValue()
    sampler.globalTuning = globalTuning
}
