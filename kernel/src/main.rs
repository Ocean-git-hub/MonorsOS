#![no_std]
#![no_main]

use core::fmt::Write;

#[no_mangle]
pub extern "sysv64" fn _start(boot_info: boot_protocol::BootInfo) {
    kernel::init();
    boot_info.con_out.write_str("Hello, kernel").unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
