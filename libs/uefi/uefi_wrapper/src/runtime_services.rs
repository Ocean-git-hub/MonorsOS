use crate::time::Time;
use crate::result::Result;
use uefi_core::status::Status;
use core::mem;
use core::ptr;

pub use uefi_core::runtime_services::ResetType;

#[repr(transparent)]
pub struct RuntimeServices(uefi_core::runtime_services::RuntimeServices);

impl RuntimeServices {
    pub fn time(&self) -> Time {
        let mut time = mem::MaybeUninit::<Time>::uninit();
        unsafe {
            (self.0.get_time)(&mut (*time.as_mut_ptr()).0, ptr::null_mut());
            time.assume_init()
        }
    }

    pub fn set_time(&self, time: &Time) -> Result {
        (self.0.set_time)(&time.0).into_result(())
    }

    pub fn reset_system(&self, reset_type: ResetType) -> ! {
        (self.0.reset_system)(reset_type, Status::Success, 0, ptr::null_mut())
    }
}

