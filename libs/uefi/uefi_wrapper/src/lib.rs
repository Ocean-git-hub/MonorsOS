#![no_std]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod guid;
pub mod table_header;
pub mod configuration_table;
pub mod system_table;
pub mod boot_services;
pub mod runtime_services;
pub mod time;
pub mod memory;
pub mod protocols;
pub mod result;

#[cfg(feature = "alloc")]
pub mod allocator;

pub use uefi_core::{PhysicalAddress, VirtualAddress};

use crate::system_table::SystemTable;
use core::ffi::c_void;

static mut SYSTEM_TABLE: Option<SystemTable> = None;

pub unsafe fn init(system_table: SystemTable) {
    SYSTEM_TABLE = Some(system_table);
    #[cfg(feature = "alloc")]
        allocator::init(crate::system_table());
}

pub fn system_table() -> &'static SystemTable {
    unsafe {
        SYSTEM_TABLE.as_ref().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Handle(uefi_core::Handle);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Event(pub(crate) uefi_core::Event);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MemoryMapKey(pub(crate) uefi_core::MemoryMapKey);

pub type EventNotifyFunction = unsafe extern "efiapi" fn(event: Event, context: *mut c_void);

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {$crate::_print(format_args!($($arg)*))}
}

#[macro_export]
macro_rules! println {
    () => {$crate::print!("\n")};
    ($($arg:tt)*) => {$crate::print!("{}\n", format_args!($($arg)*))};
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    system_table().con_out().write_fmt(args)
        .unwrap();
}

#[cfg(feature = "alloc")]
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}
