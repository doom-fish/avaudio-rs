# avaudio-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 125
VERIFIED: 93
GAPS: 31
EXEMPT: 1
COVERAGE_PCT: 75.00%

Audit scope: top-level `AVAudio*` declarations in `AVFAudio.framework` headers (classes, protocols, enums/options, typedefs, constants, and helper functions), not per-method coverage.
Filtered out 80 symbols explicitly unavailable on macOS. The remaining gap set still includes standalone `AVAudioSession*` types that Apple leaves header-visible in the macOS SDK without `API_UNAVAILABLE(macos)` annotations.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `AVAudio3DAngularOrientation` | typealias | `AVAudioTypes.h` | AudioEnvironmentNode::{listener_orientation, set_listener_orientation} + AudioListenerOrientation |
| `AVAudio3DMixingPointSourceInHeadMode` | enum | `AVAudioMixing.h` | Audio3DMixingPointSourceInHeadMode |
| `AVAudio3DMixingRenderingAlgorithm` | enum | `AVAudioMixing.h` | Audio3DMixingRenderingAlgorithm |
| `AVAudio3DMixingSourceMode` | enum | `AVAudioMixing.h` | Audio3DMixingSourceMode |
| `AVAudio3DPoint` | typealias | `AVAudioTypes.h` | AudioEnvironmentNode::{listener_position, set_listener_position} + AudioListenerPosition |
| `AVAudio3DVector` | typealias | `AVAudioTypes.h` | Audio3DVector::new() |
| `AVAudio3DVectorOrientation` | typealias | `AVAudioTypes.h` | Audio3DVectorOrientation::new() |
| `AVAudioApplication` | class | `AVAudioApplication.h` | `AudioApplication::{shared, input_muted, set_input_muted, record_permission, request_record_permission}` |
| `AVAudioApplicationRecordPermission` | enum | `AVAudioApplication.h` | `AudioApplication::record_permission() + AudioApplicationRecordPermission` |
| `AVAudioBitRateStrategy` | constant | `AVAudioSettings.h` | AudioBitRateStrategy + AudioSettingsConstants |
| `AVAudioBitRateStrategy_Constant` | constant | `AVAudioSettings.h` | AudioSettingsConstants::current().bit_rate_strategy_constant |
| `AVAudioBitRateStrategy_LongTermAverage` | constant | `AVAudioSettings.h` | AudioSettingsConstants::current().bit_rate_strategy_long_term_average |
| `AVAudioBitRateStrategy_Variable` | constant | `AVAudioSettings.h` | AudioSettingsConstants::current().bit_rate_strategy_variable |
| `AVAudioBitRateStrategy_VariableConstrained` | constant | `AVAudioSettings.h` | AudioSettingsConstants::current().bit_rate_strategy_variable_constrained |
| `AVAudioBuffer` | class | `AVAudioBuffer.h` | AudioBufferHandle, AudioBufferInfo |
| `AVAudioChannelCount` | typealias | `AVAudioTypes.h` | AudioChannelCount |
| `AVAudioCommonFormat` | enum | `AVAudioFormat.h` | AudioCommonFormat, AudioFormat::common_format() |
| `AVAudioContentSource` | enum | `AVAudioSettings.h` | AudioContentSource |
| `AVAudioConverter` | class | `AVAudioConverter.h` | AudioConverter |
| `AVAudioDynamicRangeControlConfiguration` | enum | `AVAudioSettings.h` | AudioDynamicRangeControlConfiguration |
| `AVAudioEngine` | class | `AVAudioEngine.h` | AudioEngine |
| `AVAudioEnvironmentDistanceAttenuationModel` | enum | `AVAudioEnvironmentNode.h` | AudioDistanceAttenuation.model + AudioEnvironmentNode::set_distance_attenuation() |
| `AVAudioEnvironmentDistanceAttenuationParameters` | class | `AVAudioEnvironmentNode.h` | AudioDistanceAttenuation + AudioEnvironmentNode::{distance_attenuation, set_distance_attenuation} |
| `AVAudioEnvironmentNode` | class | `AVAudioEnvironmentNode.h` | AudioEnvironmentNode |
| `AVAudioEnvironmentOutputType` | enum | `AVAudioEnvironmentNode.h` | AudioEnvironmentOutputType |
| `AVAudioEnvironmentReverbParameters` | class | `AVAudioEnvironmentNode.h` | AudioEnvironmentNode::{reverb_blend, set_reverb_blend} |
| `AVAudioFile` | class | `AVAudioFile.h` | AudioFile |
| `AVAudioFileTypeKey` | constant | `AVAudioSettings.h` | AudioSettingsConstants::current().audio_file_type_key |
| `AVAudioFormat` | class | `AVAudioFormat.h` | AudioFormat |
| `AVAudioFrameCount` | typealias | `AVAudioTypes.h` | AudioFrameCount |
| `AVAudioFramePosition` | typealias | `AVAudioTypes.h` | AudioFramePosition |
| `AVAudioInputNode` | class | `AVAudioIONode.h` | AudioInputNode |
| `AVAudioMixerNode` | class | `AVAudioMixerNode.h` | AudioMixerNode |
| `AVAudioNode` | class | `AVAudioNode.h` | AudioNodeHandle + AudioEngine graph helpers |
| `AVAudioNodeBus` | typealias | `AVAudioTypes.h` | AudioNodeBus |
| `AVAudioNodeCompletionHandler` | block | `AVAudioTypes.h` | AudioPlayerNode::{schedule_buffer_with_completion, schedule_file_with_completion} |
| `AVAudioOutputNode` | class | `AVAudioIONode.h` | AudioOutputNode |
| `AVAudioPCMBuffer` | class | `AVAudioBuffer.h` | PCMBuffer |
| `AVAudioPacketCount` | typealias | `AVAudioTypes.h` | AudioPacketCount |
| `AVAudioPlayer` | class | `AVAudioPlayer.h` | AudioSimplePlayer |
| `AVAudioPlayerNode` | class | `AVAudioPlayerNode.h` | AudioPlayerNode |
| `AVAudioQuality` | enum | `AVAudioSettings.h` | AudioQuality |
| `AVAudioRecorder` | class | `AVAudioRecorder.h` | AudioRecorder |
| `AVAudioSequencer` | class | `AVAudioSequencer.h` | `AudioSequencer` |
| `AVAudioSequencerUserCallback` | block | `AVAudioSequencer.h` | `AudioSequencer::{set_user_callback, clear_user_callback} + AudioSequencerUserEvent` |
| `AVAudioSessionActivationOptions` | enum | `AVAudioSessionTypes.h` | AudioSessionActivationOptions |
| `AVAudioSessionAnchoringStrategy` | enum | `AVAudioSessionTypes.h` | AudioSessionAnchoringStrategy |
| `AVAudioSessionIOType` | enum | `AVAudioSessionTypes.h` | AudioSessionIOType |
| `AVAudioSessionInterruptionOptions` | enum | `AVAudioSessionTypes.h` | AudioSessionInterruptionOptions |
| `AVAudioSessionInterruptionType` | enum | `AVAudioSessionTypes.h` | AudioSessionInterruptionType |
| `AVAudioSessionMicrophoneInjectionMode` | enum | `AVAudioSessionTypes.h` | AudioSessionMicrophoneInjectionMode |
| `AVAudioSessionPromptStyle` | enum | `AVAudioSessionTypes.h` | AudioSessionPromptStyle |
| `AVAudioSessionRenderingMode` | enum | `AVAudioSessionTypes.h` | AudioSessionRenderingMode |
| `AVAudioSessionRouteChangeReason` | enum | `AVAudioSessionTypes.h` | AudioSessionRouteChangeReason |
| `AVAudioSessionSetActiveOptions` | enum | `AVAudioSessionTypes.h` | AudioSessionSetActiveOptions |
| `AVAudioSessionSilenceSecondaryAudioHintType` | enum | `AVAudioSessionTypes.h` | AudioSessionSilenceSecondaryAudioHintType |
| `AVAudioSessionSoundStageSize` | enum | `AVAudioSessionTypes.h` | AudioSessionSoundStageSize |
| `AVAudioSessionSpatialExperience` | enum | `AVAudioSessionTypes.h` | AudioSessionSpatialExperience |
| `AVAudioSinkNode` | class | `AVAudioSinkNode.h` | `AudioSinkNode` |
| `AVAudioSinkNodeReceiverBlock` | block | `AVAudioSinkNode.h` | `AudioSinkNode::new() + AudioSinkRenderContext` |
| `AVAudioSourceNode` | class | `AVAudioSourceNode.h` | `AudioSourceNode` |
| `AVAudioSourceNodeRenderBlock` | block | `AVAudioSourceNode.h` | `AudioSourceNode::{new, new_with_format} + AudioSourceRenderContext` |
| `AVAudioStereoOrientation` | enum | `AVAudioSessionTypes.h` | AudioStereoOrientation |
| `AVAudioUnit` | class | `AVAudioUnit.h` | AudioUnitHandle + AudioUnitInfo |
| `AVAudioUnitComponent` | class | `AVAudioUnitComponent.h` | `AudioUnitComponentInfo + AudioUnitComponentManager::components()` |
| `AVAudioUnitComponentManager` | class | `AVAudioUnitComponent.h` | `AudioUnitComponentManager` |
| `AVAudioUnitComponentTagsDidChangeNotification` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitDelay` | class | `AVAudioUnitDelay.h` | `AudioUnitDelay` |
| `AVAudioUnitDistortion` | class | `AVAudioUnitDistortion.h` | `AudioUnitDistortion` |
| `AVAudioUnitDistortionPreset` | enum | `AVAudioUnitDistortion.h` | `AudioUnitDistortionPreset + AudioUnitDistortion::load_factory_preset()` |
| `AVAudioUnitEQ` | class | `AVAudioUnitEQ.h` | AudioUnitEQ |
| `AVAudioUnitEQFilterParameters` | class | `AVAudioUnitEQ.h` | AudioUnitEQ::{band_info, set_band_params} |
| `AVAudioUnitEQFilterType` | enum | `AVAudioUnitEQ.h` | AudioEQBandInfo.filter_type / AudioEQBandParams.filter_type |
| `AVAudioUnitEffect` | class | `AVAudioUnitEffect.h` | AudioUnitHandle::bypass() on AudioUnitEQ / AudioUnitReverb (partial) |
| `AVAudioUnitGenerator` | class | `AVAudioUnitGenerator.h` | AudioUnitGenerator |
| `AVAudioUnitMIDIInstrument` | class | `AVAudioUnitMIDIInstrument.h` | AudioUnitMIDIInstrument |
| `AVAudioUnitManufacturerNameApple` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitReverb` | class | `AVAudioUnitReverb.h` | AudioUnitReverb |
| `AVAudioUnitReverbPreset` | enum | `AVAudioUnitReverb.h` | AudioUnitReverbPreset |
| `AVAudioUnitSampler` | class | `AVAudioUnitSampler.h` | `AudioUnitSampler` |
| `AVAudioUnitTimeEffect` | class | `AVAudioUnitTimeEffect.h` | AudioUnitHandle::bypass() on AudioUnitTimePitch (partial) |
| `AVAudioUnitTimePitch` | class | `AVAudioUnitTimePitch.h` | AudioUnitTimePitch |
| `AVAudioUnitTypeEffect` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitTypeFormatConverter` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitTypeGenerator` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitTypeMIDIProcessor` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitTypeMixer` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitTypeMusicDevice` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitTypeMusicEffect` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitTypeOfflineEffect` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitTypeOutput` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitTypePanner` | constant | `AVAudioUnitComponent.h` | `AudioUnitComponentManager::standard_constants() + AudioUnitComponentConstants` |
| `AVAudioUnitVarispeed` | class | `AVAudioUnitVarispeed.h` | `AudioUnitVarispeed` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `AVAudio3DMixing` | protocol | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudioChannelLayout` | class | `AVAudioChannelLayout.h` | Foundational helper type is not wrapped yet. |
| `AVAudioCompressedBuffer` | class | `AVAudioBuffer.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioConnectionPoint` | class | `AVAudioConnectionPoint.h` | Foundational helper type is not wrapped yet. |
| `AVAudioConverterInputStatus` | enum | `AVAudioConverter.h` | Converter status or prime APIs are not exposed. |
| `AVAudioConverterOutputStatus` | enum | `AVAudioConverter.h` | Converter status or prime APIs are not exposed. |
| `AVAudioConverterPrimeInfo` | struct | `AVAudioConverter.h` | Converter status or prime APIs are not exposed. |
| `AVAudioConverterPrimeMethod` | enum | `AVAudioConverter.h` | Converter status or prime APIs are not exposed. |
| `AVAudioEngineConfigurationChangeNotification` | constant | `AVAudioEngine.h` | Manual-rendering or notification surface is not wrapped. |
| `AVAudioEngineManualRenderingBlock` | block | `AVAudioEngine.h` | Manual-rendering or notification surface is not wrapped. |
| `AVAudioEngineManualRenderingError` | enum | `AVAudioEngine.h` | Manual-rendering or notification surface is not wrapped. |
| `AVAudioEngineManualRenderingMode` | enum | `AVAudioEngine.h` | Manual-rendering or notification surface is not wrapped. |
| `AVAudioEngineManualRenderingStatus` | enum | `AVAudioEngine.h` | Manual-rendering or notification surface is not wrapped. |
| `AVAudioIONode` | class | `AVAudioIONode.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioIONodeInputBlock` | block | `AVAudioIONode.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioMixing` | protocol | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudioMixingDestination` | class | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudioNodeTapBlock` | block | `AVAudioNode.h` | Node tap/time helpers are not fully wrapped. |
| `AVAudioPlayerDelegate` | protocol | `AVAudioPlayer.h` | Delegate protocol is not bridged. |
| `AVAudioPlayerNodeBufferOptions` | enum | `AVAudioPlayerNode.h` | Buffer options or typed completion-callback APIs are not wrapped. |
| `AVAudioPlayerNodeCompletionCallbackType` | enum | `AVAudioPlayerNode.h` | Buffer options or typed completion-callback APIs are not wrapped. |
| `AVAudioPlayerNodeCompletionHandler` | block | `AVAudioPlayerNode.h` | Buffer options or typed completion-callback APIs are not wrapped. |
| `AVAudioRecorderDelegate` | protocol | `AVAudioRecorder.h` | Delegate protocol is not bridged. |
| `AVAudioRoutingArbiter` | class | `AVAudioRoutingArbiter.h` | No wrapper for routing-arbitration APIs. |
| `AVAudioRoutingArbitrationCategory` | enum | `AVAudioRoutingArbiter.h` | No wrapper for routing-arbitration APIs. |
| `AVAudioSessionCapability` | class | `AVAudioSessionRoute.h` | No public Rust wrapper; this standalone session symbol remains header-visible in the macOS SDK. |
| `AVAudioStereoMixing` | protocol | `AVAudioMixing.h` | No protocol-level mixing abstraction in the public Rust API. |
| `AVAudioTime` | class | `AVAudioTime.h` | Foundational helper type is not wrapped yet. |
| `AVAudioVoiceProcessingOtherAudioDuckingConfiguration` | struct | `AVAudioIONode.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioVoiceProcessingOtherAudioDuckingLevel` | enum | `AVAudioIONode.h` | No public Rust wrapper or matching Swift bridge thunk. |
| `AVAudioVoiceProcessingSpeechActivityEvent` | enum | `AVAudioIONode.h` | No public Rust wrapper or matching Swift bridge thunk. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `AVAudioApplicationMicrophoneInjectionPermission` | enum | `AVAudioApplication.h` | iOS / visionOS-only permission surface; the associated property and request API are `API_UNAVAILABLE(macos)`. | `AVAudioApplication.h:123-134 API_UNAVAILABLE(macos)` |

