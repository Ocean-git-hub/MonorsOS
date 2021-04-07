use crate::guid::GUID;

#[repr(transparent)]
#[derive(Debug)]
pub struct ConfigurationTable(uefi_core::configuration_table::ConfigurationTable);

impl ConfigurationTable {
    pub fn vendor_guid(&self) -> GUID {
        GUID(self.0.vendor_guid)
    }

    pub unsafe fn vendor_table<T>(&self) -> &T {
        &*(self.0.vendor_table as *const T)
    }
}
