use crate::configuration_table::ConfigurationTable;
use crate::protocols::console::text_input::SimpleTextInputProtocol;
use crate::protocols::console::text_output::SimpleTextOutputProtocol;
use crate::runtime_services::RuntimeServices;
use crate::boot_services::BootServices;
use core::slice;

#[repr(transparent)]
pub struct SystemTable(uefi_core::system_table::SystemTable);

impl SystemTable {
    #[inline]
    pub fn firmware_vendor(&self) -> *const u16 {
        self.0.firmware_vendor
    }

    #[inline]
    #[cfg(feature = "alloc")]
    pub fn firmware_vendor_string(&self) -> alloc::string::String {
        let mut len = 0;
        unsafe {
            while *(self.0.firmware_vendor.add(len)) != 0 {
                len += 1;
            }
        }
        alloc::string::String::from_utf16_lossy(
            unsafe { slice::from_raw_parts(self.0.firmware_vendor, len) }
        )
    }

    #[inline]
    pub fn con_in(&self) -> &SimpleTextInputProtocol {
        unsafe  { &*(self.0.con_in as *const SimpleTextInputProtocol) }
    }

    #[inline]
    pub fn con_out(&self) -> &mut SimpleTextOutputProtocol {
        unsafe { &mut *(self.0.con_out as *mut SimpleTextOutputProtocol) }
    }

    #[inline]
    pub fn runtime_services(&self) -> &'static RuntimeServices {
        unsafe { &*(self.0.runtime_service as *const RuntimeServices) }
    }

    #[inline]
    pub fn boot_services(&self) -> &BootServices {
        unsafe { &*(self.0.boot_services as *const BootServices) }
    }

    #[inline]
    pub fn configuration_table(&self) -> &'static [ConfigurationTable] {
        unsafe {
            slice::from_raw_parts(
                self.0.configuration_table as *const ConfigurationTable,
                self.0.number_of_table_entries)
        }
    }
}
