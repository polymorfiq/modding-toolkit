use crate::*;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(0x8))]
pub struct FDataTableRowHandle {
    pub data_table: *const UnknownType,
    pub row_name: FName
}