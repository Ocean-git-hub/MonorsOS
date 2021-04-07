#![no_std]

#[cfg(target_arch = "x86_64")]
pub type KernelEntryFunction = extern "sysv64" fn(BootInfo);

use uefi_wrapper::protocols::console::text_output::SimpleTextOutputProtocol;

#[repr(C)]
pub struct BootInfo {
    // pub memory_map: MemoryMap<'a>,
    // pub runtime_services: &'static RuntimeServices,
    // pub configuration_table: &'static [ConfigurationTable],

    pub con_out: &'static mut SimpleTextOutputProtocol,
}
