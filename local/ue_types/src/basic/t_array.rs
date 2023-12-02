use simple_endian::u32le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TArray<T> {
    // Size: 0x10
    pub data: *const T,
    pub array_num: u32le,
    pub array_max: u32le
}

impl<T> TArray<T> {
    pub fn len(&self) -> usize { self.array_num.to_native() as usize }
    pub fn max_size(&self) -> usize { self.array_max.to_native() as usize }

    pub fn from_data(data: *const T, num: u32, max: u32) -> Self {
        Self {
            data: data,
            array_num: num.into(),
            array_max: max.into()
        }
    }

    pub fn ref_at_index<'b>(&self, idx: usize) -> Option<&'b T> {
        if idx < self.array_num.to_native() as usize {
            unsafe { self.data.offset(idx as isize).as_ref::<'b>() }
        } else {
            None
        }
    }
}

impl<T: Copy> TArray<T> {
    pub fn at_index<'b>(&self, idx: usize) -> Result<&'b T, String> {
        if idx < self.array_num.to_native() as usize {
            unsafe { Ok(self.data.offset(idx as isize).as_ref::<'b>().unwrap()) }
        } else {
            Err(format!("Index out of bounds: {}", idx))
        }
    }
}