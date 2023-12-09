#![feature(pointer_byte_offsets)]

mod game_base;
pub use game_base::*;

pub mod virtual_funcs;
pub use virtual_funcs::*;

pub mod behavior;
pub use behavior::*;

pub mod offsets_client;
pub mod offsets_server;

pub mod funcs;
pub use funcs::*;

#[cfg(feature = "server-sdk")]
mod offsets { pub use crate::offsets_server::*; }

#[cfg(feature = "client-sdk")]
mod offsets { pub use crate::offsets_client::*; }

static mut GAME_BASE: GameBase = GameBase::empty("static_init");