use simple_endian::*;
use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FTickFunction {
    pub vftable: *const UnknownType,
    pub tick_group: TEnumAsByte<UnknownType>,
    pub end_tick_group: TEnumAsByte<UnknownType>,
    pub actual_start_tick_group: TEnumAsByte<UnknownType>,
    pub actual_end_tick_group: TEnumAsByte<UnknownType>,
    _bf_c: u8,
    pub b_registered: u8,
    pub b_was_interval: u8,
    pub tick_state: u8,
    pub tick_visited_g_frame_counter: u32le,
    pub tick_queued_g_frame_counter: u32le,
    pub task_pointer: *const UnknownType,
    pub prerequisites: TArray<UnknownType, FDefaultAllocator>,
    pub next: *const UnknownType,
    pub relative_tick_cooldown: u32le,
    pub last_tick_time_seconds: u32le,
    pub tick_interval: u32le,
    _padding_a: [u8; 4],
    pub tick_task_level: *const UnknownType 
}