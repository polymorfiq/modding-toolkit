use crate::*;
use simple_endian::u32le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TNameEntryArray<'a> {
    pub chunks: &'a &'a FNameEntry<'a>,
    pub num_elems: u32le,
    pub num_chunks: u32le
}