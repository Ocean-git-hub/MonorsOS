use crate::result::Result;
use crate::{PhysicalAddress, MemoryMapKey, Event, Handle};
use crate::memory::{AllocateType, MemoryMap};
use crate::memory::MemoryType;
use crate::time::TimerDelay;
use crate::EventNotifyFunction;
use crate::protocols::Protocol;
use uefi_core::status::Status;
use core::{mem, ptr, slice};
use core::ffi::c_void;

pub use uefi_core::boot_services::TPL;
pub use uefi_core::boot_services::EventType;

#[repr(transparent)]
pub struct BootServices(uefi_core::boot_services::BootServices);

impl BootServices {
    pub fn allocate_pages(&self, allocate_type: AllocateType, memory_type: MemoryType, pages: usize)
                          -> Result<PhysicalAddress> {
        let mut address = PhysicalAddress(0);
        (self.0.allocate_pages)(allocate_type, memory_type, pages, &mut address)
            .into_result(address)
    }

    pub fn free_pages(&self, address: PhysicalAddress, pages: usize) {
        match (self.0.free_pages)(address, pages) {
            Status::Success => {}
            status => panic!("Could not free page: {:?}", status)
        };
    }

    pub fn memory_map_size(&self) -> usize {
        let mut memory_map_size = 0;
        let mut map_key = mem::MaybeUninit::<MemoryMapKey>::uninit();
        let mut descriptor_size = 0;
        let mut descriptor_version = 0;
        let status;
        status = (self.0.get_memory_map)(
            &mut memory_map_size,
            ptr::null_mut(),
            map_key.as_mut_ptr() as *mut _,
            &mut descriptor_size,
            &mut descriptor_version,
        );
        assert_eq!(status, Status::BufferTooSmall);

        // --- from UEFI specification 2.8 242p ---
        // "If the MemoryMap buffer is too small, the EFI_BUFFER_TOO_SMALL error code is returned
        //  and the MemoryMapSize value contains the size of the buffer needed to contain the
        //  current memorymap. The actual size of the buffer allocated for the consequent call to
        //  GetMemoryMap() should be bigger then the value returned in MemoryMapSize, since
        //  allocation of the new buffer may potentially increase memory map size."
        memory_map_size + descriptor_size * 2
    }

    pub fn memory_map<'a>(&self, buffer: &'a mut [u8]) -> Result<(MemoryMap<'a>, MemoryMapKey)> {
        let mut map_key = mem::MaybeUninit::<MemoryMapKey>::uninit();
        let mut memory_map_size = buffer.len();
        let mut descriptor_size = 0;
        let mut descriptor_version = 0;

        (self.0.get_memory_map)(&mut memory_map_size,
                                buffer.as_mut_ptr(),
                                map_key.as_mut_ptr() as *mut _,
                                &mut descriptor_size,
                                &mut descriptor_version,
        ).into_result((
            MemoryMap {
                buffer,
                descriptor_size,
                num_descriptors: memory_map_size / descriptor_size,
            },
            unsafe { map_key.assume_init() }
        ))
    }

    pub fn allocate_pool(&self, pool_type: MemoryType, size: usize) -> Result<*mut u8> {
        let mut buffer = ptr::null_mut();
        (self.0.allocate_pool)(pool_type, size, &mut buffer).into_result(buffer as *mut u8)
    }

    pub fn free_pool(&self, buffer: *mut u8) {
        match (self.0.free_pool)(buffer as *mut c_void) {
            Status::Success => {}
            status => panic!("Could not free buffer from pool: {:?}", status)
        }
    }

    pub unsafe fn create_event(
        &self,
        event_type: EventType,
        notify_tpl: TPL,
        notify_function: Option<EventNotifyFunction>,
        notify_context: Option<*const c_void>,
    ) -> Result<Event> {
        let mut event = mem::MaybeUninit::<Event>::uninit();
        (self.0.create_event)(
            event_type.0,
            notify_tpl,
            mem::transmute(notify_function),
            notify_context.unwrap_or(ptr::null_mut()),
            event.as_mut_ptr() as *mut _).into_result(event.assume_init())
    }

    pub fn set_timer(&self, event: Event, delay_type: TimerDelay, trigger_time: u64) -> Result {
        (self.0.set_timer)(event.0, delay_type, trigger_time).into_result(())
    }

    pub fn wait_for_event(&self, events: &[Event]) -> Result<usize> {
        let mut index = 0;
        (self.0.wait_for_event)(events.len(), events.as_ptr() as *const _, &mut index)
            .into_result(index)
    }

    pub fn signal_event(&self, event: Event) -> Result {
        (self.0.signal_event)(event.0).into_result(())
    }

    pub fn close_event(&self, event: Event) {
        match (self.0.close_event)(event.0) {
            Status::Success => {}
            status => panic!("Could not close event: {:?}", status)
        }
    }

    pub fn exit_boot_services(&self, image_handle: Handle) -> MemoryMap {
        loop {
            let size = self.memory_map_size();
            let buffer_pointer =
                self.allocate_pool(MemoryType::ConventionalMemory, size)
                    .expect("Could not allocate pool");
            let buffer =
                unsafe { slice::from_raw_parts_mut(buffer_pointer, size) };

            match self.memory_map(buffer) {
                Ok((memory_map, map_key)) => {
                    match (self.0.exit_boot_services)(image_handle.0, map_key.0) {
                        Status::Success => return memory_map,
                        _ => {}
                    }
                }
                Err(_) => {}
            }
            self.free_pool(buffer_pointer);
        }
    }

    pub fn locate_protocol<T: Protocol>(&self, registration: Option<*const c_void>) -> Result<&T> {
        let mut protocol_pointer = ptr::null_mut();
        (self.0.locate_protocol)(
            &T::guid().0,
            registration.unwrap_or(ptr::null_mut()),
            &mut protocol_pointer,
        ).into_result(unsafe { &*(protocol_pointer as *mut T) })
    }
}
