use core::ffi::c_void;
use crate::guid::GUID;

#[repr(C)]
#[derive(Debug)]
pub struct ConfigurationTable {
    pub vendor_guid: GUID,
    pub vendor_table: *const c_void,
}
