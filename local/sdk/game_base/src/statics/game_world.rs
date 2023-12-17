use crate::{GObjects, WorldProxy};
use ue_types::*;

pub struct GameWorld {
    addr: *const UWorld
}

impl GameWorld {
    pub fn world() -> Option<GameWorld> { Some(Self{addr: std::ptr::addr_of!(*WorldProxy::world()?)}) }
    pub fn base(&self) -> Option<&'static UWorld> { unsafe { self.addr.as_ref::<'static>() } }

    pub fn player_controllers(&self) -> Vec<&'static APlayerController> {
        let world = self.base().expect("Could not unwrap world");

        let mut player_controllers: Vec<&APlayerController> = vec![];
        for i in 0..world.player_controller_list.len() {
            let player_controller_ptr = world.player_controller_list.at_index(i);
            if !player_controller_ptr.is_ok() { continue; }

            let player_controller_idx = player_controller_ptr.unwrap().ptr.obj_index;

            let player_controller = GObjects::objects::<APlayerController>().item_at_idx(player_controller_idx.to_native() as usize);
            if !player_controller.is_some() { continue; }

            let player_controller = unsafe { ((*player_controller.unwrap()).object_addr as *const APlayerController).as_ref() };
            if player_controller.is_some() {
                player_controllers.push(player_controller.unwrap());
            }
        }
        
        player_controllers
    }
}