# API Coverage

## AVAudioEngine
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioEngine.init()` | ✅ | `AudioEngine::new()` |
| `engine.prepare()` | ✅ | `AudioEngine::prepare()` |
| `engine.start()` | ✅ | `AudioEngine::start()` |
| `engine.stop()` | ✅ | `AudioEngine::stop()` |
| `engine.reset()` | ✅ | `AudioEngine::reset()` |
| `engine.attach(_:)` | ✅ | `AudioEngine::attach_node()` |
| `engine.connect(_:to:format:)` | ✅ | `AudioEngine::connect_nodes()` |
| `engine.connect(_:to:engine.mainMixerNode, format:)` | ✅ | `AudioEngine::connect_node_to_main_mixer()` |
| `engine.mainMixerNode` | ✅ | `AudioEngine::main_mixer_node()` |
| `engine.inputNode` | ✅ | `AudioEngine::input_node()` |
| `engine.outputNode` | ✅ | `AudioEngine::output_node()` |
| Manual rendering mode / status / format / sample time | ✅ | `AudioEngine::{enable_manual_rendering_mode, manual_rendering_info, manual_rendering_format, manual_rendering_sample_time}` |
| Offline/manual block rendering | ✅ | `render_offline()` / `manual_rendering_block_render()` |
| Configuration-change notification | ✅ | `AudioEngine::configuration_change_notification_name()` |

## AVAudioPlayerNode
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioPlayerNode.init()` | ✅ | `AudioPlayerNode::new()` |
| `play()` / `pause()` / `stop()` | ✅ | Direct wrappers |
| `scheduleBuffer(_:)` | ✅ | `AudioPlayerNode::schedule_buffer()` |
| `scheduleFile(_:)` | ✅ | `AudioPlayerNode::schedule_file()` |
| Completion handler scheduling | ✅ | Rust closure trampoline |
| Buffer options / typed completion callback types | ✅ | `AudioPlayerNodeBufferOptions` + `AudioPlayerNodeCompletionCallbackType` |
| `scheduleBuffer(_:at:options:)` | ✅ | `schedule_buffer_with_options()` |
| Typed completion scheduling overloads | ✅ | `schedule_buffer_with_callback_type()` / `schedule_file_with_callback_type()` |
| Graph attachment via `AVAudioNode` | ✅ | `AudioNodeHandle` + `AudioMixing` implementations |

## AVAudioMixerNode
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioMixerNode.init()` | ✅ | `AudioMixerNode::new()` |
| `outputVolume` | ✅ | Getter/setter |
| Use in engine graphs | ✅ | Implements `AudioNodeHandle` |

## AVAudioInputNode
| Symbol | Status | Notes |
|--------|--------|-------|
| `engine.inputNode` | ✅ | `AudioEngine::input_node()` |
| `outputFormat(forBus:)` | ✅ | `AudioInputNode::output_format()` |
| `inputFormat(forBus:)` | ✅ | `AudioInputNode::input_format()` |
| `installTap(onBus:bufferSize:format:block:)` | ✅ | Scaffold helper installs a no-op tap block |
| `removeTap(onBus:)` | ✅ | `AudioInputNode::remove_tap()` |
| Manual-rendering input block | ✅ | `set_manual_rendering_input_pcm_format_scaffold()` / `set_manual_rendering_input_pcm_format_with_callback()` |
| `presentationLatency` / `voiceProcessingEnabled` | ✅ | Via the shared `AudioIONode` trait |
| Input voice-processing bypass / AGC / mute | ✅ | Direct getters/setters |
| Speech-activity listener / ducking configuration | ✅ | Rust callback + `AudioVoiceProcessingOtherAudioDuckingConfiguration` |
| Mixing / stereo / 3D mixing protocols | ✅ | `AudioMixing`, `AudioStereoMixing`, `Audio3DMixing` trait impls |

## AVAudioOutputNode
| Symbol | Status | Notes |
|--------|--------|-------|
| `engine.outputNode` | ✅ | `AudioEngine::output_node()` |
| `outputFormat(forBus:)` | ✅ | `AudioOutputNode::output_format()` |
| `presentationLatency` / `voiceProcessingEnabled` | ✅ | Via the shared `AudioIONode` trait |

## AVAudioEnvironmentNode
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioEnvironmentNode.init()` | ✅ | `AudioEnvironmentNode::new()` |
| `listenerPosition` | ✅ | Getter/setter |
| `listenerAngularOrientation` | ✅ | Exposed as `AudioListenerOrientation` |
| `distanceAttenuationParameters` | ✅ | Exposed as `AudioDistanceAttenuation` |
| `reverbParameters.level` | ✅ | `set_reverb_blend()` / `reverb_blend()` |

## AVAudioUnit / AVAudioUnitEffect / subtype families
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioUnit.instantiate(with:options:completionHandler:)` | ✅ | `AudioUnit::instantiate()` |
| `AVAudioUnit.auAudioUnit` | ✅ | `AudioUnit::au_audio_unit()` / `AUAudioUnitHandle` |
| `AVAudioUnit.auAudioUnitPreset` loading | ✅ | `AudioUnitHandle::load_preset_at_path()` |
| Component description / manufacturer metadata | ✅ | `AudioComponentDescription`, `AudioUnitMetadata`, `AudioUnitComponentInfo` |
| Common node handle support | ✅ | `AudioUnitHandle` trait |
| Generic engine attachment | ✅ | All units also implement `AudioNodeHandle` |
| `bypass` | ✅ | `AudioUnitHandle::bypass()` / `set_bypass()` |
| Shared effect/time-effect state | ✅ | `AudioUnitHandle::unit_info()` |
| Generic `AVAudioUnitEffect` | ✅ | `AudioUnitEffect::with_component_description()` |
| Generic `AVAudioUnitTimeEffect` | ✅ | `AudioUnitTimeEffect::with_component_description()` |
| Generic `AVAudioUnitGenerator` | ✅ | `AudioUnitGenerator::with_component_description()` |
| Generic `AVAudioUnitMIDIInstrument` | ✅ | `AudioUnitMIDIInstrument::with_component_description()` |

## AVAudioUnitTimePitch
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioUnitTimePitch.init()` | ✅ | `AudioUnitTimePitch::new()` |
| `pitch` | ✅ | Getter/setter |
| `rate` | ✅ | Getter/setter |
| `overlap` | ✅ | Getter/setter |

## AVAudioUnitReverb
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioUnitReverb.init()` | ✅ | `AudioUnitReverb::new()` |
| `wetDryMix` | ✅ | Getter/setter |
| `loadFactoryPreset(_:)` | ✅ | `AudioUnitReverbPreset` enum |

## AVAudioUnitEQ
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioUnitEQ(numberOfBands:)` | ✅ | `AudioUnitEQ::new()` |
| `globalGain` | ✅ | Getter/setter |
| `bands.count` | ✅ | `AudioUnitEQ::band_count()` |
| Per-band filter type / frequency / bandwidth / gain / bypass | ✅ | `band_info()` + `set_band_params()` |

## AVAudioConverter
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioConverter.init(from:to:)` | ✅ | `AudioConverter::new()` |
| `convert(to:error:withInputFrom:)` | ✅ | One-shot buffer conversion helper |
| Converter format inspection | ✅ | `AudioConverter::info()` |
| Prime method / prime info | ✅ | `prime_method()`, `set_prime_method()`, `prime_info()`, `set_prime_info()` |
| Input/output status mirrors | ✅ | `AudioConverterInputStatus` + `AudioConverterOutputStatus` |
| Conversion status reporting | ✅ | `AudioConverter::convert_buffer_status()` |

## AVAudioPlayer
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioPlayer(contentsOf:)` | ✅ | `AudioSimplePlayer::create_from_path()` |
| `play()` / `pause()` / `stop()` | ✅ | Direct wrappers |
| `volume` / `pan` / `rate` | ✅ | Getter/setter |
| `duration` / `currentTime` | ✅ | Getter/setter where available |
| `isPlaying` | ✅ | `AudioSimplePlayer::is_playing()` |
| `numberOfLoops` | ✅ | Getter/setter |
| Delegate callbacks | ✅ | `AudioSimplePlayerDelegate` + `set_delegate()` / `clear_delegate()` |

## AVAudioRecorder
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioRecorder(url:settings:)` | ✅ | `AudioRecorder::create()` |
| `record()` / `pause()` / `stop()` | ✅ | Direct wrappers |
| `isRecording` / `currentTime` | ✅ | Direct wrappers |
| Metering (`isMeteringEnabled`, `updateMeters`, power queries) | ✅ | Exposed on the Rust wrapper |
| Permission prompts / entitlement handling | 🟡 | Runtime-managed by host app |
| Delegate callbacks | ✅ | `AudioRecorderDelegate` + `set_delegate()` / `clear_delegate()` |

## AVAudioSession
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioSession.sharedInstance().sampleRate` | ✅ | macOS compatibility stub returns `48_000.0` |
| `AVAudioSession.sharedInstance().outputVolume` | ✅ | macOS compatibility stub returns `1.0` |
| `AVAudioSession.sharedInstance().isOtherAudioPlaying` | ✅ | macOS stub returns `false` |
| Category / mode / activation APIs | ⏭️ | iOS-only API surface |

## AVAudioMixing / routing / helper types
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioMixing` / `AVAudioStereoMixing` / `AVAudio3DMixing` | ✅ | `AudioMixing`, `AudioStereoMixing`, and `Audio3DMixing` traits |
| `AVAudioMixingDestination` | ✅ | `AudioMixingDestination` + `connection_point()` |
| `AVAudioRoutingArbiter` | ✅ | `AudioRoutingArbiter::{shared, begin, leave}` |
| `AVAudioSessionCapability` | ✅ | `AudioSessionCapability` |
| `AVAudioChannelLayout` / `AVAudioConnectionPoint` / `AVAudioCompressedBuffer` / `AVAudioTime` | ✅ | Public helper wrappers |

## AVAudioApplication
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioApplication.shared` | ✅ | `AudioApplication::shared()` |
| `isInputMuted` | ✅ | `AudioApplication::input_muted()` |
| `setInputMuted(_:)` | ✅ | `AudioApplication::set_input_muted()` (host apps may need Apple\'s mute handler) |
| `recordPermission` | ✅ | `AudioApplicationRecordPermission` |
| `requestRecordPermission` | ✅ | Rust bool-callback trampoline |
| Microphone-injection permission | ⏭️ | iOS / visionOS only; unavailable on macOS |

## AVAudioSourceNode
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioSourceNode.init(renderBlock:)` | ✅ | `AudioSourceNode::new()` |
| `AVAudioSourceNode.init(format:renderBlock:)` | ✅ | `AudioSourceNode::new_with_format()` |
| Render block | ✅ | `AudioSourceRenderContext` callback |
| Use in engine graphs | ✅ | Implements `AudioNodeHandle` |

## AVAudioSinkNode
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioSinkNode.init(receiverBlock:)` | ✅ | `AudioSinkNode::new()` |
| Receiver block | ✅ | `AudioSinkRenderContext` callback |
| Use in engine graphs | ✅ | Implements `AudioNodeHandle` |

## AVAudioSequencer
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioSequencer.init(audioEngine:)` | ✅ | `AudioSequencer::with_engine()` |
| `load(from:options:)` / `load(from:data:options:)` | ✅ | `load_from_path_with_options()` / `load_from_data_with_options()` |
| `write(to:smpteResolution:replaceExisting:)` | ✅ | `AudioSequencer::write_to_path()` |
| `data(withSMPTEResolution:error:)` | ✅ | `AudioSequencer::data_with_smpte_resolution()` |
| `tracks` / `tempoTrack` / `createAndAppendTrack()` / `removeTrack(_:)` | ✅ | `track_at_index()`, `tracks()`, `tempo_track()`, `create_and_append_track()`, `remove_track()` |
| `userInfo` / `AVAudioSequencer.InfoDictionaryKey` | ✅ | `user_info()` + `AudioSequencerInfoDictionaryKeys` |
| `currentPositionInSeconds` / `currentPositionInBeats` | ✅ | Getter/setter wrappers |
| `rate` / `isPlaying` | ✅ | Direct wrappers |
| `secondsForBeats(_:)` / `beatsForSeconds(_:)` | ✅ | Direct wrappers |
| `hostTime(forBeats:error:)` / `beats(forHostTime:error:)` | ✅ | `host_time_for_beats()` / `beats_for_host_time()` |
| `prepareToPlay()` / `start()` / `stop()` | ✅ | Direct wrappers |
| `reverseEvents()` | ✅ | `AudioSequencer::reverse_events()` |
| `setUserCallback(_:)` | ✅ | `AudioSequencerUserEvent` callback |

## AVMusicTrack / AVMusicEvent subclasses
| Symbol | Status | Notes |
|--------|--------|-------|
| Track snapshots and loop/mute/solo/length editing | ✅ | `MusicTrack`, `MusicTrackInfo`, `BeatRange` |
| Destination audio unit routing | ✅ | `destination_audio_unit()` / `set_destination_audio_unit()` |
| `addEvent(_:at:)` | ✅ | `MusicTrack::add_event()` with `MusicEvent` subclasses |
| `moveEvents` / `clearEvents` / `cutEvents` | ✅ | Direct wrappers |
| `copyEvents` / `copyAndMergeEvents` | ✅ | Direct wrappers |
| `enumerateEvents(in:)` | ✅ | `enumerate_events_in_range()` / `events_in_range()` |
| MIDI / tempo / AU preset / user events | ✅ | `MusicEvent` enum + concrete event structs |

## AVAudioUnitComponentManager / AVAudioUnitComponent
| Symbol | Status | Notes |
|--------|--------|-------|
| `sharedAudioUnitComponentManager` | ✅ | `AudioUnitComponentManager::shared()` |
| `tagNames` / `standardLocalizedTagNames` | ✅ | JSON-backed vectors |
| Component discovery | ✅ | `components()` returns `AudioUnitComponentInfo` snapshots |
| Standard type/manufacturer constants | ✅ | `AudioUnitComponentConstants` |
| Tag-change notification constant | ✅ | `AudioUnitComponentConstants::tags_did_change_notification` |

## AVAudioUnitDelay
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioUnitDelay.init()` | ✅ | `AudioUnitDelay::new()` |
| `delayTime` / `feedback` / `lowPassCutoff` / `wetDryMix` | ✅ | Getter/setter wrappers |

## AVAudioUnitDistortion
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioUnitDistortion.init()` | ✅ | `AudioUnitDistortion::new()` |
| `preGain` / `wetDryMix` | ✅ | Getter/setter wrappers |
| `loadFactoryPreset(_:)` | ✅ | `AudioUnitDistortionPreset` enum |

## AVAudioUnitSampler
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioUnitSampler.init()` | ✅ | `AudioUnitSampler::new()` |
| `loadInstrument(at:)` | ✅ | `AudioUnitSampler::load_instrument()` |
| `loadAudioFiles(at:)` | ✅ | `AudioUnitSampler::load_audio_files()` |
| `loadSoundBankInstrument(at:program:bankMSB:bankLSB:)` | ✅ | `AudioUnitSampler::load_sound_bank_instrument()` |
| `stereoPan` / `overallGain` / `globalTuning` | ✅ | Getter/setter wrappers |
| `masterGain` | ✅ | Getter/setter wrapper for the deprecated API |
| Shared MIDI-instrument send methods | ✅ | `AudioUnitMIDIInstrumentHandle` implementation |

## AVAudioUnitVarispeed
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioUnitVarispeed.init()` | ✅ | `AudioUnitVarispeed::new()` |
| `rate` | ✅ | Getter/setter wrappers |

## AVAudioBuffer
| Symbol | Status | Notes |
|--------|--------|-------|
| `format` | ✅ | `PCMBuffer::format()` |
| `audioBufferList` / `mutableAudioBufferList` inspection | ✅ | `AudioBufferHandle::buffer_info()` |
| `AVAudioCompressedBuffer` | ✅ | `AudioCompressedBuffer` |

## AVAudioFile
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioFile(forReading:)` | ✅ | `AudioFile::open_for_reading()` |
| Processing/file format inspection | ✅ | JSON-backed info payloads |
| `read(into:frameCount:)` | ✅ | `AudioFile::read_pcm_buffer()` |

## AVAudioPCMBuffer
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioPCMBuffer(pcmFormat:frameCapacity:)` | ✅ | `PCMBuffer::new()` |
| `frameLength` | ✅ | Getter + `set_frame_length()` |
| `format` | ✅ | `PCMBuffer::format()` |

## AVAudioFormat
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioFormat(commonFormat:sampleRate:channels:interleaved:)` | ✅ | `AudioFormat::standard()` |
| `commonFormat` / `sampleRate` / `channelCount` / `interleaved` | ✅ | Individual Rust accessors |

## Shared AVAudioTypes / AVAudioSettings / AVAudioSessionTypes mirrors
| Symbol | Status | Notes |
|--------|--------|-------|
| Core numeric typealiases (`AVAudioChannelCount`, `AVAudioFrameCount`, `AVAudioFramePosition`, `AVAudioPacketCount`, `AVAudioNodeBus`) | ✅ | Public Rust type aliases |
| 3D helper vectors / orientations and mixing enums | ✅ | `Audio3DVector`, `Audio3DVectorOrientation`, `Audio3DMixing*` enums |
| Settings enums/constants (`AVAudioQuality`, `AVAudioContentSource`, `AVAudioDynamicRangeControlConfiguration`, file/bit-rate keys) | ✅ | `AudioSettingsConstants` + enum mirrors |
| Session option/enums mirrors | ✅ | `AudioSession*` option sets and enums |
