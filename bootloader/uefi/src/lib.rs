#![no_std]
#![feature(abi_efiapi)]
#![allow(dead_code)]

extern crate alloc;

use core::mem;

use elf;
use elf::loader::ELF64Loader;
use uefi_wrapper::boot_services::BootServices;
use uefi_wrapper::memory::{AllocateType, MemoryType};
use uefi_wrapper::println;
use uefi_wrapper::protocols::console::text_input::SimpleTextInputProtocol;
use uefi_wrapper::protocols::console::text_output::SimpleTextOutputProtocol;
use uefi_wrapper::runtime_services::{ResetType, RuntimeServices};
use uefi_wrapper::system_table::SystemTable;

use crate::arch::paging::{map_page, PAGE_SIZE};
use crate::protocol::file::read_file;

pub mod boot_menu;
mod arch;
mod protocol;

const KERNEL_PATH: &str = "\\boot\\kernel";

static mut BOOT_SERVICES: Option<&BootServices> = None;
static mut RUNTIME_SERVICES: Option<&'static RuntimeServices> = None;
static mut CON_OUT: Option<&SimpleTextOutputProtocol> = None;
static mut CON_IN: Option<&SimpleTextInputProtocol> = None;

pub unsafe fn init(system_table: SystemTable) {
    uefi_wrapper::init(system_table);

    BOOT_SERVICES = Some(uefi_wrapper::system_table().boot_services());
    RUNTIME_SERVICES = Some(uefi_wrapper::system_table().runtime_services());
    CON_OUT = Some(uefi_wrapper::system_table().con_out());
    CON_IN = Some(uefi_wrapper::system_table().con_in());

    con_out().clear_screen().unwrap();
}

pub fn boot_services<'a>() -> &'a BootServices {
    unsafe {
        BOOT_SERVICES.unwrap()
    }
}

pub fn runtime_services() -> &'static RuntimeServices {
    unsafe {
        RUNTIME_SERVICES.unwrap()
    }
}

pub fn con_out<'a>() -> &'a SimpleTextOutputProtocol {
    unsafe {
        CON_OUT.unwrap()
    }
}

pub fn con_in<'a>() -> &'a SimpleTextInputProtocol {
    unsafe {
        CON_IN.unwrap()
    }
}

fn max_largest_screen_mode() -> usize {
    let mut max_area = 0;
    let mut mode = 0;
    for i in 0..con_out().mode().max_mode() as usize {
        let screen = con_out().query_mode(i)
            .unwrap_or_default();
        let area = screen.column() * screen.row();
        if max_area < area {
            max_area = area;
            mode = i;
        }
    }
    mode
}

pub fn set_max_largest_screen_mode() {
    con_out().set_mode(max_largest_screen_mode()).unwrap();
}


pub fn boot_kernel() {
    con_out().clear_screen().unwrap();
    println!("Booting kernel...");

    let kernel_entry_point;
    {
        let kernel_file = read_file(KERNEL_PATH)
            .expect("Could not read file");
        println!("Kernel file size: {}B", kernel_file.len());

        let kernel_loader = ELF64Loader::new(kernel_file.as_slice())
            .expect("Could not create instance of ELF64Loader");

        assert_eq!(kernel_loader.file_header().file_type(), elf::FileType::EXECUTABLE,
                   "Kernel file is not executable");

        for program_header in kernel_loader.program_header_iter()
            .expect("Could not get program_header")
            .into_iter() {
            if program_header.segment_type() != elf::SegmentType::LOAD {
                continue;
            }

            let pages = program_header.segment_pages(PAGE_SIZE as u64) as usize;
            let allocate_page =
                boot_services().allocate_pages(
                    AllocateType::AnyPages,
                    MemoryType::LoaderData,
                    pages,
                ).expect("Could not allocate pages");

            unsafe {
                for i in 0..pages {
                    map_page(
                        program_header.start_address() as usize + i * PAGE_SIZE,
                        allocate_page.0 as usize + i * PAGE_SIZE,
                    );
                }
            }
        }

        unsafe { kernel_loader.load_programs().expect("Could not load kernel"); }
        kernel_entry_point = kernel_loader.file_header().entry_point();
    }
    println!("Kernel entry point: {:#x}", kernel_entry_point);
    let kernel_entry_point: boot_protocol::KernelEntryFunction =
        unsafe { mem::transmute(kernel_entry_point) };

    kernel_entry_point(boot_protocol::BootInfo {
        con_out: uefi_wrapper::system_table().con_out()
    });
}


pub fn shutdown() {
    runtime_services().reset_system(ResetType::Shutdown)
}

pub fn wait_for_any_key() {
    while let Ok(_) = con_in().input_read_key() {};
    boot_services().wait_for_event(&[con_in().wait_for_key()])
        .unwrap();
}
