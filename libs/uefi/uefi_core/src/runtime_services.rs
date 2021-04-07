use crate::table_header::TableHeader;
use crate::status::Status;
use crate::time::{Time, TimeCapabilities};
use crate::memory::MemoryDescriptor;
use core::ffi::c_void;

#[repr(C)]
pub struct RuntimeServices {
    pub header: TableHeader,
    pub get_time: extern "efiapi" fn(time: &mut Time, capabilities: *mut TimeCapabilities) -> Status,

    pub set_time: extern "efiapi" fn(time: &Time) -> Status,

    pub get_wakeup_time:
    extern "efiapi" fn(enabled: &mut bool, pending: &mut bool, time: &mut Time) -> Status,

    pub set_wakeup_time: extern "efiapi" fn(enable: bool, time: *const Time) -> Status,

    pub set_virtual_address_map: extern "efiapi" fn(
        memory_map_size: usize,
        descriptor_size: usize,
        descriptor_version: u32,
        virtual_map: *const MemoryDescriptor,
    ) -> Status,

    _pad: [usize; 5],

    pub reset_system: extern "efiapi" fn(
        reset_type: ResetType,
        reset_status: Status,
        data_size: usize,
        reset_data: *const c_void,
    ) -> !,
}

#[repr(u32)]
#[derive(Debug)]
pub enum ResetType {
    Cold,
    Warm,
    Shutdown,
    PlatformSpecific,
}
