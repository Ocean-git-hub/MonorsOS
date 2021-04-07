#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod allocator;

pub fn init() {}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}
