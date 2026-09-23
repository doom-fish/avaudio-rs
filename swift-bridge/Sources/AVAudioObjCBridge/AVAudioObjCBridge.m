#import "AVAudioObjCBridge.h"

#import <objc/runtime.h>
#import <os/lock.h>
#include <stdatomic.h>

static NSString *const AVAXErrorDomain = @"AVAudioObjCBridge";

static BOOL AVAXFail(NSError *_Nullable *_Nullable error, NSInteger code, NSString *message) {
    if (error != NULL) {
        *error = [NSError errorWithDomain:AVAXErrorDomain
                                     code:code
                                 userInfo:@{NSLocalizedDescriptionKey: message}];
    }
    return NO;
}

static void AVAXSetError(NSError *_Nullable *_Nullable error, NSException *exception) {
    AVAXFail(error, 1, exception.reason ?: exception.name);
}

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

@interface AVAXScheduleToken : NSObject
- (instancetype)initWithCounter:(AVAXScheduleCounter *)counter;
- (void)finish;
@end

@implementation AVAXScheduleToken {
    AVAXScheduleCounter *_counter;
    atomic_bool _finished;
}

- (instancetype)initWithCounter:(AVAXScheduleCounter *)counter {
    self = [super init];
    if (self != nil) {
        _counter = counter;
        atomic_init(&_finished, false);
        atomic_fetch_add(&counter->_count, 1);
    }
    return self;
}

- (void)finish {
    if (!atomic_exchange(&_finished, true)) {
        atomic_fetch_sub(&_counter->_count, 1);
    }
}

- (void)dealloc {
    [self finish];
}

@end

static char AVAXScheduleCounterKey;
static os_unfair_lock AVAXScheduleCounterLock = OS_UNFAIR_LOCK_INIT;

static AVAXScheduleCounter *AVAXScheduleCounterForBuffer(AVAudioPCMBuffer *buffer) {
    os_unfair_lock_lock(&AVAXScheduleCounterLock);
    AVAXScheduleCounter *counter = objc_getAssociatedObject(buffer, &AVAXScheduleCounterKey);
    if (counter == nil) {
        counter = [[AVAXScheduleCounter alloc] init];
        objc_setAssociatedObject(buffer, &AVAXScheduleCounterKey, counter, OBJC_ASSOCIATION_RETAIN);
    }
    os_unfair_lock_unlock(&AVAXScheduleCounterLock);
    return counter;
}

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

BOOL AVAXEngineAttach(AVAudioEngine *engine, AVAudioNode *node, NSError *_Nullable *_Nullable error) {
    AVAudioEngine *owner = node.engine;
    if (owner != nil && owner != engine) {
        return AVAXFail(error, 2, @"node is attached to another engine");
    }
    @try {
        [engine attachNode:node];
        return YES;
    } @catch (NSException *exception) {
        AVAXSetError(error, exception);
        return NO;
    }
}

BOOL AVAXEngineConnect(
    AVAudioEngine *engine,
    AVAudioNode *source,
    AVAudioNode *destination,
    AVAudioFormat *_Nullable format,
    NSError *_Nullable *_Nullable error
) {
    if (source.engine != engine || destination.engine != engine) {
        return AVAXFail(error, 2, @"both nodes must be attached to this engine");
    }
    @try {
        [engine connect:source to:destination format:format];
        return YES;
    } @catch (NSException *exception) {
        AVAXSetError(error, exception);
        return NO;
    }
}

BOOL AVAXEnginePrepare(AVAudioEngine *engine, NSError *_Nullable *_Nullable error) {
    @try {
        [engine prepare];
        return YES;
    } @catch (NSException *exception) {
        AVAXSetError(error, exception);
        return NO;
    }
}

BOOL AVAXEngineStart(AVAudioEngine *engine, NSError *_Nullable *_Nullable error) {
    @try {
        return [engine startAndReturnError:error];
    } @catch (NSException *exception) {
        AVAXSetError(error, exception);
        return NO;
    }
}

BOOL AVAXNodeInstallTap(
    AVAudioNode *node,
    AVAudioNodeBus bus,
    AVAudioFrameCount bufferSize,
    AVAudioFormat *_Nullable format,
    AVAudioNodeTapBlock block,
    NSError *_Nullable *_Nullable error
) {
    if (node.engine == nil) {
        return AVAXFail(error, 2, @"node is not attached to an engine");
    }
    @try {
        [node installTapOnBus:bus bufferSize:bufferSize format:format block:block];
        return YES;
    } @catch (NSException *exception) {
        AVAXSetError(error, exception);
        return NO;
    }
}

BOOL AVAXPlayerPlay(AVAudioPlayerNode *player, NSError *_Nullable *_Nullable error) {
    if (player.engine == nil) {
        return AVAXFail(error, 2, @"player node is not attached to an engine");
    }
    @try {
        [player play];
        return YES;
    } @catch (NSException *exception) {
        AVAXSetError(error, exception);
        return NO;
    }
}

BOOL AVAXPlayerScheduleBuffer(
    AVAudioPlayerNode *player,
    AVAudioPCMBuffer *buffer,
    AVAudioTime *_Nullable when,
    AVAudioPlayerNodeBufferOptions options,
    AVAudioPlayerNodeCompletionCallbackType callbackType,
    void (^_Nullable completion)(AVAudioPlayerNodeCompletionCallbackType),
    NSError *_Nullable *_Nullable error
) {
    if (buffer.format.channelCount != [player outputFormatForBus:0].channelCount) {
        return AVAXFail(error, 2, @"buffer channel count does not match the player output format");
    }
    AVAXScheduleToken *token = [[AVAXScheduleToken alloc] initWithCounter:AVAXScheduleCounterForBuffer(buffer)];
    void (^handler)(AVAudioPlayerNodeCompletionCallbackType) = ^(AVAudioPlayerNodeCompletionCallbackType type) {
        [token finish];
        if (completion != nil) {
            completion(type);
        }
    };
    @try {
        [player scheduleBuffer:buffer
                        atTime:when
                       options:options
        completionCallbackType:callbackType
             completionHandler:handler];
        return YES;
    } @catch (NSException *exception) {
        [token finish];
        AVAXSetError(error, exception);
        return NO;
    }
}

BOOL AVAXPlayerScheduleFile(
    AVAudioPlayerNode *player,
    AVAudioFile *file,
    AVAudioTime *_Nullable when,
    AVAudioPlayerNodeCompletionCallbackType callbackType,
    void (^_Nullable completion)(AVAudioPlayerNodeCompletionCallbackType),
    NSError *_Nullable *_Nullable error
) {
    @try {
        [player scheduleFile:file atTime:when completionCallbackType:callbackType completionHandler:completion];
        return YES;
    } @catch (NSException *exception) {
        AVAXSetError(error, exception);
        return NO;
    }
}
