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

NS_ASSUME_NONNULL_END
