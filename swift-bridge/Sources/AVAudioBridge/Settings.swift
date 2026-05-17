import AVFoundation
import Foundation

private struct AudioSettingsConstantsPayload: Codable {
    let audioFileTypeKey: String
    let bitRateStrategyConstant: String
    let bitRateStrategyLongTermAverage: String
    let bitRateStrategyVariable: String
    let bitRateStrategyVariableConstrained: String
}

@_cdecl("av_audio_settings_constants_json")
public func av_audio_settings_constants_json(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    let payload = AudioSettingsConstantsPayload(
        audioFileTypeKey: AVAudioFileTypeKey,
        bitRateStrategyConstant: AVAudioBitRateStrategy_Constant,
        bitRateStrategyLongTermAverage: AVAudioBitRateStrategy_LongTermAverage,
        bitRateStrategyVariable: AVAudioBitRateStrategy_Variable,
        bitRateStrategyVariableConstrained: AVAudioBitRateStrategy_VariableConstrained
    )
    do {
        return ffiString(try avaEncodeJSON(payload))
    } catch {
        outError?.pointee = ffiString(error.localizedDescription)
        return nil
    }
}
