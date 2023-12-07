use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FActorTickFunction {
    base_tick_function: FTickFunction,
    target: *const AActor
}