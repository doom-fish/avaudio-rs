#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

pub type SimpleCallback = unsafe extern "C" fn(userdata: *mut c_void);
pub type DropCallback = unsafe extern "C" fn(userdata: *mut c_void);

extern "C" {
    pub fn ava_string_free(s: *mut c_char);

    pub fn av_audio_format_create_standard(
        sample_rate: f64,
        channel_count: u32,
        interleaved: bool,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_format_release(format: *mut c_void);
    pub fn av_audio_format_info_json(format: *mut c_void, out_error_message: *mut *mut c_char) -> *mut c_char;

    pub fn av_audio_file_open_for_reading(
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_file_release(file: *mut c_void);
    pub fn av_audio_file_info_json(file: *mut c_void, out_error_message: *mut *mut c_char) -> *mut c_char;
    pub fn av_audio_file_copy_processing_format(file: *mut c_void) -> *mut c_void;
    pub fn av_audio_file_copy_file_format(file: *mut c_void) -> *mut c_void;
    pub fn av_audio_file_read_pcm_buffer(
        file: *mut c_void,
        frame_count: u32,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;

    pub fn av_audio_pcm_buffer_release(buffer: *mut c_void);
    pub fn av_audio_pcm_buffer_info_json(
        buffer: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_pcm_buffer_copy_format(buffer: *mut c_void) -> *mut c_void;

    pub fn av_audio_engine_create(out_error_message: *mut *mut c_char) -> *mut c_void;
    pub fn av_audio_engine_release(engine: *mut c_void);
    pub fn av_audio_engine_info_json(engine: *mut c_void, out_error_message: *mut *mut c_char) -> *mut c_char;
    pub fn av_audio_engine_prepare(engine: *mut c_void);
    pub fn av_audio_engine_start(engine: *mut c_void, out_error_message: *mut *mut c_char) -> i32;
    pub fn av_audio_engine_stop(engine: *mut c_void);
    pub fn av_audio_engine_reset(engine: *mut c_void);
    pub fn av_audio_engine_attach_player_node(engine: *mut c_void, player: *mut c_void);
    pub fn av_audio_engine_connect_player_to_main_mixer(
        engine: *mut c_void,
        player: *mut c_void,
        format: *mut c_void,
    );
    pub fn av_audio_engine_copy_main_mixer_output_format(engine: *mut c_void, bus: usize) -> *mut c_void;

    pub fn av_audio_player_node_create(out_error_message: *mut *mut c_char) -> *mut c_void;
    pub fn av_audio_player_node_release(player: *mut c_void);
    pub fn av_audio_player_node_info_json(player: *mut c_void, out_error_message: *mut *mut c_char) -> *mut c_char;
    pub fn av_audio_player_node_play(player: *mut c_void);
    pub fn av_audio_player_node_pause(player: *mut c_void);
    pub fn av_audio_player_node_stop(player: *mut c_void);
    pub fn av_audio_player_node_schedule_buffer(
        player: *mut c_void,
        buffer: *mut c_void,
        callback: Option<SimpleCallback>,
        userdata: *mut c_void,
        drop_userdata: Option<DropCallback>,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn av_audio_player_node_schedule_file(
        player: *mut c_void,
        file: *mut c_void,
        callback: Option<SimpleCallback>,
        userdata: *mut c_void,
        drop_userdata: Option<DropCallback>,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FORMAT_ERROR: i32 = -2;
    pub const FILE_ERROR: i32 = -3;
    pub const ENGINE_ERROR: i32 = -4;
    pub const PLAYER_ERROR: i32 = -5;
    pub const CALLBACK_ERROR: i32 = -6;
    pub const OPERATION_FAILED: i32 = -7;
}
