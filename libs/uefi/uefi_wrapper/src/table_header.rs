#[repr(transparent)]
#[derive(Debug)]
pub struct TableHeader(uefi_core::table_header::TableHeader);