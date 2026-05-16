import AVFoundation
import Foundation

final class BoolCallbackBox {
    let callback: AVABoolCallback?
    let userData: UnsafeMutableRawPointer?
    let dropUserData: AVADropCallback?

    init(
        callback: AVABoolCallback?,
        userData: UnsafeMutableRawPointer?,
        dropUserData: AVADropCallback?
    ) {
        self.callback = callback
        self.userData = userData
        self.dropUserData = dropUserData
    }

    func fire(_ value: Bool) {
        callback?(userData, value)
    }

    deinit {
        if let userData, let dropUserData {
            dropUserData(userData)
        }
    }
}

@_cdecl("av_audio_application_get_input_muted")
public func av_audio_application_get_input_muted(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard #available(macOS 14.0, *) else {
        outError?.pointee = ffiString("AVAudioApplication requires macOS 14.0")
        return false
    }
    return AVAudioApplication.shared.isInputMuted
}

@_cdecl("av_audio_application_set_input_muted")
public func av_audio_application_set_input_muted(
    _ muted: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 14.0, *) else {
        outError?.pointee = ffiString("AVAudioApplication requires macOS 14.0")
        return AVA_OPERATION_FAILED
    }
    do {
        try AVAudioApplication.shared.setInputMuted(muted)
        return AVA_OK
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return AVA_OPERATION_FAILED
    }
}

@_cdecl("av_audio_application_get_record_permission")
public func av_audio_application_get_record_permission(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 14.0, *) else {
        outError?.pointee = ffiString("AVAudioApplication requires macOS 14.0")
        return -1
    }
    switch AVAudioApplication.shared.recordPermission {
    case .undetermined:
        return 0
    case .denied:
        return 1
    case .granted:
        return 2
    @unknown default:
        return -1
    }
}

@_cdecl("av_audio_application_request_record_permission")
public func av_audio_application_request_record_permission(
    _ callback: AVABoolCallback?,
    _ userData: UnsafeMutableRawPointer?,
    _ dropUserData: AVADropCallback?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 14.0, *) else {
        outError?.pointee = ffiString("AVAudioApplication requires macOS 14.0")
        return AVA_OPERATION_FAILED
    }
    guard let callback else {
        outError?.pointee = ffiString("record-permission callback must not be nil")
        return AVA_INVALID_ARGUMENT
    }
    let box = BoolCallbackBox(callback: callback, userData: userData, dropUserData: dropUserData)
    AVAudioApplication.requestRecordPermission(completionHandler: { granted in
        box.fire(granted)
    })
    return AVA_OK
}
