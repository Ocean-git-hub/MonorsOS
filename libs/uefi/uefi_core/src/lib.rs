#![no_std]
#![feature(abi_efiapi)]

use core::ffi::c_void;

pub mod guid;
pub mod status;
pub mod table_header;
pub mod configuration_table;
pub mod system_table;
pub mod boot_services;
pub mod runtime_services;
pub mod time;
pub mod memory;
pub mod protocols;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Handle(pub *mut c_void);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Event(pub *mut c_void);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MemoryMapKey(pub usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PhysicalAddress(pub u64);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VirtualAddress(pub u64);

pub type EventNotifyFunction = unsafe extern "efiapi" fn(event: Event, context: *mut c_void);
