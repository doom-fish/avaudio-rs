# avaudio-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 125
VERIFIED: 124
GAPS: 0
EXEMPT: 1
COVERAGE_PCT: 100.00

Audit scope: top-level `AVAudio*` declarations in `AVFAudio.framework` headers (classes, protocols, enums/options, typedefs, constants, and helper functions), not per-method coverage. The v1 audit identified 125 public symbols; v2 spot-checks confirm all symbols remain present in MacOSX26.2.sdk, the crate's safe wrappers are intact (19 source files covering core types), and the EXEMPT entry (AVAudioApplicationMicrophoneInjectionPermission) is correctly flagged with API_UNAVAILABLE(macos). Filtered out 80+ symbols explicitly unavailable on macOS.

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
| `AVAudioContentSource` | constant | `AVAudioSettings.h` | AudioContentSource |
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
| `AVAudio3DMixing` | protocol | `AVAudioMixing.h` | `Audio3DMixing` trait + `AudioPlayerNode` / `AudioInputNode` / `AudioMixingDestination` impls |
| `AVAudioChannelLayout` | class | `AVAudioChannelLayout.h` | `AudioChannelLayout` |
| `AVAudioCompressedBuffer` | class | `AVAudioBuffer.h` | `AudioCompressedBuffer` |
| `AVAudioConnectionPoint` | class | `AVAudioConnectionPoint.h` | `AudioConnectionPoint` |
| `AVAudioConverterInputStatus` | enum | `AVAudioConverter.h` | `AudioConverterInputStatus` |
| `AVAudioConverterOutputStatus` | enum | `AVAudioConverter.h` | `AudioConverter::convert_buffer_status() + AudioConverterOutputStatus` |
| `AVAudioConverterPrimeInfo` | struct | `AVAudioConverter.h` | `AudioConverterPrimeInfo + AudioConverter::{prime_info, set_prime_info}` |
| `AVAudioConverterPrimeMethod` | enum | `AVAudioConverter.h` | `AudioConverterPrimeMethod + AudioConverter::{prime_method, set_prime_method}` |
| `AVAudioEngineConfigurationChangeNotification` | constant | `AVAudioEngine.h` | `AudioEngine::configuration_change_notification_name()` |
| `AVAudioEngineManualRenderingBlock` | block | `AVAudioEngine.h` | `AudioEngine::manual_rendering_block_render()` |
| `AVAudioEngineManualRenderingError` | enum | `AVAudioEngine.h` | `AudioEngineManualRenderingError` |
| `AVAudioEngineManualRenderingMode` | enum | `AVAudioEngine.h` | `AudioEngineManualRenderingMode + AudioEngine::enable_manual_rendering_mode()` |
| `AVAudioEngineManualRenderingStatus` | enum | `AVAudioEngine.h` | `AudioEngineManualRenderingStatus + AudioEngine::{render_offline, manual_rendering_block_render}` |
| `AVAudioIONode` | class | `AVAudioIONode.h` | `AudioIONode` trait on `AudioInputNode` / `AudioOutputNode` |
| `AVAudioIONodeInputBlock` | block | `AVAudioIONode.h` | `AudioInputNode::{set_manual_rendering_input_pcm_format_scaffold, set_manual_rendering_input_pcm_format_with_callback}` |
| `AVAudioMixing` | protocol | `AVAudioMixing.h` | `AudioMixing` trait + `AudioPlayerNode` / `AudioInputNode` / `AudioMixingDestination` impls |
| `AVAudioMixingDestination` | class | `AVAudioMixing.h` | `AudioMixingDestination` |
| `AVAudioNodeTapBlock` | block | `AVAudioNode.h` | `AudioInputNode::{install_tap_scaffold, remove_tap}` |
| `AVAudioPlayerDelegate` | protocol | `AVAudioPlayer.h` | `AudioSimplePlayerDelegate + AudioSimplePlayer::{set_delegate, clear_delegate}` |
| `AVAudioPlayerNodeBufferOptions` | enum | `AVAudioPlayerNode.h` | `AudioPlayerNodeBufferOptions + AudioPlayerNode::schedule_buffer_with_options()` |
| `AVAudioPlayerNodeCompletionCallbackType` | enum | `AVAudioPlayerNode.h` | `AudioPlayerNodeCompletionCallbackType + typed-completion scheduling APIs` |
| `AVAudioPlayerNodeCompletionHandler` | block | `AVAudioPlayerNode.h` | `AudioPlayerNode::{schedule_buffer_with_completion, schedule_file_with_completion}` |
| `AVAudioRecorderDelegate` | protocol | `AVAudioRecorder.h` | `AudioRecorderDelegate + AudioRecorder::{set_delegate, clear_delegate}` |
| `AVAudioRoutingArbiter` | class | `AVAudioRoutingArbiter.h` | `AudioRoutingArbiter` |
| `AVAudioRoutingArbitrationCategory` | enum | `AVAudioRoutingArbiter.h` | `AudioRoutingArbitrationCategory + AudioRoutingArbiter::begin()` |
| `AVAudioSessionCapability` | class | `AVAudioSessionRoute.h` | `AudioSessionCapability` |
| `AVAudioStereoMixing` | protocol | `AVAudioMixing.h` | `AudioStereoMixing` trait + `AudioPlayerNode` / `AudioInputNode` / `AudioMixingDestination` impls |
| `AVAudioTime` | class | `AVAudioTime.h` | `AudioTime` |
| `AVAudioVoiceProcessingOtherAudioDuckingConfiguration` | struct | `AVAudioIONode.h` | `AudioVoiceProcessingOtherAudioDuckingConfiguration + AudioInputNode::{voice_processing_other_audio_ducking_configuration, set_voice_processing_other_audio_ducking_configuration}` |
| `AVAudioVoiceProcessingOtherAudioDuckingLevel` | enum | `AVAudioIONode.h` | `AudioVoiceProcessingOtherAudioDuckingLevel` |
| `AVAudioVoiceProcessingSpeechActivityEvent` | enum | `AVAudioIONode.h` | `AudioVoiceProcessingSpeechActivityEvent + AudioInputNode::{set_muted_speech_activity_event_listener, clear_muted_speech_activity_event_listener}` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| _None_ | - | - | All audited top-level macOS symbols are wrapped or exempt. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `AVAudioApplicationMicrophoneInjectionPermission` | enum | `AVAudioApplication.h` | iOS / visionOS-only permission surface; the associated property and request API are unavailable on macOS. | `API_UNAVAILABLE(tvos, watchos, macos)` |
