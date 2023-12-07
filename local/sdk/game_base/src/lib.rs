#![feature(pointer_byte_offsets)]

mod game_base;
pub use game_base::*;

pub mod functionality;
pub use functionality::*;

pub mod behavior;
pub use behavior::*;

static mut GAME_BASE: GameBase = GameBase::empty("static_init");

#[cfg(feature = "server-sdk")]
const OFFSET_FUNC_GETNAMES: isize = 0xF08E80;
#[cfg(feature = "client-sdk")]
const OFFSET_FUNC_GETNAMES: isize = 0x10E94B0;

#[cfg(feature = "server-sdk")]
const OFFSET_STRUCT_GOBJECTS: isize = 0x645FEC8;
#[cfg(feature = "client-sdk")]
const OFFSET_STRUCT_GOBJECTS: isize = 0x753EC50;

#[cfg(feature = "server-sdk")]
const OFFSET_STRUCT_UWORLD_PROXY: isize = 0x0;
#[cfg(feature = "client-sdk")]
const OFFSET_STRUCT_UWORLD_PROXY: isize = 0x7678520;

#[cfg(feature = "server-sdk")]
const OFFSET_FUNC_GET_DISPLAY_NAME: isize = 0xF08E10;
#[cfg(feature = "client-sdk")]
const OFFSET_FUNC_GET_DISPLAY_NAME: isize = 0x10E9440;