# avaudio-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 125
VERIFIED: 30
GAPS: 95
EXEMPT: 0
COVERAGE_PCT: 24.00%

Audit scope: top-level `AVAudio*` declarations in `AVFAudio.framework` headers (classes, protocols, enums/options, typedefs, constants, and helper functions), not per-method coverage.
Filtered out 80 symbols explicitly unavailable on macOS. The remaining gap set still includes standalone `AVAudioSession*` types that Apple leaves header-visible in the macOS SDK without `API_UNAVAILABLE(macos)` annotations.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `AVAudio3DAngularOrientation` | typealias | `AVAudioTypes.h` | AudioEnvironmentNode::{listener_orientation, set_listener_orientation} + AudioListenerOrientation |
| `AVAudio3DPoint` | typealias | `AVAudioTypes.h` | AudioEnvironmentNode::{listener_position, set_listener_position} + AudioListenerPosition |
| `AVAudioBuffer` | class | `AVAudioBuffer.h` | AudioBufferHandle, AudioBufferInfo |
| `AVAudioCommonFormat` | enum | `AVAudioFormat.h` | AudioCommonFormat, AudioFormat::common_format() |
| `AVAudioConverter` | class | `AVAudioConverter.h` | AudioConverter |
| `AVAudioEngine` | class | `AVAudioEngine.h` | AudioEngine |
| `AVAudioEnvironmentDistanceAttenuationModel` | enum | `AVAudioEnvironmentNode.h` | AudioDistanceAttenuation.model + AudioEnvironmentNode::set_distance_attenuation() |
| `AVAudioEnvironmentDistanceAttenuationParameters` | class | `AVAudioEnvironmentNode.h` | AudioDistanceAttenuation + AudioEnvironmentNode::{distance_attenuation, set_distance_attenuation} |
| `AVAudioEnvironmentNode` | class | `AVAudioEnvironmentNode.h` | AudioEnvironmentNode |
| `AVAudioEnvironmentReverbParameters` | class | `AVAudioEnvironmentNode.h` | AudioEnvironmentNode::{reverb_blend, set_reverb_blend} |
| `AVAudioFile` | class | `AVAudioFile.h` | AudioFile |
| `AVAudioFormat` | class | `AVAudioFormat.h` | AudioFormat |
| `AVAudioInputNode` | class | `AVAudioIONode.h` | AudioInputNode |
| `AVAudioMixerNode` | class | `AVAudioMixerNode.h` | AudioMixerNode |
| `AVAudioNode` | class | `AVAudioNode.h` | AudioNodeHandle + AudioEngine graph helpers |
| `AVAudioNodeCompletionHandler` | block | `AVAudioTypes.h` | AudioPlayerNode::{schedule_buffer_with_completion, schedule_file_with_completion} |
| `AVAudioOutputNode` | class | `AVAudioIONode.h` | AudioOutputNode |
| `AVAudioPCMBuffer` | class | `AVAudioBuffer.h` | PCMBuffer |
| `AVAudioPlayer` | class | `AVAudioPlayer.h` | AudioSimplePlayer |
| `AVAudioPlayerNode` | class | `AVAudioPlayerNode.h` | AudioPlayerNode |
| `AVAudioRecorder` | class | `AVAudioRecorder.h` | AudioRecorder |
| `AVAudioUnit` | class | `AVAudioUnit.h` | AudioUnitHandle + AudioUnitInfo |
| `AVAudioUnitEQ` | class | `AVAudioUnitEQ.h` | AudioUnitEQ |
| `AVAudioUnitEQFilterParameters` | class | `AVAudioUnitEQ.h` | AudioUnitEQ::{band_info, set_band_params} |
| `AVAudioUnitEQFilterType` | enum | `AVAudioUnitEQ.h` | AudioEQBandInfo.filter_type / AudioEQBandParams.filter_type |
| `AVAudioUnitEffect` | class | `AVAudioUnitEffect.h` | AudioUnitHandle::bypass() on AudioUnitEQ / AudioUnitReverb (partial) |
| `AVAudioUnitReverb` | class | `AVAudioUnitReverb.h` | AudioUnitReverb |
| `AVAudioUnitReverbPreset` | enum | `AVAudioUnitReverb.h` | AudioUnitReverbPreset |
| `AVAudioUnitTimeEffect` | class | `AVAudioUnitTimeEffect.h` | AudioUnitHandle::bypass() on AudioUnitTimePitch (partial) |
| `AVAudioUnitTimePitch` | class | `AVAudioUnitTimePitch.h` | AudioUnitTimePitch |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `AVAudio3DMixing` | protocol | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudio3DMixingPointSourceInHeadMode` | enum | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudio3DMixingRenderingAlgorithm` | enum | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudio3DMixingSourceMode` | enum | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudio3DVector` | typealias | `AVAudioTypes.h` | 3D helper vector/function is not wrapped. |
| `AVAudio3DVectorOrientation` | typealias | `AVAudioTypes.h` | 3D helper vector/function is not wrapped. |
| `AVAudioApplication` | class | `AVAudioApplication.h` | No wrapper for application-level permission or input-mute APIs. |
| `AVAudioApplicationMicrophoneInjectionPermission` | enum | `AVAudioApplication.h` | No wrapper for application-level permission or input-mute APIs. |
| `AVAudioApplicationRecordPermission` | enum | `AVAudioApplication.h` | No wrapper for application-level permission or input-mute APIs. |
| `AVAudioBitRateStrategy` | constant | `AVAudioSettings.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioBitRateStrategy_Constant` | constant | `AVAudioSettings.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioBitRateStrategy_LongTermAverage` | constant | `AVAudioSettings.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioBitRateStrategy_Variable` | constant | `AVAudioSettings.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioBitRateStrategy_VariableConstrained` | constant | `AVAudioSettings.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioChannelCount` | typealias | `AVAudioTypes.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioChannelLayout` | class | `AVAudioChannelLayout.h` | Foundational helper type is not wrapped yet. |
| `AVAudioCompressedBuffer` | class | `AVAudioBuffer.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioConnectionPoint` | class | `AVAudioConnectionPoint.h` | Foundational helper type is not wrapped yet. |
| `AVAudioContentSource` | enum | `AVAudioSettings.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioConverterInputStatus` | enum | `AVAudioConverter.h` | Converter status or prime APIs are not exposed. |
| `AVAudioConverterOutputStatus` | enum | `AVAudioConverter.h` | Converter status or prime APIs are not exposed. |
| `AVAudioConverterPrimeInfo` | struct | `AVAudioConverter.h` | Converter status or prime APIs are not exposed. |
| `AVAudioConverterPrimeMethod` | enum | `AVAudioConverter.h` | Converter status or prime APIs are not exposed. |
| `AVAudioDynamicRangeControlConfiguration` | enum | `AVAudioSettings.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioEngineConfigurationChangeNotification` | constant | `AVAudioEngine.h` | Manual-rendering or notification surface is not wrapped. |
| `AVAudioEngineManualRenderingBlock` | block | `AVAudioEngine.h` | Manual-rendering or notification surface is not wrapped. |
| `AVAudioEngineManualRenderingError` | enum | `AVAudioEngine.h` | Manual-rendering or notification surface is not wrapped. |
| `AVAudioEngineManualRenderingMode` | enum | `AVAudioEngine.h` | Manual-rendering or notification surface is not wrapped. |
| `AVAudioEngineManualRenderingStatus` | enum | `AVAudioEngine.h` | Manual-rendering or notification surface is not wrapped. |
| `AVAudioEnvironmentOutputType` | enum | `AVAudioEnvironmentNode.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioFileTypeKey` | constant | `AVAudioSettings.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioFrameCount` | typealias | `AVAudioTypes.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioFramePosition` | typealias | `AVAudioTypes.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioIONode` | class | `AVAudioIONode.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioIONodeInputBlock` | block | `AVAudioIONode.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioMixing` | protocol | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudioMixingDestination` | class | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudioNodeBus` | typealias | `AVAudioTypes.h` | Node tap/time helpers are not fully wrapped. |
| `AVAudioNodeTapBlock` | block | `AVAudioNode.h` | Node tap/time helpers are not fully wrapped. |
| `AVAudioPacketCount` | typealias | `AVAudioTypes.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioPlayerDelegate` | protocol | `AVAudioPlayer.h` | Delegate protocol is not bridged. |
| `AVAudioPlayerNodeBufferOptions` | enum | `AVAudioPlayerNode.h` | Buffer options or typed completion-callback APIs are not wrapped. |
| `AVAudioPlayerNodeCompletionCallbackType` | enum | `AVAudioPlayerNode.h` | Buffer options or typed completion-callback APIs are not wrapped. |
| `AVAudioPlayerNodeCompletionHandler` | block | `AVAudioPlayerNode.h` | Buffer options or typed completion-callback APIs are not wrapped. |
| `AVAudioQuality` | enum | `AVAudioSettings.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioRecorderDelegate` | protocol | `AVAudioRecorder.h` | Delegate protocol is not bridged. |
| `AVAudioRoutingArbiter` | class | `AVAudioRoutingArbiter.h` | No wrapper for routing-arbitration APIs. |
| `AVAudioRoutingArbitrationCategory` | enum | `AVAudioRoutingArbiter.h` | No wrapper for routing-arbitration APIs. |
| `AVAudioSequencer` | class | `AVAudioSequencer.h` | No wrapper for sequencing or timeline APIs. |
| `AVAudioSequencerUserCallback` | block | `AVAudioSequencer.h` | No wrapper for sequencing or timeline APIs. |
| `AVAudioSessionActivationOptions` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionAnchoringStrategy` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionCapability` | class | `AVAudioSessionRoute.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionIOType` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionInterruptionOptions` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionInterruptionType` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionMicrophoneInjectionMode` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionPromptStyle` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionRenderingMode` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionRouteChangeReason` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionSetActiveOptions` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionSilenceSecondaryAudioHintType` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionSoundStageSize` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSessionSpatialExperience` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioSinkNode` | class | `AVAudioSinkNode.h` | No wrapper for custom render-block nodes. |
| `AVAudioSinkNodeReceiverBlock` | block | `AVAudioSinkNode.h` | No wrapper for custom render-block nodes. |
| `AVAudioSourceNode` | class | `AVAudioSourceNode.h` | No wrapper for custom render-block nodes. |
| `AVAudioSourceNodeRenderBlock` | block | `AVAudioSourceNode.h` | No wrapper for custom render-block nodes. |
| `AVAudioStereoMixing` | protocol | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudioStereoOrientation` | enum | `AVAudioSessionTypes.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioTime` | class | `AVAudioTime.h` | Foundational helper type is not wrapped yet. |
| `AVAudioUnitComponent` | class | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitComponentManager` | class | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitComponentTagsDidChangeNotification` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitDelay` | class | `AVAudioUnitDelay.h` | No public Rust wrapper for this audio-unit subtype. |
| `AVAudioUnitDistortion` | class | `AVAudioUnitDistortion.h` | No public Rust wrapper for this audio-unit subtype. |
| `AVAudioUnitDistortionPreset` | enum | `AVAudioUnitDistortion.h` | No public Rust wrapper for this audio-unit subtype. |
| `AVAudioUnitGenerator` | class | `AVAudioUnitGenerator.h` | No public Rust wrapper for this audio-unit subtype. |
| `AVAudioUnitMIDIInstrument` | class | `AVAudioUnitMIDIInstrument.h` | No public Rust wrapper for this audio-unit subtype. |
| `AVAudioUnitManufacturerNameApple` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitSampler` | class | `AVAudioUnitSampler.h` | No public Rust wrapper for this audio-unit subtype. |
| `AVAudioUnitTypeEffect` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitTypeFormatConverter` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitTypeGenerator` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitTypeMIDIProcessor` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitTypeMixer` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitTypeMusicDevice` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitTypeMusicEffect` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitTypeOfflineEffect` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitTypeOutput` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitTypePanner` | constant | `AVAudioUnitComponent.h` | No wrapper for audio-unit discovery or component management. |
| `AVAudioUnitVarispeed` | class | `AVAudioUnitVarispeed.h` | No public Rust wrapper for this audio-unit subtype. |
| `AVAudioVoiceProcessingOtherAudioDuckingConfiguration` | struct | `AVAudioIONode.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioVoiceProcessingOtherAudioDuckingLevel` | enum | `AVAudioIONode.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioVoiceProcessingSpeechActivityEvent` | enum | `AVAudioIONode.h` | No public Rust wrapper or matching Swift bridge thunk. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| _None_ | - | - | No macOS-deprecated top-level AVAudio* symbols remained after filtering. | - |

