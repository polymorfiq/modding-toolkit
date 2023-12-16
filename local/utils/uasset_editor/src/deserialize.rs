use ue_types::*;
use std::io::{Seek, SeekFrom, Read};

pub trait FileDeserialize {
    fn parse_inline<R: Seek + Read>(source: &mut R) -> Self;
}

impl FileDeserialize for FPackageFileSummary {
    fn parse_inline<R: Seek + Read>(source: &mut R) -> FPackageFileSummary {
        let mut summary = FPackageFileSummary::default();
        
        let orig_pos = source.stream_position().unwrap();
        unsafe {
            let summary_slice = std::slice::from_raw_parts_mut(&mut summary as *mut _ as *mut u8, std::mem::size_of::<FPackageFileSummary>());
            source.read_exact(summary_slice).expect("Could not read file header");
        }
        let end_pos = source.stream_position().unwrap();

        source.seek(SeekFrom::Start(orig_pos + 0x10));
        summary.custom_version_container = FCustomVersionContainer::parse_inline(source);

        source.seek(SeekFrom::Start(orig_pos + 0x28));
        summary.folder_name = FString::parse_inline(source);

        source.seek(SeekFrom::Start(orig_pos + 0x40));
        summary.localization_id = FString::parse_inline(source);

        source.seek(SeekFrom::Start(orig_pos + 0x90));
        summary.generations = TArray::parse_inline(source);

        source.seek(SeekFrom::Start(orig_pos + 0xA0));
        summary.saved_by_engine_version = FEngineVersion::parse_inline(source);

        source.seek(SeekFrom::Start(orig_pos + 0xC0));
        summary.compatible_with_engine_version = FEngineVersion::parse_inline(source);

        source.seek(SeekFrom::Start(orig_pos + 0x100));
        summary.chunk_ids = TArray::parse_inline(source);

        // summary.custom_version_container = FCustomVersionContainer::default();
        // summary.folder_name = FString::default();
        // summary.localization_id = FString::default();
        // summary.generations = TArray::default();
        // summary.chunk_ids = TArray::default();

        source.seek(SeekFrom::Start(end_pos));
        summary
    }
}

impl FileDeserialize for FCustomVersionContainer {
    fn parse_inline<R: Seek + Read>(source: &mut R) -> FCustomVersionContainer {
        let mut container = FCustomVersionContainer::default();

        container.versions = TArray::<FCustomVersion, FDefaultAllocator>::parse_inline(source);
        container
    }
}

impl FileDeserialize for u32 {
    fn parse_inline<R: Seek + Read>(source: &mut R) -> u32 {
        let mut num = u32::default();
        
        unsafe {
            let num_slice = std::slice::from_raw_parts_mut(&mut num as *mut _ as *mut u8, std::mem::size_of::<u32>());
            source.read_exact(num_slice).expect("Could not read array slice");
        }

        num
    }
}


impl FileDeserialize for FEngineVersion {
    fn parse_inline<R: Seek + Read>(source: &mut R) -> FEngineVersion {
        let mut engine_version = FEngineVersion::default();
        
        let orig_pos = source.stream_position().unwrap();
        unsafe {
            let version_slice = std::slice::from_raw_parts_mut(&mut engine_version as *mut _ as *mut u8, std::mem::size_of::<FEngineVersion>());
            source.read_exact(version_slice).expect("Could not read array slice");
        }
        let end_pos = source.stream_position().unwrap();

        source.seek(SeekFrom::Start(orig_pos + std::mem::size_of::<FEngineVersionBase>() as u64 + 0x4));
        engine_version.branch = FString::default();

        source.seek(SeekFrom::Start(end_pos));
        engine_version
    }
}

impl FileDeserialize for FString {
    fn parse_inline<R: Seek + Read>(source: &mut R) -> FString {
        let mut f_str = FString::default();
        f_str.data = TArray::parse_inline(source);
        f_str
    }
}

impl<T, A> FileDeserialize for TArray<T, A> {
    fn parse_inline<R: Seek + Read>(source: &mut R) -> TArray<T, A> {
        let mut array = TArray::<T, A>::default();
        
        unsafe {
            let array_slice = std::slice::from_raw_parts_mut(&mut array as *mut _ as *mut u8, std::mem::size_of::<TArray<T, A>>());
            source.read_exact(array_slice).expect("Could not read array slice");
        }
        let end_of_array = source.stream_position();

        let data_file_offset: u64 = array.data as u64;
        source.seek(SeekFrom::Start(data_file_offset));

        let mut data_vec: Vec<T> = Vec::with_capacity(array.array_num as usize);
        unsafe {
            let data_slice = std::slice::from_raw_parts_mut(&mut data_vec as *mut _ as *mut u8, std::mem::size_of::<T>() * data_vec.len());
            source.read_exact(data_slice).expect("Could not read array data slice");
        }
        array.data = std::ptr::addr_of!(*data_vec.leak()) as *const T;
        
        array
    }
}