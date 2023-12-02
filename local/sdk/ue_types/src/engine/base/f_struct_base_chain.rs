use simple_endian::u32le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FStructBaseChain {
    // Size: 0x4
    pub struct_base_chain_array: *const *const FStructBaseChain,
    pub num_struct_bases_in_chain_minus_one: u32le,
    _padding: [u8; 4]
}