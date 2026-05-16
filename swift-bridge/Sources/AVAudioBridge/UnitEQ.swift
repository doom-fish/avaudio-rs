import AVFoundation
import Foundation

struct EQBandInfoPayload: Codable {
    let filterType: Int32
    let frequency: Float
    let bandwidth: Float
    let gain: Float
    let bypass: Bool
}

@_cdecl("av_audio_unit_eq_create")
public func av_audio_unit_eq_create(
    _ bandCount: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard bandCount >= 0 else {
        outError?.pointee = ffiString("band count must be non-negative")
        return nil
    }
    return Unmanaged.passRetained(AVAudioUnitEQ(numberOfBands: Int(bandCount))).toOpaque()
}

@_cdecl("av_audio_unit_eq_release")
public func av_audio_unit_eq_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnitEQ>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_unit_eq_get_global_gain")
public func av_audio_unit_eq_get_global_gain(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioUnitEQ>.fromOpaque(ptr).takeUnretainedValue().globalGain
}

@_cdecl("av_audio_unit_eq_set_global_gain")
public func av_audio_unit_eq_set_global_gain(_ ptr: UnsafeMutableRawPointer, _ gain: Float) {
    let node = Unmanaged<AVAudioUnitEQ>.fromOpaque(ptr).takeUnretainedValue()
    node.globalGain = gain
}

@_cdecl("av_audio_unit_eq_get_band_count")
public func av_audio_unit_eq_get_band_count(_ ptr: UnsafeMutableRawPointer) -> Int32 {
    Int32(Unmanaged<AVAudioUnitEQ>.fromOpaque(ptr).takeUnretainedValue().bands.count)
}

@_cdecl("av_audio_unit_eq_get_band_info_json")
public func av_audio_unit_eq_get_band_info_json(
    _ ptr: UnsafeMutableRawPointer,
    _ bandIndex: Int32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let node = Unmanaged<AVAudioUnitEQ>.fromOpaque(ptr).takeUnretainedValue()
    guard bandIndex >= 0, Int(bandIndex) < node.bands.count else {
        outError?.pointee = ffiString("band index out of range")
        return nil
    }
    let band = node.bands[Int(bandIndex)]
    let payload = EQBandInfoPayload(
        filterType: Int32(band.filterType.rawValue),
        frequency: band.frequency,
        bandwidth: band.bandwidth,
        gain: band.gain,
        bypass: band.bypass
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_unit_eq_set_band_params")
public func av_audio_unit_eq_set_band_params(
    _ ptr: UnsafeMutableRawPointer,
    _ bandIndex: Int32,
    _ filterType: Int32,
    _ frequency: Float,
    _ bandwidth: Float,
    _ gain: Float,
    _ bypass: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let node = Unmanaged<AVAudioUnitEQ>.fromOpaque(ptr).takeUnretainedValue()
    guard bandIndex >= 0, Int(bandIndex) < node.bands.count else {
        outError?.pointee = ffiString("band index out of range")
        return AVA_INVALID_ARGUMENT
    }
    let band = node.bands[Int(bandIndex)]
    band.filterType = AVAudioUnitEQFilterType(rawValue: Int(filterType)) ?? .parametric
    band.frequency = frequency
    band.bandwidth = bandwidth
    band.gain = gain
    band.bypass = bypass
    return AVA_OK
}
