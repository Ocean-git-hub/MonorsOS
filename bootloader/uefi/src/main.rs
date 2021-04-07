#![no_std]
#![no_main]
#![feature(abi_efiapi)]

extern crate alloc;

use alloc::vec;

use uefi::*;
use uefi::boot_menu::*;
use uefi_wrapper::println;
use uefi_wrapper::system_table::SystemTable;

#[no_mangle]
unsafe extern "efiapi" fn efi_main(_image_handle: usize, system_table: SystemTable) {
    uefi::init(system_table);
    println!("Welcome to MonorsOS UEFI-Bootloader v{}", env!("CARGO_PKG_VERSION"));

    {
        let mut boot_menu = BootMenu::new(
            1,
            vec![
                BootMenuOption::new("Boot MonorsOS", || ()),
                BootMenuOption::new("Shutdown", shutdown)
            ]);
        boot_menu.menu_loop();
    }

    boot_kernel();
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

