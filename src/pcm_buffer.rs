//! [`PCMBuffer`] — `AVAudioPCMBuffer` wrappers.

#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

use core::ffi::{c_char, c_void};
use core::fmt;
use core::mem::size_of;
use core::ptr;
use core::slice;

use serde::Deserialize;

use crate::error::{from_swift, AVAudioError};
use crate::ffi;
use crate::format::{AudioFormat, AudioFormatInfo};
use crate::util::parse_json_and_free;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PCMBufferInfo {
    pub frame_capacity: u32,
    pub frame_length: u32,
    pub format: AudioFormatInfo,
}

pub struct PCMBuffer {
    pub(crate) ptr: *mut c_void,
}

impl fmt::Debug for PCMBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PCMBuffer").finish_non_exhaustive()
    }
}

mod sealed {
    use core::ffi::c_void;

    use crate::ffi::PCMBufferLayoutRaw;

    pub trait Sealed {
        fn channel_pointers(layout: &PCMBufferLayoutRaw) -> *const *mut c_void;
    }
}

pub trait PCMSample: sealed::Sealed + Copy + 'static {}

impl sealed::Sealed for f32 {
    fn channel_pointers(layout: &ffi::PCMBufferLayoutRaw) -> *const *mut c_void {
        layout.float_channel_data
    }
}

impl sealed::Sealed for i16 {
    fn channel_pointers(layout: &ffi::PCMBufferLayoutRaw) -> *const *mut c_void {
        layout.int16_channel_data
    }
}

impl sealed::Sealed for i32 {
    fn channel_pointers(layout: &ffi::PCMBufferLayoutRaw) -> *const *mut c_void {
        layout.int32_channel_data
    }
}

impl PCMSample for f32 {}
impl PCMSample for i16 {}
impl PCMSample for i32 {}

#[derive(Debug, PartialEq, Eq)]
pub enum PCMChannelData<'a, T> {
    Deinterleaved(Vec<&'a [T]>),
    Interleaved {
        channel_count: usize,
        samples: &'a [T],
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum PCMChannelDataMut<'a, T> {
    Deinterleaved(Vec<&'a mut [T]>),
    Interleaved {
        channel_count: usize,
        samples: &'a mut [T],
    },
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum PCMSamples {
    Float32(Vec<Vec<f32>>),
    Int16(Vec<Vec<i16>>),
    Int32(Vec<Vec<i32>>),
}

struct ChannelRegions {
    pointers: Vec<*mut c_void>,
    samples_per_region: usize,
    interleaved_channel_count: Option<usize>,
}

fn channel_regions<T: PCMSample>(layout: &ffi::PCMBufferLayoutRaw) -> Option<ChannelRegions> {
    let channels = T::channel_pointers(layout);
    if channels.is_null() {
        return None;
    }
    let channel_count = usize::try_from(layout.channel_count).ok()?;
    let buffer_count = usize::try_from(layout.buffer_count).ok()?;
    let stride = usize::try_from(layout.stride).ok()?;
    let frames = usize::try_from(layout.frame_length.min(layout.frame_capacity)).ok()?;
    let byte_limit = usize::try_from(layout.minimum_buffer_byte_size).ok()? / size_of::<T>();
    let (region_count, samples_per_region, interleaved_channel_count) = if layout.interleaved {
        if buffer_count != 1 || stride != channel_count || stride == 0 {
            return None;
        }
        let samples = frames.checked_mul(stride)?.min(byte_limit);
        (1, samples - samples % stride, Some(channel_count))
    } else {
        if buffer_count != channel_count || stride != 1 {
            return None;
        }
        (channel_count, frames.min(byte_limit), None)
    };
    let mut pointers = Vec::with_capacity(region_count);
    for index in 0..region_count {
        let pointer = unsafe { *channels.add(index) };
        if samples_per_region > 0 && (pointer.is_null() || !pointer.cast::<T>().is_aligned()) {
            return None;
        }
        pointers.push(pointer);
    }
    Some(ChannelRegions {
        pointers,
        samples_per_region,
        interleaved_channel_count,
    })
}

fn regions_are_disjoint<T>(regions: &ChannelRegions) -> bool {
    let length = regions.samples_per_region * size_of::<T>();
    let mut ranges: Vec<(usize, usize)> = regions
        .pointers
        .iter()
        .map(|pointer| (*pointer as usize, *pointer as usize + length))
        .collect();
    ranges.sort_unstable();
    ranges.windows(2).all(|pair| pair[0].1 <= pair[1].0)
}

const unsafe fn region_slice<'a, T>(pointer: *mut c_void, length: usize) -> &'a [T] {
    if length == 0 {
        &[]
    } else {
        slice::from_raw_parts(pointer.cast::<T>(), length)
    }
}

unsafe fn region_slice_mut<'a, T>(pointer: *mut c_void, length: usize) -> &'a mut [T] {
    if length == 0 {
        &mut []
    } else {
        slice::from_raw_parts_mut(pointer.cast::<T>(), length)
    }
}

fn split_channels<T: Copy>(data: &PCMChannelData<'_, T>) -> Vec<Vec<T>> {
    match data {
        PCMChannelData::Deinterleaved(channels) => {
            channels.iter().map(|channel| channel.to_vec()).collect()
        }
        PCMChannelData::Interleaved {
            channel_count,
            samples,
        } => (0..*channel_count)
            .map(|channel| {
                samples
                    .iter()
                    .skip(channel)
                    .step_by(*channel_count)
                    .copied()
                    .collect()
            })
            .collect(),
    }
}

impl Drop for PCMBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::av_audio_pcm_buffer_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl PCMBuffer {
    pub fn new(format: &AudioFormat, frame_capacity: u32) -> Result<Self, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let ptr =
            unsafe { ffi::av_audio_pcm_buffer_create(format.ptr, frame_capacity, &raw mut err) };
        if ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FORMAT_ERROR, err) });
        }
        Ok(Self { ptr })
    }

    pub fn info(&self) -> Result<PCMBufferInfo, AVAudioError> {
        let mut err: *mut c_char = ptr::null_mut();
        let json_ptr = unsafe { ffi::av_audio_pcm_buffer_info_json(self.ptr, &raw mut err) };
        if json_ptr.is_null() {
            return Err(unsafe { from_swift(ffi::status::FILE_ERROR, err) });
        }
        parse_json_and_free(json_ptr)
    }

    pub fn frame_capacity(&self) -> Result<u32, AVAudioError> {
        Ok(self.info()?.frame_capacity)
    }

    pub fn frame_length(&self) -> Result<u32, AVAudioError> {
        Ok(self.info()?.frame_length)
    }

    pub fn set_frame_length(&mut self, frame_length: u32) -> Result<(), AVAudioError> {
        self.ensure_writable()?;
        let mut err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::av_audio_pcm_buffer_set_frame_length(self.ptr, frame_length, &raw mut err)
        };
        if status != ffi::status::OK {
            return Err(unsafe { from_swift(status, err) });
        }
        Ok(())
    }

    pub fn format(&self) -> Result<AudioFormat, AVAudioError> {
        let ptr = unsafe { ffi::av_audio_pcm_buffer_copy_format(self.ptr) };
        if ptr.is_null() {
            return Err(AVAudioError::FormatError(
                "PCM buffer did not provide a format".into(),
            ));
        }
        Ok(AudioFormat { ptr })
    }

    pub fn is_scheduled(&self) -> bool {
        unsafe { ffi::ava_pcm_buffer_is_scheduled(self.ptr) }
    }

    pub(crate) fn ensure_writable(&self) -> Result<(), AVAudioError> {
        if self.is_scheduled() {
            return Err(AVAudioError::InvalidArgument(
                "PCM buffer is scheduled on a player node and cannot be modified until the player has consumed it".into(),
            ));
        }
        Ok(())
    }

    pub(crate) fn layout(&self) -> ffi::PCMBufferLayoutRaw {
        let mut layout = ffi::PCMBufferLayoutRaw {
            float_channel_data: ptr::null(),
            int16_channel_data: ptr::null(),
            int32_channel_data: ptr::null(),
            stride: 0,
            channel_count: 0,
            frame_length: 0,
            frame_capacity: 0,
            buffer_count: 0,
            minimum_buffer_byte_size: 0,
            interleaved: false,
            sample_rate: 0.0,
        };
        unsafe { ffi::ava_pcm_buffer_layout(self.ptr, &raw mut layout) };
        layout
    }

    pub fn channel_data<T: PCMSample>(&self) -> Option<PCMChannelData<'_, T>> {
        let regions = channel_regions::<T>(&self.layout())?;
        let length = regions.samples_per_region;
        Some(match regions.interleaved_channel_count {
            Some(channel_count) => PCMChannelData::Interleaved {
                channel_count,
                samples: unsafe { region_slice(regions.pointers[0], length) },
            },
            None => PCMChannelData::Deinterleaved(
                regions
                    .pointers
                    .iter()
                    .map(|pointer| unsafe { region_slice(*pointer, length) })
                    .collect(),
            ),
        })
    }

    pub fn channel_data_mut<T: PCMSample>(
        &mut self,
    ) -> Result<Option<PCMChannelDataMut<'_, T>>, AVAudioError> {
        self.ensure_writable()?;
        let Some(regions) = channel_regions::<T>(&self.layout()) else {
            return Ok(None);
        };
        if !regions_are_disjoint::<T>(&regions) {
            return Ok(None);
        }
        let length = regions.samples_per_region;
        Ok(Some(match regions.interleaved_channel_count {
            Some(channel_count) => PCMChannelDataMut::Interleaved {
                channel_count,
                samples: unsafe { region_slice_mut(regions.pointers[0], length) },
            },
            None => PCMChannelDataMut::Deinterleaved(
                regions
                    .pointers
                    .iter()
                    .map(|pointer| unsafe { region_slice_mut(*pointer, length) })
                    .collect(),
            ),
        }))
    }

    pub fn copy_samples(&self) -> Option<PCMSamples> {
        if let Some(data) = self.channel_data::<f32>() {
            return Some(PCMSamples::Float32(split_channels(&data)));
        }
        if let Some(data) = self.channel_data::<i16>() {
            return Some(PCMSamples::Int16(split_channels(&data)));
        }
        self.channel_data::<i32>()
            .map(|data| PCMSamples::Int32(split_channels(&data)))
    }
}
