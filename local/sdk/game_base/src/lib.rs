mod game_base;
pub use game_base::*;

pub mod in_game;
pub use in_game::*;

pub mod interface;
pub use interface::*;

pub mod behavior;
pub use behavior::*;

pub mod funcs;
pub use funcs::*;

pub mod data;
pub use data::*;

pub mod statics;
pub use statics::*;

pub mod offsets;
pub mod offsets_server;
pub mod offsets_client;

static mut GAME_BASE: GameBase = GameBase::empty("static_init");

#[cfg(feature = "client-sdk")]
pub static mut OFFSETS: offsets::GameOffsets = offsets_client::get_offsets();

#[cfg(not(feature = "client-sdk"))]
pub static mut OFFSETS: offsets::GameOffsets = offsets_server::get_offsets();