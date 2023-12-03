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
    fn correct_alignments() {
        assert_eq!(std::mem::align_of::<FVector2D>(), 0x4);
        assert_eq!(std::mem::align_of::<FQuat>(), 0x4);
        assert_eq!(std::mem::align_of::<FIntVector>(), 0x4);
        assert_eq!(std::mem::align_of::<FRotator>(), 0x4);
        assert_eq!(std::mem::align_of::<FVector>(), 0x4);
        assert_eq!(std::mem::align_of::<FBoxSphereBounds>(), 0x4);
        assert_eq!(std::mem::align_of::<FRotationConversionCache>(), 0x8);
        assert_eq!(std::mem::align_of::<FPhysicsVolumeChanged>(), 0x8);
        assert_eq!(std::mem::align_of::<FTransform>(), 0x10);
        assert_eq!(std::mem::align_of::<FNameEntry>(), 0x8);
        assert_eq!(std::mem::align_of::<TSharedPtr<UnknownType>>(), 0x8);
        assert_eq!(std::mem::align_of::<TSubclassOf<UnknownType>>(), 0x8);
        assert_eq!(std::mem::align_of::<TEnumAsByte<UnknownType>>(), 0x1);
        assert_eq!(std::mem::align_of::<TWeakPtr<UnknownType>>(), 0x8);
        assert_eq!(std::mem::align_of::<TMulticastScriptDelegate<UnknownType>>(), 0x8);
        assert_eq!(std::mem::align_of::<TBaseDynamicMulticastDelegate>(), 0x8);
        assert_eq!(std::mem::align_of::<FExec>(), 0x8);
        assert_eq!(std::mem::align_of::<FAutoCompleteNode>(), 0x8);
        assert_eq!(std::mem::align_of::<FGuid>(), 0x4);
        assert_eq!(std::mem::align_of::<FTickFunction>(), 0x8);
        assert_eq!(std::mem::align_of::<FActorTickFunction>(), 0x8);
        assert_eq!(std::mem::align_of::<AActor>(), 0x8);
        assert_eq!(std::mem::align_of::<UObject>(), 0x8);
        assert_eq!(std::mem::align_of::<UField>(), 0x8);
        assert_eq!(std::mem::align_of::<TArray<UObject>>(), 0x8);
        assert_eq!(std::mem::align_of::<FString>(), 0x8);
        assert_eq!(std::mem::align_of::<FUrl>(), 0x8);
        assert_eq!(std::mem::align_of::<UWorld>(), 0x8);
        assert_eq!(std::mem::align_of::<UGameEngine>(), 0x8);
        assert_eq!(std::mem::align_of::<UConsole>(), 0x8);
        assert_eq!(std::mem::align_of::<UActorComponent>(), 0x8);
        assert_eq!(std::mem::align_of::<USceneComponent>(), 0x10);
        assert_eq!(std::mem::align_of::<AController>(), 0x8);
        assert_eq!(std::mem::align_of::<APlayerController>(), 0x8);
        assert_eq!(std::mem::align_of::<APawn>(), 0x8);
    }
    
    #[test]
    fn correct_data_sizes() {
        assert_eq!(std::mem::size_of::<FVector2D>(), 0x8);
        assert_eq!(std::mem::size_of::<FQuat>(), 0x10);
        assert_eq!(std::mem::size_of::<FIntVector>(), 0xC);
        assert_eq!(std::mem::size_of::<FRotator>(), 0xC);
        assert_eq!(std::mem::size_of::<FVector>(), 0xC);
        assert_eq!(std::mem::size_of::<FRotationConversionCache>(), 0x20);
        assert_eq!(std::mem::size_of::<FPhysicsVolumeChanged>(), 0x10);
        assert_eq!(std::mem::size_of::<FBoxSphereBounds>(), 0x1C);
        assert_eq!(std::mem::size_of::<FTransform>(), 0x30);
        assert_eq!(std::mem::size_of::<FNameEntryHeader>(), 0xC);
        assert_eq!(std::mem::size_of::<FNameEntry>(), 0x20);
        assert_eq!(std::mem::size_of::<TSharedPtr<UnknownType>>(), 0x10);
        assert_eq!(std::mem::size_of::<TSubclassOf<UnknownType>>(), 0x8);
        assert_eq!(std::mem::size_of::<TEnumAsByte<UnknownType>>(), 0x1);
        assert_eq!(std::mem::size_of::<TWeakObjectPtr<UnknownType>>(), 0x8);
        assert_eq!(std::mem::size_of::<TWeakPtr<UnknownType>>(), 0x10);
        assert_eq!(std::mem::size_of::<TWeakPtr<FOutputDevice>>(), 0x10);
        assert_eq!(std::mem::size_of::<TMulticastScriptDelegate<UnknownType>>(), 0x10);
        assert_eq!(std::mem::size_of::<TBaseDynamicMulticastDelegate>(), 0x10);
        assert_eq!(std::mem::size_of::<FExec>(), 0x8);
        assert_eq!(std::mem::size_of::<FAutoCompleteNode>(), 0x28);
        assert_eq!(std::mem::size_of::<FGuid>(), 0x10);
        assert_eq!(std::mem::size_of::<FTickFunction>(), 0x50);
        assert_eq!(std::mem::size_of::<FActorTickFunction>(), 0x58);
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
        assert_eq!(std::mem::size_of::<UActorComponent>(), 0x100);
        assert_eq!(std::mem::size_of::<USceneComponent>(), 0x270);
        assert_eq!(std::mem::size_of::<AController>(), 0x3C8);
        assert_eq!(std::mem::size_of::<APlayerController>(), 0x698);
        assert_eq!(std::mem::size_of::<APawn>(), 0x3A8);
    }
}