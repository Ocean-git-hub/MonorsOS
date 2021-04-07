use crate::Handle;
use crate::table_header::TableHeader;
use crate::boot_services::BootServices;
use crate::runtime_services::RuntimeServices;
use crate::protocols::console::text_input::SimpleTextInputProtocol;
use crate::protocols::console::text_output::SimpleTextOutputProtocol;
use crate::configuration_table::ConfigurationTable;

#[repr(C)]
pub struct SystemTable {
    pub header: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: Handle,
    pub con_in: *const SimpleTextInputProtocol,
    pub console_out_handle: Handle,
    pub con_out: *const SimpleTextOutputProtocol,
    pub standard_error_handle: Handle,
    pub std_err: *const SimpleTextOutputProtocol,
    pub runtime_service: *const RuntimeServices,
    pub boot_services: *const BootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: *const ConfigurationTable,
}
