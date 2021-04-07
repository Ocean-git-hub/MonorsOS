use crate::{Handle, Event, PhysicalAddress, MemoryMapKey, EventNotifyFunction};
use crate::table_header::TableHeader;
use crate::status::Status;
use crate::guid::GUID;
use crate::memory::{AllocateType, MemoryType};
use crate::time::TimerDelay;
use core::ffi::c_void;
use core::ops::BitOr;

#[repr(C)]
pub struct BootServices {
    pub header: TableHeader,
    _pad: [usize; 2],

    pub allocate_pages: extern "efiapi" fn(
        allocate_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
        memory: &mut PhysicalAddress,
    ) -> Status,

    pub free_pages: extern "efiapi" fn(memory: PhysicalAddress, pages: usize) -> Status,

    pub get_memory_map: extern "efiapi" fn(
        memory_map_size: &mut usize,
        memory_map: *mut u8,
        map_key: *mut MemoryMapKey,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> Status,

    pub allocate_pool: extern "efiapi" fn(
        pool_type: MemoryType,
        size: usize,
        buffer: &mut *mut c_void,
    ) -> Status,

    pub free_pool: extern "efiapi" fn(buffer: *mut c_void) -> Status,

    pub create_event: extern "efiapi" fn(
        event_type: u32,
        notify_tpl: TPL,
        notify_function: Option<EventNotifyFunction>,
        notify_context: *const c_void,
        event: *mut Event,
    ) -> Status,

    pub set_timer:
    extern "efiapi" fn(event: Event, delay_type: TimerDelay, trigger_time: u64) -> Status,

    pub wait_for_event: extern "efiapi" fn(
        number_of_events: usize,
        event: *const Event,
        index: &mut usize,
    ) -> Status,

    pub signal_event: extern "efiapi" fn(event: Event) -> Status,

    pub close_event: extern "efiapi" fn(event: Event) -> Status,

    pub check_event: extern "efiapi" fn(event: Event) -> Status,

    _pad2: [usize; 13],

    pub exit_boot_services:
    extern "efiapi" fn(image_handle: Handle, map_key: MemoryMapKey) -> Status,

    _pad3: [usize; 10],

    pub locate_protocol: extern "efiapi" fn(
        protocol: &GUID,
        registration: *const c_void,
        interface: &mut *mut c_void,
    ) -> Status,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TPL(pub usize);

impl TPL {
    pub const APPLICATION: Self = Self(4);
    pub const CALLBACK: Self = Self(8);
    pub const NOTIFY: Self = Self(16);
    pub const HIGH_LEVEL: Self = Self(31);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EventType(pub u32);

impl EventType {
    pub const NOTIFY_WAIT: Self = Self(0x100);
    pub const NOTIFY_SIGNAL: Self = Self(0x200);
    pub const SIGNAL_EXIT_BOOT_SERVICES: Self = Self(0x201);
    pub const RUNTIME: Self = Self(0x40000000);
    pub const SIGNAL_VIRTUAL_ADDRESS_CHANGE: Self = Self(0x60000202);
    pub const TIMER: Self = Self(0x80000000);
}

impl BitOr for EventType {
    type Output = EventType;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
