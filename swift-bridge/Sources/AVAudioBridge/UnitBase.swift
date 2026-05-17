import AudioToolbox
import AVFoundation
import Foundation

struct AudioComponentDescriptionPayload: Codable {
    let componentType: UInt32
    let componentSubtype: UInt32
    let componentManufacturer: UInt32
    let componentFlags: UInt32
    let componentFlagsMask: UInt32
}

struct AudioUnitMetadataPayload: Codable {
    let componentDescription: AudioComponentDescriptionPayload
    let name: String
    let manufacturerName: String
    let version: UInt64
    let audioUnitRaw: UInt64
    let hasAuAudioUnit: Bool
}

func avaAudioComponentDescription(
    _ componentType: UInt32,
    _ componentSubtype: UInt32,
    _ componentManufacturer: UInt32,
    _ componentFlags: UInt32,
    _ componentFlagsMask: UInt32
) -> AudioComponentDescription {
    AudioComponentDescription(
        componentType: componentType,
        componentSubType: componentSubtype,
        componentManufacturer: componentManufacturer,
        componentFlags: componentFlags,
        componentFlagsMask: componentFlagsMask
    )
}

func avaAudioComponentDescriptionPayload(_ description: AudioComponentDescription) -> AudioComponentDescriptionPayload {
    AudioComponentDescriptionPayload(
        componentType: description.componentType,
        componentSubtype: description.componentSubType,
        componentManufacturer: description.componentManufacturer,
        componentFlags: description.componentFlags,
        componentFlagsMask: description.componentFlagsMask
    )
}

private func audioUnitRawValue(_ audioUnit: AudioUnit) -> UInt64 {
    UInt64(UInt(bitPattern: audioUnit))
}

private func encodeAudioUnitMetadata(_ unit: AVAudioUnit) -> AudioUnitMetadataPayload {
    AudioUnitMetadataPayload(
        componentDescription: avaAudioComponentDescriptionPayload(unit.audioComponentDescription),
        name: unit.name,
        manufacturerName: unit.manufacturerName,
        version: UInt64(unit.version),
        audioUnitRaw: audioUnitRawValue(unit.audioUnit),
        hasAuAudioUnit: true
    )
}

@_cdecl("av_audio_unit_instantiate")
public func av_audio_unit_instantiate(
    _ componentType: UInt32,
    _ componentSubtype: UInt32,
    _ componentManufacturer: UInt32,
    _ componentFlags: UInt32,
    _ componentFlagsMask: UInt32,
    _ optionsRaw: UInt32,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let description = avaAudioComponentDescription(
        componentType,
        componentSubtype,
        componentManufacturer,
        componentFlags,
        componentFlagsMask
    )
    let semaphore = DispatchSemaphore(value: 0)
    var instantiatedUnit: AVAudioUnit?
    var instantiateError: Error?
    AVAudioUnit.instantiate(with: description, options: AudioComponentInstantiationOptions(rawValue: optionsRaw)) {
        audioUnit,
        error in
        instantiatedUnit = audioUnit
        instantiateError = error
        semaphore.signal()
    }
    semaphore.wait()
    if let instantiatedUnit {
        return Unmanaged.passRetained(instantiatedUnit).toOpaque()
    }
    outError?.pointee = ffiString((instantiateError ?? BridgeError.message("failed to instantiate AVAudioUnit")).localizedDescription)
    return nil
}

@_cdecl("av_audio_unit_release")
public func av_audio_unit_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioUnit>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_unit_metadata_json")
public func av_audio_unit_metadata_json(
    _ ptr: UnsafeMutableRawPointer,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let unit = Unmanaged<AVAudioUnit>.fromOpaque(ptr).takeUnretainedValue()
    do {
        return ffiString(try avaEncodeJSON(encodeAudioUnitMetadata(unit)))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_unit_load_preset_at_url")
public func av_audio_unit_load_preset_at_url(
    _ ptr: UnsafeMutableRawPointer,
    _ pathPtr: UnsafePointer<CChar>,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let unit = Unmanaged<AVAudioUnit>.fromOpaque(ptr).takeUnretainedValue()
    let path = String(cString: pathPtr)
    do {
        try unit.loadPreset(at: URL(fileURLWithPath: path))
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}

@_cdecl("av_audio_unit_copy_au_audio_unit")
public func av_audio_unit_copy_au_audio_unit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let unit = Unmanaged<AVAudioUnit>.fromOpaque(ptr).takeUnretainedValue()
    return Unmanaged.passRetained(unit.auAudioUnit).toOpaque()
}

@_cdecl("av_au_audio_unit_release")
public func av_au_audio_unit_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AUAudioUnit>.fromOpaque(ptr).release()
}
