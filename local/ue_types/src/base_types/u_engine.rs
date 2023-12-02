use simple_endian::*;
use crate::{FWorldContext, TIndirectArray, TSharedPtr, TWeakPtr, UGameInstance, UnknownType};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UEngine {
    // Size: 0xEC8
    _unknown_a: [u8; 0xD20],
    world_list: TIndirectArray<FWorldContext<'static>>,
    _unknown_b: [u8; 0x198]
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UGameEngine {
    // Size: 0xF18
    base_engine: UEngine,
    max_delta_time: u32le,
    server_flush_log_interval: u32le,
    game_instance: *const UGameInstance<'static>,
    game_viewport_window: TWeakPtr<UnknownType>,
    scene_viewport: TSharedPtr<UnknownType>,
    game_viewport_widget: TSharedPtr<UnknownType>,
    startup_movie_capture_handle: u32le,
    _padding: [u8; 4],
    last_time_logs_flushed: u64le
}

impl UGameEngine {
    pub fn game_instance(&self) -> Option<&UGameInstance> {
        unsafe { self.game_instance.as_ref::<'static>() }
    }
}