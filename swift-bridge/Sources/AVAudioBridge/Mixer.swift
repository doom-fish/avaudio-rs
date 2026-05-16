import AVFoundation
import Foundation

@_cdecl("av_audio_mixer_node_create")
public func av_audio_mixer_node_create() -> UnsafeMutableRawPointer? {
    Unmanaged.passRetained(AVAudioMixerNode()).toOpaque()
}

@_cdecl("av_audio_mixer_node_release")
public func av_audio_mixer_node_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AVAudioMixerNode>.fromOpaque(ptr).release()
}

@_cdecl("av_audio_mixer_node_get_output_volume")
public func av_audio_mixer_node_get_output_volume(_ ptr: UnsafeMutableRawPointer) -> Float {
    Unmanaged<AVAudioMixerNode>.fromOpaque(ptr).takeUnretainedValue().outputVolume
}

@_cdecl("av_audio_mixer_node_set_output_volume")
public func av_audio_mixer_node_set_output_volume(_ ptr: UnsafeMutableRawPointer, _ volume: Float) {
    let node = Unmanaged<AVAudioMixerNode>.fromOpaque(ptr).takeUnretainedValue()
    node.outputVolume = volume
}
