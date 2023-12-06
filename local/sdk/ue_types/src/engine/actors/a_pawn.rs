use simple_endian::*;
use crate::*;

#[derive(Debug, Clone)]
#[repr(C, align(0x8))]
pub struct APawn<'a> {
    // Size: 0x3A8
    pub base_actor: AActor<'a>,
    pub base_nav_interface: INavAgentInterface,
    _bf_350: u8,
    _padding_a: [u8; 3],
    pub base_eye_height: u32le,
    pub auto_assess_player: TEnumAsByte<UnknownType>,
    pub auto_possess_ai: TEnumAsByte<UnknownType>,
    pub remote_view_pitch: u8,
    _padding_b: [u8; 5],
    pub ai_controller_class: TSubclassOf<UnknownType>,
    pub player_state: *const UnknownType,
    pub blended_replay_view_pitch: u32le,
    _padding_c: [u8; 4],
    pub last_hit_by: *const UnknownType,
    pub controller: *const AController<'a>,
    pub allowed_yaw_error: u32le,
    pub control_input_vector: FVector,
    pub last_control_input_vector: FVector,
    _padding_d: [u8; 4]
}

impl<'a> APawn<'a> {
    pub fn get_nav_agent_location(&self) -> FVector {
        let get_nav_agent_location: fn(*const APawn, *mut FVector) = unsafe {
            std::mem::transmute(0x7FF74B595650 as *const fn(*const APawn, *mut FVector))
        };

        let mut result: FVector = FVector{x: 0f32, y: 0f32, z: 0f32};
        unsafe { 
            (get_nav_agent_location)(std::ptr::addr_of!(*self).byte_offset(std::mem::size_of::<AActor>() as isize), std::ptr::addr_of_mut!(result));
        }
        
        result
    }
}