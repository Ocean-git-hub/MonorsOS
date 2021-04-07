use crate::boot_services::BootServices;
use crate::system_table::SystemTable;
use crate::memory::MemoryType;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

static mut BOOT_SERVICES: Option<&BootServices> = None;

pub fn init(system_table: &'static SystemTable) {
    unsafe { BOOT_SERVICES = Some(system_table.boot_services()); }
}

fn boot_services<'a>() -> &'a BootServices {
    unsafe { BOOT_SERVICES.unwrap() }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let memory_type = MemoryType::LoaderData;

        if align <= 8 {
            boot_services().allocate_pool(memory_type, size)
                .unwrap_or(ptr::null_mut())
        } else {

            //   +----------------size + align---------------+
            //   |                       |--> aligned        |
            //   +~~~~+---pointer size---+-----size-----+~~~~+
            //   |~~~~| allocate_pointer |     data     |~~~~|
            //   +~~~~+------------------+--------------+~~~~+

            let allocate_pointer =
                match boot_services().allocate_pool(memory_type, size + align) {
                    Ok(pointer) => pointer,
                    Err(_) => return ptr::null_mut()
                };
            let mut offset = allocate_pointer.align_offset(align);
            if offset == 0 {
                offset = align;
            }
            let pointer = allocate_pointer.add(offset);
            (pointer as *mut *mut u8).sub(1).write(allocate_pointer);
            pointer
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut ptr = ptr;
        if layout.align() > 8 {
            ptr = (ptr as *mut *mut u8).sub(1).read();
        }
        boot_services().free_pool(ptr);
    }
}
