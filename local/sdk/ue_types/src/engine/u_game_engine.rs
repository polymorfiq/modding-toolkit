use crate::*;
use simple_endian::{u32le, u64le};

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct UGameEngine {
    // Size: 0xF18
    pub base_engine: UEngine,
    pub max_delta_time: u32le,
    pub server_flush_log_interval: u32le,
    pub game_instance: *const UGameInstance,
    pub game_viewport_window: TWeakPtr<UnknownType>,
    pub scene_viewport: TSharedPtr<UnknownType>,
    pub game_viewport_widget: TSharedPtr<UnknownType>,
    pub startup_movie_capture_handle: u32le,
    _padding: [u8; 4],
    pub last_time_logs_flushed: u64le
}