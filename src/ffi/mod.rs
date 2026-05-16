#![allow(dead_code, missing_docs)]

use core::ffi::{c_char, c_void};

pub type SimpleCallback = unsafe extern "C" fn(userdata: *mut c_void);
pub type DropCallback = unsafe extern "C" fn(userdata: *mut c_void);
pub type BoolCallback = unsafe extern "C" fn(userdata: *mut c_void, value: bool);
pub type SourceNodeRenderCallback = unsafe extern "C" fn(
    userdata: *mut c_void,
    is_silence: *mut bool,
    timestamp: *const c_void,
    frame_count: u32,
    output_data: *mut c_void,
) -> i32;
pub type SinkNodeReceiverCallback = unsafe extern "C" fn(
    userdata: *mut c_void,
    timestamp: *const c_void,
    frame_count: u32,
    input_data: *const c_void,
) -> i32;
pub type SequencerUserCallback = unsafe extern "C" fn(
    userdata: *mut c_void,
    track_ptr: *mut c_void,
    bytes_ptr: *const u8,
    bytes_len: usize,
    beat: f64,
);

extern "C" {
    pub fn ava_string_free(s: *mut c_char);
    pub fn av_audio_node_release(node: *mut c_void);
    pub fn av_audio_buffer_info_json(
        buffer: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;

    pub fn av_audio_format_create_standard(
        sample_rate: f64,
        channel_count: u32,
        interleaved: bool,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_format_release(format: *mut c_void);
    pub fn av_audio_format_info_json(
        format: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;

    pub fn av_audio_file_open_for_reading(
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_file_release(file: *mut c_void);
    pub fn av_audio_file_info_json(
        file: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_file_copy_processing_format(file: *mut c_void) -> *mut c_void;
    pub fn av_audio_file_copy_file_format(file: *mut c_void) -> *mut c_void;
    pub fn av_audio_file_read_pcm_buffer(
        file: *mut c_void,
        frame_count: u32,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;

    pub fn av_audio_pcm_buffer_create(
        format: *mut c_void,
        frame_capacity: u32,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_pcm_buffer_release(buffer: *mut c_void);
    pub fn av_audio_pcm_buffer_info_json(
        buffer: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_pcm_buffer_copy_format(buffer: *mut c_void) -> *mut c_void;
    pub fn av_audio_pcm_buffer_set_frame_length(
        buffer: *mut c_void,
        frame_length: u32,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn av_audio_engine_create(out_error_message: *mut *mut c_char) -> *mut c_void;
    pub fn av_audio_engine_release(engine: *mut c_void);
    pub fn av_audio_engine_info_json(
        engine: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
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
    pub fn av_audio_engine_copy_main_mixer_output_format(
        engine: *mut c_void,
        bus: usize,
    ) -> *mut c_void;
    pub fn av_audio_engine_attach_node(engine: *mut c_void, node: *mut c_void);
    pub fn av_audio_engine_connect_nodes(
        engine: *mut c_void,
        from_node: *mut c_void,
        to_node: *mut c_void,
        format: *mut c_void,
    );
    pub fn av_audio_engine_connect_node_to_main_mixer(
        engine: *mut c_void,
        node: *mut c_void,
        format: *mut c_void,
    );
    pub fn av_audio_engine_get_main_mixer_node(engine: *mut c_void) -> *mut c_void;
    pub fn av_audio_engine_get_input_node(engine: *mut c_void) -> *mut c_void;
    pub fn av_audio_engine_get_output_node(engine: *mut c_void) -> *mut c_void;

    pub fn av_audio_player_node_create(out_error_message: *mut *mut c_char) -> *mut c_void;
    pub fn av_audio_player_node_release(player: *mut c_void);
    pub fn av_audio_player_node_get_node_unretained(player: *mut c_void) -> *mut c_void;
    pub fn av_audio_player_node_info_json(
        player: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
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

    pub fn av_audio_mixer_node_create() -> *mut c_void;
    pub fn av_audio_mixer_node_release(node: *mut c_void);
    pub fn av_audio_mixer_node_get_output_volume(node: *mut c_void) -> f32;
    pub fn av_audio_mixer_node_set_output_volume(node: *mut c_void, volume: f32);

    pub fn av_audio_input_node_release(node: *mut c_void);
    pub fn av_audio_input_node_output_format_json(
        node: *mut c_void,
        bus: i32,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_input_node_input_format_json(
        node: *mut c_void,
        bus: i32,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_input_node_install_tap_scaffold(
        node: *mut c_void,
        bus: i32,
        buffer_size: u32,
        format: *mut c_void,
    ) -> i32;
    pub fn av_audio_input_node_remove_tap(node: *mut c_void, bus: i32);

    pub fn av_audio_output_node_release(node: *mut c_void);
    pub fn av_audio_output_node_output_format_json(
        node: *mut c_void,
        bus: i32,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;

    pub fn av_audio_environment_node_create() -> *mut c_void;
    pub fn av_audio_environment_node_release(node: *mut c_void);
    pub fn av_audio_environment_node_set_listener_position(
        node: *mut c_void,
        x: f32,
        y: f32,
        z: f32,
    );
    pub fn av_audio_environment_node_get_listener_position_json(
        node: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_environment_node_set_listener_orientation(
        node: *mut c_void,
        yaw: f32,
        pitch: f32,
        roll: f32,
    );
    pub fn av_audio_environment_node_get_listener_orientation_json(
        node: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_environment_node_set_distance_attenuation(
        node: *mut c_void,
        model: i32,
        reference_distance: f32,
        maximum_distance: f32,
        rolloff_factor: f32,
    );
    pub fn av_audio_environment_node_get_distance_attenuation_json(
        node: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_environment_node_set_reverb_blend(node: *mut c_void, blend: f32);
    pub fn av_audio_environment_node_get_reverb_blend(node: *mut c_void) -> f32;

    pub fn av_audio_unit_info_json(
        unit: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_unit_get_bypass(unit: *mut c_void) -> bool;
    pub fn av_audio_unit_set_bypass(unit: *mut c_void, bypass: bool);

    pub fn av_audio_unit_time_pitch_create() -> *mut c_void;
    pub fn av_audio_unit_time_pitch_release(node: *mut c_void);
    pub fn av_audio_unit_time_pitch_get_pitch(node: *mut c_void) -> f32;
    pub fn av_audio_unit_time_pitch_set_pitch(node: *mut c_void, pitch: f32);
    pub fn av_audio_unit_time_pitch_get_rate(node: *mut c_void) -> f32;
    pub fn av_audio_unit_time_pitch_set_rate(node: *mut c_void, rate: f32);
    pub fn av_audio_unit_time_pitch_get_overlap(node: *mut c_void) -> f32;
    pub fn av_audio_unit_time_pitch_set_overlap(node: *mut c_void, overlap: f32);

    pub fn av_audio_unit_reverb_create() -> *mut c_void;
    pub fn av_audio_unit_reverb_release(node: *mut c_void);
    pub fn av_audio_unit_reverb_get_wet_dry_mix(node: *mut c_void) -> f32;
    pub fn av_audio_unit_reverb_set_wet_dry_mix(node: *mut c_void, mix: f32);
    pub fn av_audio_unit_reverb_load_factory_preset(node: *mut c_void, preset: i32);

    pub fn av_audio_unit_eq_create(
        band_count: i32,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_unit_eq_release(node: *mut c_void);
    pub fn av_audio_unit_eq_get_global_gain(node: *mut c_void) -> f32;
    pub fn av_audio_unit_eq_set_global_gain(node: *mut c_void, gain: f32);
    pub fn av_audio_unit_eq_get_band_count(node: *mut c_void) -> i32;
    pub fn av_audio_unit_eq_get_band_info_json(
        node: *mut c_void,
        band_index: i32,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_unit_eq_set_band_params(
        node: *mut c_void,
        band_index: i32,
        filter_type: i32,
        frequency: f32,
        bandwidth: f32,
        gain: f32,
        bypass: bool,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn av_audio_converter_create(
        input_format: *mut c_void,
        output_format: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_converter_release(converter: *mut c_void);
    pub fn av_audio_converter_info_json(
        converter: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_converter_convert_buffer(
        converter: *mut c_void,
        input_buffer: *mut c_void,
        output_buffer: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn av_audio_simple_player_create_from_path(
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_simple_player_release(player: *mut c_void);
    pub fn av_audio_simple_player_play(player: *mut c_void) -> bool;
    pub fn av_audio_simple_player_pause(player: *mut c_void);
    pub fn av_audio_simple_player_stop(player: *mut c_void);
    pub fn av_audio_simple_player_get_volume(player: *mut c_void) -> f32;
    pub fn av_audio_simple_player_set_volume(player: *mut c_void, volume: f32);
    pub fn av_audio_simple_player_get_pan(player: *mut c_void) -> f32;
    pub fn av_audio_simple_player_set_pan(player: *mut c_void, pan: f32);
    pub fn av_audio_simple_player_get_rate(player: *mut c_void) -> f32;
    pub fn av_audio_simple_player_set_rate(player: *mut c_void, rate: f32);
    pub fn av_audio_simple_player_get_duration(player: *mut c_void) -> f64;
    pub fn av_audio_simple_player_get_current_time(player: *mut c_void) -> f64;
    pub fn av_audio_simple_player_set_current_time(player: *mut c_void, time: f64);
    pub fn av_audio_simple_player_is_playing(player: *mut c_void) -> bool;
    pub fn av_audio_simple_player_get_number_of_loops(player: *mut c_void) -> i32;
    pub fn av_audio_simple_player_set_number_of_loops(player: *mut c_void, loop_count: i32);
    pub fn av_audio_simple_player_prepare_to_play(player: *mut c_void) -> bool;

    pub fn av_audio_recorder_create(
        path: *const c_char,
        sample_rate: f64,
        channels: i32,
        bit_depth: i32,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_recorder_release(recorder: *mut c_void);
    pub fn av_audio_recorder_record(recorder: *mut c_void) -> bool;
    pub fn av_audio_recorder_stop(recorder: *mut c_void);
    pub fn av_audio_recorder_pause(recorder: *mut c_void);
    pub fn av_audio_recorder_is_recording(recorder: *mut c_void) -> bool;
    pub fn av_audio_recorder_current_time(recorder: *mut c_void) -> f64;
    pub fn av_audio_recorder_set_metering_enabled(recorder: *mut c_void, enabled: bool);
    pub fn av_audio_recorder_update_meters(recorder: *mut c_void);
    pub fn av_audio_recorder_average_power(recorder: *mut c_void, channel: i32) -> f32;
    pub fn av_audio_recorder_peak_power(recorder: *mut c_void, channel: i32) -> f32;
    pub fn av_audio_recorder_delete_recording(recorder: *mut c_void) -> bool;

    pub fn av_audio_session_get_sample_rate() -> f64;
    pub fn av_audio_session_get_output_volume() -> f32;
    pub fn av_audio_session_is_other_audio_playing() -> bool;
    pub fn av_audio_application_get_input_muted(
        out_error_message: *mut *mut c_char,
    ) -> bool;
    pub fn av_audio_application_set_input_muted(
        muted: bool,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn av_audio_application_get_record_permission(
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn av_audio_application_request_record_permission(
        callback: Option<BoolCallback>,
        userdata: *mut c_void,
        drop_userdata: Option<DropCallback>,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn av_audio_source_node_create(
        callback: Option<SourceNodeRenderCallback>,
        userdata: *mut c_void,
        drop_userdata: Option<DropCallback>,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_source_node_create_with_format(
        format: *mut c_void,
        callback: Option<SourceNodeRenderCallback>,
        userdata: *mut c_void,
        drop_userdata: Option<DropCallback>,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_source_node_release(node: *mut c_void);

    pub fn av_audio_sink_node_create(
        callback: Option<SinkNodeReceiverCallback>,
        userdata: *mut c_void,
        drop_userdata: Option<DropCallback>,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_sink_node_release(node: *mut c_void);

    pub fn av_audio_sequencer_create(out_error_message: *mut *mut c_char) -> *mut c_void;
    pub fn av_audio_sequencer_create_with_engine(
        engine: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn av_audio_sequencer_release(sequencer: *mut c_void);
    pub fn av_audio_sequencer_info_json(
        sequencer: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_sequencer_load_from_url(
        sequencer: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn av_audio_sequencer_reverse_events(
        sequencer: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn av_audio_sequencer_set_current_position_in_seconds(sequencer: *mut c_void, seconds: f64);
    pub fn av_audio_sequencer_set_current_position_in_beats(sequencer: *mut c_void, beats: f64);
    pub fn av_audio_sequencer_set_rate(sequencer: *mut c_void, rate: f32);
    pub fn av_audio_sequencer_seconds_for_beats(sequencer: *mut c_void, beats: f64) -> f64;
    pub fn av_audio_sequencer_beats_for_seconds(sequencer: *mut c_void, seconds: f64) -> f64;
    pub fn av_audio_sequencer_prepare_to_play(sequencer: *mut c_void);
    pub fn av_audio_sequencer_start(
        sequencer: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn av_audio_sequencer_stop(sequencer: *mut c_void);
    pub fn av_audio_sequencer_set_user_callback(
        sequencer: *mut c_void,
        callback: Option<SequencerUserCallback>,
        userdata: *mut c_void,
        drop_userdata: Option<DropCallback>,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn av_audio_unit_component_manager_tag_names_json(
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_unit_component_manager_standard_localized_tag_names_json(
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_unit_component_manager_components_json(
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn av_audio_unit_component_constants_json(
        out_error_message: *mut *mut c_char,
    ) -> *mut c_char;

    pub fn av_audio_unit_delay_create() -> *mut c_void;
    pub fn av_audio_unit_delay_release(node: *mut c_void);
    pub fn av_audio_unit_delay_get_delay_time(node: *mut c_void) -> f64;
    pub fn av_audio_unit_delay_set_delay_time(node: *mut c_void, delay_time: f64);
    pub fn av_audio_unit_delay_get_feedback(node: *mut c_void) -> f32;
    pub fn av_audio_unit_delay_set_feedback(node: *mut c_void, feedback: f32);
    pub fn av_audio_unit_delay_get_low_pass_cutoff(node: *mut c_void) -> f32;
    pub fn av_audio_unit_delay_set_low_pass_cutoff(node: *mut c_void, low_pass_cutoff: f32);
    pub fn av_audio_unit_delay_get_wet_dry_mix(node: *mut c_void) -> f32;
    pub fn av_audio_unit_delay_set_wet_dry_mix(node: *mut c_void, wet_dry_mix: f32);

    pub fn av_audio_unit_distortion_create() -> *mut c_void;
    pub fn av_audio_unit_distortion_release(node: *mut c_void);
    pub fn av_audio_unit_distortion_get_pre_gain(node: *mut c_void) -> f32;
    pub fn av_audio_unit_distortion_set_pre_gain(node: *mut c_void, pre_gain: f32);
    pub fn av_audio_unit_distortion_get_wet_dry_mix(node: *mut c_void) -> f32;
    pub fn av_audio_unit_distortion_set_wet_dry_mix(node: *mut c_void, wet_dry_mix: f32);
    pub fn av_audio_unit_distortion_load_factory_preset(node: *mut c_void, preset: i32);

    pub fn av_audio_unit_sampler_create() -> *mut c_void;
    pub fn av_audio_unit_sampler_release(node: *mut c_void);
    pub fn av_audio_unit_sampler_load_instrument(
        node: *mut c_void,
        path: *const c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn av_audio_unit_sampler_load_sound_bank_instrument(
        node: *mut c_void,
        path: *const c_char,
        program: i32,
        bank_msb: i32,
        bank_lsb: i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn av_audio_unit_sampler_get_stereo_pan(node: *mut c_void) -> f32;
    pub fn av_audio_unit_sampler_set_stereo_pan(node: *mut c_void, stereo_pan: f32);
    pub fn av_audio_unit_sampler_get_overall_gain(node: *mut c_void) -> f32;
    pub fn av_audio_unit_sampler_set_overall_gain(node: *mut c_void, overall_gain: f32);
    pub fn av_audio_unit_sampler_get_global_tuning(node: *mut c_void) -> f32;
    pub fn av_audio_unit_sampler_set_global_tuning(node: *mut c_void, global_tuning: f32);

    pub fn av_audio_unit_varispeed_create() -> *mut c_void;
    pub fn av_audio_unit_varispeed_release(node: *mut c_void);
    pub fn av_audio_unit_varispeed_get_rate(node: *mut c_void) -> f32;
    pub fn av_audio_unit_varispeed_set_rate(node: *mut c_void, rate: f32);
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
