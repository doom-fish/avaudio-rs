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

## AVAudioPlayerNode
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioPlayerNode.init()` | ✅ | `AudioPlayerNode::new()` |
| `play()` / `pause()` / `stop()` | ✅ | Direct wrappers |
| `scheduleBuffer(_:)` | ✅ | `AudioPlayerNode::schedule_buffer()` |
| `scheduleFile(_:)` | ✅ | `AudioPlayerNode::schedule_file()` |
| Completion handler scheduling | ✅ | Rust closure trampoline |
| Graph attachment via `AVAudioNode` | ✅ | `AudioNodeHandle` implementation |

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
| `installTap(onBus:bufferSize:format:block:)` | 🟡 | Scaffold only; tap discards buffers |
| `removeTap(onBus:)` | ✅ | `AudioInputNode::remove_tap()` |

## AVAudioOutputNode
| Symbol | Status | Notes |
|--------|--------|-------|
| `engine.outputNode` | ✅ | `AudioEngine::output_node()` |
| `outputFormat(forBus:)` | ✅ | `AudioOutputNode::output_format()` |

## AVAudioEnvironmentNode
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioEnvironmentNode.init()` | ✅ | `AudioEnvironmentNode::new()` |
| `listenerPosition` | ✅ | Getter/setter |
| `listenerAngularOrientation` | ✅ | Exposed as `AudioListenerOrientation` |
| `distanceAttenuationParameters` | ✅ | Exposed as `AudioDistanceAttenuation` |
| `reverbParameters.level` | ✅ | `set_reverb_blend()` / `reverb_blend()` |

## AVAudioUnitEffect / AVAudioUnit-backed nodes
| Symbol | Status | Notes |
|--------|--------|-------|
| Common node handle support | ✅ | `AudioUnitHandle` trait |
| Generic engine attachment | ✅ | All units also implement `AudioNodeHandle` |
| `bypass` | ✅ | `AudioUnitHandle::bypass()` / `set_bypass()` |
| Shared effect/time-effect state | ✅ | `AudioUnitHandle::unit_info()` |

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

## AVAudioPlayer
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioPlayer(contentsOf:)` | ✅ | `AudioSimplePlayer::create_from_path()` |
| `play()` / `pause()` / `stop()` | ✅ | Direct wrappers |
| `volume` / `pan` / `rate` | ✅ | Getter/setter |
| `duration` / `currentTime` | ✅ | Getter/setter where available |
| `isPlaying` | ✅ | `AudioSimplePlayer::is_playing()` |
| `numberOfLoops` | ✅ | Getter/setter |
| Delegate callbacks | ⏭️ | Not yet bridged |

## AVAudioRecorder
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioRecorder(url:settings:)` | ✅ | `AudioRecorder::create()` |
| `record()` / `pause()` / `stop()` | ✅ | Direct wrappers |
| `isRecording` / `currentTime` | ✅ | Direct wrappers |
| Metering (`isMeteringEnabled`, `updateMeters`, power queries) | ✅ | Exposed on the Rust wrapper |
| Permission prompts / entitlement handling | 🟡 | Runtime-managed by host app |
| Delegate callbacks | ⏭️ | Not yet bridged |

## AVAudioSession
| Symbol | Status | Notes |
|--------|--------|-------|
| `AVAudioSession.sharedInstance().sampleRate` | ✅ | macOS compatibility stub returns `48_000.0` |
| `AVAudioSession.sharedInstance().outputVolume` | ✅ | macOS compatibility stub returns `1.0` |
| `AVAudioSession.sharedInstance().isOtherAudioPlaying` | ✅ | macOS stub returns `false` |
| Category / mode / activation APIs | ⏭️ | iOS-only API surface |

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
| `load(from:options:)` | ✅ | `AudioSequencer::load_from_path()` |
| `currentPositionInSeconds` / `currentPositionInBeats` | ✅ | Getter/setter wrappers |
| `rate` / `isPlaying` | ✅ | Direct wrappers |
| `secondsForBeats(_:)` / `beatsForSeconds(_:)` | ✅ | Direct wrappers |
| `prepareToPlay()` / `start()` / `stop()` | ✅ | Direct wrappers |
| `reverseEvents()` | ✅ | `AudioSequencer::reverse_events()` |
| `setUserCallback(_:)` | ✅ | `AudioSequencerUserEvent` callback |

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
| `loadSoundBankInstrument(at:program:bankMSB:bankLSB:)` | ✅ | `AudioUnitSampler::load_sound_bank_instrument()` |
| `stereoPan` / `overallGain` / `globalTuning` | ✅ | Getter/setter wrappers |

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
