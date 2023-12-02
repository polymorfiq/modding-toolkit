#![feature(pointer_byte_offsets)]
use core::ffi::c_void;

pub mod basic;
pub use basic::*;

pub mod funcs;

pub mod engine;
pub use engine::*;

pub mod controllers;
pub use controllers::*;

pub mod interface;
pub use interface::*;

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn correct_data_sizes() {
        assert_eq!(std::mem::size_of::<FNameEntry>(), 0x18);
        assert_eq!(std::mem::size_of::<TEnumAsByte<UnknownType>>(), 0x1);
        assert_eq!(std::mem::size_of::<TWeakPtr<UnknownType>>(), 0x10);
        assert_eq!(std::mem::size_of::<TWeakPtr<FOutputDevice>>(), 0x10);
        assert_eq!(std::mem::size_of::<FExec>(), 0x8);
        assert_eq!(std::mem::size_of::<FAutoCompleteNode>(), 0x28);
        assert_eq!(std::mem::size_of::<FGuid>(), 0x10);
        assert_eq!(std::mem::size_of::<AActor>(), 0x348);
        assert_eq!(std::mem::size_of::<UObject>(), 0x30);
        assert_eq!(std::mem::size_of::<UField>(), 0x38);
        assert_eq!(std::mem::size_of::<TArray<UObject>>(), 0x10);
        assert_eq!(std::mem::size_of::<FString>(), 0x10);
        assert_eq!(std::mem::size_of::<FUrl>(), 0x68);
        assert_eq!(std::mem::size_of::<ULevel>(), 0x288);
        assert_eq!(std::mem::size_of::<UWorld>(), 0x730);
        assert_eq!(std::mem::size_of::<FSeamlessTravelHandler>(), 0xA8);
        assert_eq!(std::mem::size_of::<FWorldContext>(), 0x278);
        assert_eq!(std::mem::size_of::<UEngine>(), 0xEC8);
        assert_eq!(std::mem::size_of::<UGameEngine>(), 0xF18);
        assert_eq!(std::mem::size_of::<UConsole>(), 0x140);
    }
}