#![feature(pointer_byte_offsets)]

mod game_base;
pub use game_base::*;

pub mod in_game;
pub use in_game::*;

pub mod interface;
pub use interface::*;

pub mod behavior;
pub use behavior::*;

pub mod offsets_client;
pub mod offsets_server;

pub mod funcs;
pub use funcs::*;

pub mod data;
pub use data::*;

pub mod statics;
pub use statics::*;

#[cfg(feature = "server-sdk")]
pub mod offsets { pub use crate::offsets_server::*; }

#[cfg(feature = "client-sdk")]
pub mod offsets { pub use crate::offsets_client::*; }

static mut GAME_BASE: GameBase = GameBase::empty("static_init");