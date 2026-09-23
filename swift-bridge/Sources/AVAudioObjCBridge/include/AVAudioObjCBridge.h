#import <AVFoundation/AVFoundation.h>
#include <stdbool.h>
#include <stdint.h>

NS_ASSUME_NONNULL_BEGIN

typedef struct {
    void *const _Nullable *_Nullable floatChannelData;
    void *const _Nullable *_Nullable int16ChannelData;
    void *const _Nullable *_Nullable int32ChannelData;
    uint64_t stride;
    uint32_t channelCount;
    uint32_t frameLength;
    uint32_t frameCapacity;
    uint32_t bufferCount;
    uint32_t minimumBufferByteSize;
    bool interleaved;
    double sampleRate;
} AVAXPCMBufferLayout;

void ava_pcm_buffer_layout(void *buffer, AVAXPCMBufferLayout *layout);
bool ava_pcm_buffer_is_scheduled(void *buffer);

BOOL AVAXEngineAttach(AVAudioEngine *engine, AVAudioNode *node, NSError *_Nullable *_Nullable error);
BOOL AVAXEngineConnect(
    AVAudioEngine *engine,
    AVAudioNode *source,
    AVAudioNode *destination,
    AVAudioFormat *_Nullable format,
    NSError *_Nullable *_Nullable error
);
BOOL AVAXEnginePrepare(AVAudioEngine *engine, NSError *_Nullable *_Nullable error);
BOOL AVAXEngineStart(AVAudioEngine *engine, NSError *_Nullable *_Nullable error);
BOOL AVAXNodeInstallTap(
    AVAudioNode *node,
    AVAudioNodeBus bus,
    AVAudioFrameCount bufferSize,
    AVAudioFormat *_Nullable format,
    AVAudioNodeTapBlock block,
    NSError *_Nullable *_Nullable error
);
BOOL AVAXPlayerPlay(AVAudioPlayerNode *player, NSError *_Nullable *_Nullable error);
BOOL AVAXPlayerScheduleBuffer(
    AVAudioPlayerNode *player,
    AVAudioPCMBuffer *buffer,
    AVAudioTime *_Nullable when,
    AVAudioPlayerNodeBufferOptions options,
    AVAudioPlayerNodeCompletionCallbackType callbackType,
    void (^_Nullable completion)(AVAudioPlayerNodeCompletionCallbackType),
    NSError *_Nullable *_Nullable error
);
BOOL AVAXPlayerScheduleFile(
    AVAudioPlayerNode *player,
    AVAudioFile *file,
    AVAudioTime *_Nullable when,
    AVAudioPlayerNodeCompletionCallbackType callbackType,
    void (^_Nullable completion)(AVAudioPlayerNodeCompletionCallbackType),
    NSError *_Nullable *_Nullable error
);

NS_ASSUME_NONNULL_END
