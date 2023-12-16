use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct TArray<T, Allocator> {
    // Size: 0x10
    pub data: *const T,
    pub array_num: u32,
    pub array_max: u32,
    _phantom: PhantomData<Allocator>
}

impl<T, A> Default for TArray<T, A> {
    fn default() -> Self {
        Self{data: std::ptr::null(), array_num: 0u32, array_max: 0u32, _phantom: PhantomData}
    }
}

impl<T, Allocator> TArray<T, Allocator> {
    pub fn len(&self) -> usize { self.array_num as usize }
    pub fn max_size(&self) -> usize { self.array_max as usize }

    pub fn from_data(data: *const T, num: u32, max: u32) -> Self {
        Self {
            data: data,
            array_num: num.into(),
            array_max: max.into(),
            _phantom: PhantomData
        }
    }

    pub fn ref_at_index<'b>(&self, idx: usize) -> Option<&'b T> {
        if idx < self.array_num as usize {
            unsafe { self.data.offset(idx as isize).as_ref::<'b>() }
        } else {
            None
        }
    }
}

impl<T: Copy, Allocator> TArray<T, Allocator> {
    pub fn at_index<'b>(&self, idx: usize) -> Result<&'b T, String> {
        if idx < self.array_num as usize {
            let item_ptr = unsafe { self.data.offset(idx as isize) };
            let item = unsafe { item_ptr.as_ref::<'b>() };

            if item.is_some() {
                Ok(item.unwrap())
            } else {
                Err(format!("TArray - Could not convert pointer into address... {:p}", item_ptr))
            }
        } else {
            Err(format!("Index out of bounds: {}", idx))
        }
    }
}