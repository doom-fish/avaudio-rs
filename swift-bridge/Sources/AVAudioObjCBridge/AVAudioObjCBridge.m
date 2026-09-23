#import "AVAudioObjCBridge.h"

#import <objc/runtime.h>
#include <stdatomic.h>

@interface AVAXScheduleCounter : NSObject {
@public
    atomic_long _count;
}
@end

@implementation AVAXScheduleCounter

- (instancetype)init {
    self = [super init];
    if (self != nil) {
        atomic_init(&_count, 0);
    }
    return self;
}

@end

static char AVAXScheduleCounterKey;

void ava_pcm_buffer_layout(void *raw, AVAXPCMBufferLayout *layout) {
    AVAudioPCMBuffer *buffer = (__bridge AVAudioPCMBuffer *)raw;
    AVAudioFormat *format = buffer.format;
    const AudioBufferList *list = buffer.audioBufferList;
    uint32_t minimum = 0;
    for (UInt32 index = 0; index < list->mNumberBuffers; index++) {
        uint32_t size = list->mBuffers[index].mDataByteSize;
        if (index == 0 || size < minimum) {
            minimum = size;
        }
    }
    layout->floatChannelData = (void *const *)buffer.floatChannelData;
    layout->int16ChannelData = (void *const *)buffer.int16ChannelData;
    layout->int32ChannelData = (void *const *)buffer.int32ChannelData;
    layout->stride = buffer.stride;
    layout->channelCount = format.channelCount;
    layout->frameLength = buffer.frameLength;
    layout->frameCapacity = buffer.frameCapacity;
    layout->bufferCount = list->mNumberBuffers;
    layout->minimumBufferByteSize = minimum;
    layout->interleaved = format.isInterleaved;
    layout->sampleRate = format.sampleRate;
}

bool ava_pcm_buffer_is_scheduled(void *raw) {
    AVAudioPCMBuffer *buffer = (__bridge AVAudioPCMBuffer *)raw;
    AVAXScheduleCounter *counter = objc_getAssociatedObject(buffer, &AVAXScheduleCounterKey);
    return counter != nil && atomic_load(&counter->_count) > 0;
}
