use simple_endian::*;
use std::marker::PhantomData;
use widestring::{WideString, WideChar};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TIndirectArray<T> {
    // Size: 0x10
    data: TArray<*const T>,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TArray<T> {
    // Size: 0x10
    data: *const T,
    array_num: u32le,
    array_max: u32le
}

impl<T> TArray<T> {
    pub fn from_data(data: *const T, num: u32, max: u32) -> Self {
        Self {
            data: data,
            array_num: num.into(),
            array_max: max.into()
        }
    }
}

impl<T: Copy> TArray<T> {
    pub fn at_index<'b>(&self, idx: usize) -> Option<&'b T> {
        if idx < self.array_num.to_native() as usize {
            unsafe { self.data.offset(idx as isize).as_ref::<'b>() }
        } else {
            None
        }
    }
}

impl<T> TArray<T> {
    pub fn ref_at_index<'b>(&self, idx: usize) -> Option<&'b T> {
        if idx < self.array_num.to_native() as usize {
            unsafe { self.data.offset(idx as isize).as_ref::<'b>() }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize { self.array_num.to_native() as usize }
    pub fn max_size(&self) -> usize { self.array_max.to_native() as usize }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TMap<K, V> {
    // Size: 0x50
    _something: [u8; 0x50],
    _phantom_a: PhantomData<K>,
    _phantom_b: PhantomData<V>,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UnknownType {}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TEnumAsByte<T> {
    data: u8,
    _phantom_a: PhantomData<T>
}

impl<T> TEnumAsByte<T> {
    pub fn data(&self) -> u8 { self.data }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TWeakObjectPtr<T> {
    ptr: u64,
    _phantom: PhantomData<T>
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TWeakPtr<T> {
    object: *const T,
    weak_ref_count: u16le
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TSharedPtr<T> {
    object: *const T,
    shared_ref_count: u16le
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TSubclassOf<T> {
    ptr: u64,
    _phantom: PhantomData<T>
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TBaseDynamicMulticastDelegate {
    _unknown: [u8; 0x10]
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FURL {
    // Size: 0x68
    protocol: FString,
    host: FString,
    port: u32le,
    valid: u32le,
    map: FString,
    redirect_url: FString,
    op: TArray<FString>,
    portal: FString
}

impl FURL {
    pub fn protocol(&self) -> &FString { &self.protocol }
    pub fn host(&self) -> &FString { &self.host }
    pub fn port(&self) -> u32 { self.port.to_native() }
    pub fn valid(&self) -> u32 { self.valid.to_native() }
    pub fn map(&self) -> &FString { &self.map }
    pub fn redirect_url(&self) -> &FString { &self.redirect_url }
    pub fn op(&self) -> &TArray<FString> { &self.op }
    pub fn portal(&self) -> &FString { &self.portal }
    pub fn to_string(&self) -> String {
        format!("{:?}://{:?}:{:?}/{:?}?{:?}", self.protocol.len(), self.host.len(), self.port, self.map.len(), self.portal.len())
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FString {
    // Size: 0x10
    data: TArray<WideChar>
}

impl FString {
    pub fn to_string (&self) -> String { self.to_wide_string().to_string().unwrap() }
    pub fn to_wide_string(&self) -> WideString {
        let mut chars: Vec<WideChar> = vec![];
        if self.data.len() == 0 { return "".to_string().into() };

        for i in 0..self.data.len() {
            let curr = self.data.at_index(i).unwrap();
            chars.push(*curr);
        }

        WideString::from_vec(chars)
    }
    pub fn len(&self) -> usize { self.data.len() }
}

static mut F_STRING_STORAGE: Vec<OwnedFString> = vec![];
#[derive(Debug, Clone)]
#[repr(C)]
pub struct OwnedFString {
    // Size: 0x10
    str_data: WideString
}

impl OwnedFString {
    pub fn from_string(data: String) -> &'static Self {
        let wide_str = WideString::from_str(data.as_str());
        let new_str = Self{str_data: wide_str};

        unsafe {
            F_STRING_STORAGE.push(new_str);
            F_STRING_STORAGE.last().unwrap()
        }
    }

    pub fn fstring(&self) -> FString {
        let char_ptr = self.str_data.as_ptr() as *const WideChar;
        let str_len = self.str_data.len() as u32;
        let capacity = self.str_data.capacity() as u32;

        let str_array = TArray::<WideChar>::from_data(char_ptr, str_len, capacity);

        FString{data: str_array}
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FIntVector {
    _data: [u8; 0xC]
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FGuid {
    // Size: 0x10
    a: u32le,
    b: u32le,
    c: u32le,
    d: u32le
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FExec {
    _unknown: [u8; 0x8]
}