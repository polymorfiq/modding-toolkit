#![feature(pointer_byte_offsets)]
use core::ffi::c_void;

pub mod basic;
pub use basic::*;

pub mod engine;
pub use engine::*;

pub mod controllers;
pub use controllers::*;

pub mod interface;
pub use interface::*;

pub mod ue_widestring { pub use widestring::*; }
pub mod ue_endian { pub use simple_endian::*; }

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn correct_alignments() {
        assert_eq!(std::mem::align_of::<FHeapAllocatorForAnyElementType>(), 0x8);
        assert_eq!(std::mem::align_of::<TInlineAllocator<5, u32, 0x8, FHeapAllocatorForAnyElementType>>(), 0x8);
        assert_eq!(std::mem::align_of::<TInlineAllocator<5, u32, 0x10, FHeapAllocatorForAnyElementType>>(), 0x10);
        assert_eq!(std::mem::align_of::<FRelativeBitReference>(), 0x4);
        assert_eq!(std::mem::align_of::<TConstSetBitIterator<FDefaultBitArrayAllocator>>(), 0x8);
        assert_eq!(std::mem::align_of::<TBitArray<FDefaultBitArrayAllocator>>(), 0x8);
        assert_eq!(std::mem::align_of::<FDataTableRowHandle>(), 0x8);
        assert_eq!(std::mem::align_of::<FEngineVersionBase>(), 0x4);
        assert_eq!(std::mem::align_of::<FEngineVersion>(), 0x8);
        assert_eq!(std::mem::align_of::<FCustomVersionContainer>(), 0x8);
        assert_eq!(std::mem::align_of::<FVector2D>(), 0x4);
        assert_eq!(std::mem::align_of::<FQuat>(), 0x4);
        assert_eq!(std::mem::align_of::<FIntVector>(), 0x4);
        assert_eq!(std::mem::align_of::<FRotator>(), 0x4);
        assert_eq!(std::mem::align_of::<FVector>(), 0x4);
        assert_eq!(std::mem::align_of::<FColor>(), 0x4);
        assert_eq!(std::mem::align_of::<FBox>(), 0x4);
        assert_eq!(std::mem::align_of::<FDelegateBase>(), 0x8);
        assert_eq!(std::mem::align_of::<FGCObject>(), 0x8);
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
        assert_eq!(std::mem::align_of::<FSubsystemCollection<UnknownType>>(), 0x8);
        assert_eq!(std::mem::align_of::<FExec>(), 0x8);
        assert_eq!(std::mem::align_of::<FPrimaryAssetType>(), 0x4);
        assert_eq!(std::mem::align_of::<FPrimaryAssetId>(), 0x4);
        assert_eq!(std::mem::align_of::<FAutoCompleteNode>(), 0x8);
        assert_eq!(std::mem::align_of::<FGuid>(), 0x4);
        assert_eq!(std::mem::align_of::<FTickFunction>(), 0x8);
        assert_eq!(std::mem::align_of::<FActorTickFunction>(), 0x8);
        assert_eq!(std::mem::align_of::<AActor>(), 0x8);
        assert_eq!(std::mem::align_of::<AInfo>(), 0x8);
        assert_eq!(std::mem::align_of::<UObject<*const UnknownType>>(), 0x8);
        assert_eq!(std::mem::align_of::<UField>(), 0x8);
        assert_eq!(std::mem::align_of::<TArray<UObject<*const UnknownType>, FDefaultAllocator>>(), 0x8);
        assert_eq!(std::mem::align_of::<TSparseArrayBaseIterator<UnknownType, TAlignedBytes1<0>, FDefaultAllocator, FDefaultBitArrayAllocator>>(), 0x8);
        assert_eq!(std::mem::align_of::<TSparseArray<UnknownType, TAlignedBytes1<0>, FDefaultAllocator, FDefaultBitArrayAllocator>>(), 0x8);
        assert_eq!(std::mem::align_of::<TMulticastDelegate<UnknownType, UnknownType>>(), 0x8);
        assert_eq!(std::mem::align_of::<FString>(), 0x8);
        assert_eq!(std::mem::align_of::<FText>(), 0x8);
        assert_eq!(std::mem::align_of::<FSoftObjectPath>(), 0x8);
        assert_eq!(std::mem::align_of::<FCustomVersion>(), 0x4);
        assert_eq!(std::mem::align_of::<FAssetData>(), 0x8);
        assert_eq!(std::mem::align_of::<FObjectResource>(), 0x8);
        assert_eq!(std::mem::align_of::<FObjectExport>(), 0x8);
        assert_eq!(std::mem::align_of::<FObjectImport>(), 0x8);
        assert_eq!(std::mem::align_of::<FPackageFileSummary>(), 0x8);
        assert_eq!(std::mem::align_of::<FUrl>(), 0x8);
        assert_eq!(std::mem::align_of::<FWorldPSCPool>(), 0x8);
        assert_eq!(std::mem::align_of::<UWorld>(), 0x8);
        assert_eq!(std::mem::align_of::<UEngine>(), 0x8);
        assert_eq!(std::mem::align_of::<UGameEngine>(), 0x8);
        assert_eq!(std::mem::align_of::<UConsole>(), 0x8);
        assert_eq!(std::mem::align_of::<UActorComponent>(), 0x8);
        assert_eq!(std::mem::align_of::<USceneComponent>(), 0x10);
        assert_eq!(std::mem::align_of::<AController>(), 0x8);
        assert_eq!(std::mem::align_of::<APlayerController>(), 0x8);
        assert_eq!(std::mem::align_of::<APawn>(), 0x8);
        assert_eq!(std::mem::align_of::<AGameModeBase>(), 0x8);
        assert_eq!(std::mem::align_of::<AGameMode>(), 0x8);
        assert_eq!(std::mem::align_of::<APlayerState>(), 0x8);
    }
    
    #[test]
    fn correct_data_sizes() {
        assert_eq!(std::mem::size_of::<FHeapAllocatorForAnyElementType>(), 0x8);
        assert_eq!(std::mem::size_of::<TInlineAllocator<5, *const UnknownType, 0x8, FHeapAllocatorForAnyElementType>>(), 0x30);
        assert_eq!(std::mem::size_of::<FRelativeBitReference>(), 0x8);
        assert_eq!(std::mem::size_of::<TConstSetBitIterator<FDefaultBitArrayAllocator>>(), 0x20);
        assert_eq!(std::mem::size_of::<TBitArray<FDefaultBitArrayAllocator>>(), 0x20);
        assert_eq!(std::mem::size_of::<FEngineVersionBase>(), 0xC);
        assert_eq!(std::mem::size_of::<FEngineVersion>(), 0x20);
        assert_eq!(std::mem::size_of::<FCustomVersionContainer>(), 0x10);
        assert_eq!(std::mem::size_of::<FVector2D>(), 0x8);
        assert_eq!(std::mem::size_of::<FQuat>(), 0x10);
        assert_eq!(std::mem::size_of::<FIntVector>(), 0xC);
        assert_eq!(std::mem::size_of::<FRotator>(), 0xC);
        assert_eq!(std::mem::size_of::<FVector>(), 0xC);
        assert_eq!(std::mem::size_of::<FColor>(), 0x4);
        assert_eq!(std::mem::size_of::<FDelegateBase>(), 0x10);
        assert_eq!(std::mem::size_of::<FBox>(), 0x1C);
        assert_eq!(std::mem::size_of::<FGCObject>(), 0x10);
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
        assert_eq!(std::mem::size_of::<FSubsystemCollection<UnknownType>>(), 0xC8);
        assert_eq!(std::mem::size_of::<FExec>(), 0x8);
        assert_eq!(std::mem::size_of::<FPrimaryAssetType>(), 0xC);
        assert_eq!(std::mem::size_of::<FPrimaryAssetId>(), 0x18);
        assert_eq!(std::mem::size_of::<FAutoCompleteNode>(), 0x28);
        assert_eq!(std::mem::size_of::<FGuid>(), 0x10);
        assert_eq!(std::mem::size_of::<FTickFunction>(), 0x50);
        assert_eq!(std::mem::size_of::<FActorTickFunction>(), 0x58);
        assert_eq!(std::mem::size_of::<AActor>(), 0x348);
        assert_eq!(std::mem::size_of::<AInfo>(), 0x348);
        assert_eq!(std::mem::size_of::<UObject<*const UnknownType>>(), 0x30);
        assert_eq!(std::mem::size_of::<UField>(), 0x38);
        assert_eq!(std::mem::size_of::<TArray<UObject<*const UnknownType>, FDefaultAllocator>>(), 0x10);
        assert_eq!(std::mem::size_of::<TSparseArrayBaseIterator<UnknownType, TAlignedBytes1<0>, FDefaultAllocator, FDefaultBitArrayAllocator>>(), 0x28);
        assert_eq!(std::mem::size_of::<TSparseArray<UnknownType, TAlignedBytes1<0>, FDefaultAllocator, FDefaultBitArrayAllocator>>(), 0x38);
        assert_eq!(std::mem::size_of::<TMulticastDelegate<UnknownType, UnknownType>>(), 0x18);
        assert_eq!(std::mem::size_of::<FString>(), 0x10);
        assert_eq!(std::mem::size_of::<FText>(), 0x18);
        assert_eq!(std::mem::size_of::<FSoftObjectPath>(), 0x20);
        assert_eq!(std::mem::size_of::<FCustomVersion>(), 0x24);
        assert_eq!(std::mem::size_of::<FAssetData>(), 0x68);
        assert_eq!(std::mem::size_of::<FObjectResource>(), 0x10);
        assert_eq!(std::mem::size_of::<FObjectExport>(), 0x88);
        assert_eq!(std::mem::size_of::<FObjectImport>(), 0x40);
        assert_eq!(std::mem::size_of::<FPackageFileSummary>(), 0x118);
        assert_eq!(std::mem::size_of::<FUrl>(), 0x68);
        assert_eq!(std::mem::size_of::<ULevel>(), 0x2B0);
        assert_eq!(std::mem::size_of::<FWorldPSCPool>(), 0x58);
        assert_eq!(std::mem::size_of::<UWorld>(), 0x730);
        assert_eq!(std::mem::size_of::<FSeamlessTravelHandler>(), 0xA8);
        assert_eq!(std::mem::size_of::<FWorldContext>(), 0x278);
        assert_eq!(std::mem::size_of::<UEngine>(), 0xEC8);
        assert_eq!(std::mem::size_of::<UConsole>(), 0x140);
        assert_eq!(std::mem::size_of::<UActorComponent>(), 0x100);
        assert_eq!(std::mem::size_of::<USceneComponent>(), 0x270);
        assert_eq!(std::mem::size_of::<APawn>(), 0x3A8);
        assert_eq!(std::mem::size_of::<AController>(), 0x3C8);
        assert_eq!(std::mem::size_of::<*const UGameInstance>(), 0x8);
        assert_eq!(std::mem::size_of::<UGameEngine>(), 0xF18);
        assert_eq!(std::mem::size_of::<APlayerController>(), 0x698);
        assert_eq!(std::mem::size_of::<AGameModeBase>(), 0x3E8);
        assert_eq!(std::mem::size_of::<AGameMode>(), 0x438);
    }
}