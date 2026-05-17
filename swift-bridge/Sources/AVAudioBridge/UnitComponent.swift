import AVFoundation
import Foundation

struct AudioUnitComponentInfoPayload: Codable {
    let name: String
    let componentDescription: AudioComponentDescriptionPayload
    let typeName: String
    let localizedTypeName: String
    let manufacturerName: String
    let version: UInt64
    let versionString: String
    let availableArchitectures: [Int64]
    let sandboxSafe: Bool
    let hasMIDIInput: Bool
    let hasMIDIOutput: Bool
    let userTagNames: [String]
    let allTagNames: [String]
    let iconURL: String?
    let passesAUVal: Bool
    let hasCustomView: Bool
}

struct AudioUnitComponentConstantsPayload: Codable {
    let tagsDidChangeNotification: String
    let manufacturerNameApple: String
    let typeEffect: String
    let typeFormatConverter: String
    let typeGenerator: String
    let typeMIDIProcessor: String
    let typeMixer: String
    let typeMusicDevice: String
    let typeMusicEffect: String
    let typeOfflineEffect: String
    let typeOutput: String
    let typePanner: String
}

private func audioUnitComponentManager() -> AVAudioUnitComponentManager {
    AVAudioUnitComponentManager.shared()
}

private func encodeComponent(_ component: AVAudioUnitComponent) -> AudioUnitComponentInfoPayload {
    AudioUnitComponentInfoPayload(
        name: component.name,
        componentDescription: avaAudioComponentDescriptionPayload(component.audioComponentDescription),
        typeName: component.typeName,
        localizedTypeName: component.localizedTypeName,
        manufacturerName: component.manufacturerName,
        version: UInt64(component.version),
        versionString: component.versionString,
        availableArchitectures: component.availableArchitectures.map(\.int64Value),
        sandboxSafe: component.isSandboxSafe,
        hasMIDIInput: component.hasMIDIInput,
        hasMIDIOutput: component.hasMIDIOutput,
        userTagNames: component.userTagNames,
        allTagNames: component.allTagNames,
        iconURL: component.iconURL?.absoluteString,
        passesAUVal: component.passesAUVal,
        hasCustomView: component.hasCustomView
    )
}

private func notificationNameString(_ value: Any) -> String {
    if let notification = value as? NSNotification.Name {
        return notification.rawValue
    }
    if let string = value as? String {
        return string
    }
    return String(describing: value)
}

@_cdecl("av_audio_unit_component_manager_tag_names_json")
public func av_audio_unit_component_manager_tag_names_json(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        return ffiString(try avaEncodeJSON(audioUnitComponentManager().tagNames))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_unit_component_manager_standard_localized_tag_names_json")
public func av_audio_unit_component_manager_standard_localized_tag_names_json(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        return ffiString(try avaEncodeJSON(audioUnitComponentManager().standardLocalizedTagNames))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_unit_component_manager_components_json")
public func av_audio_unit_component_manager_components_json(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let components = audioUnitComponentManager().components(matching: NSPredicate(value: true))
        .map(encodeComponent)
    do {
        return ffiString(try avaEncodeJSON(components))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}

@_cdecl("av_audio_unit_component_constants_json")
public func av_audio_unit_component_constants_json(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let payload = AudioUnitComponentConstantsPayload(
        tagsDidChangeNotification: notificationNameString(Notification.Name.AVAudioUnitComponentTagsDidChange),
        manufacturerNameApple: AVAudioUnitManufacturerNameApple,
        typeEffect: AVAudioUnitTypeEffect,
        typeFormatConverter: AVAudioUnitTypeFormatConverter,
        typeGenerator: AVAudioUnitTypeGenerator,
        typeMIDIProcessor: AVAudioUnitTypeMIDIProcessor,
        typeMixer: AVAudioUnitTypeMixer,
        typeMusicDevice: AVAudioUnitTypeMusicDevice,
        typeMusicEffect: AVAudioUnitTypeMusicEffect,
        typeOfflineEffect: AVAudioUnitTypeOfflineEffect,
        typeOutput: AVAudioUnitTypeOutput,
        typePanner: AVAudioUnitTypePanner
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}
