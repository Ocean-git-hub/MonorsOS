use super::{Identification, FileType};

#[repr(C)]
#[derive(Debug)]
pub struct FileHeader64 {
    ident: Identification,
    file_type: FileType,
    machine: u16,
    version: u32,
    entry: u64,
    program_header_offset: u64,
    section_header_offset: u64,
    flags: u32,
    header_size: u16,
    program_header_entry_size: u16,
    program_header_entries: u16,
    section_header_entry_size: u16,
    section_header_entries: u16,
    section_name_string_table_index: u16,
}

impl FileHeader64 {
    pub fn identification(&self) -> &Identification {
        &self.ident
    }

    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    pub fn entry_point(&self) -> u64 {
        self.entry
    }

    pub fn program_header_offset(&self) -> usize {
        self.program_header_offset as usize
    }

    pub fn program_entry_size(&self) -> usize {
        self.program_header_entry_size as usize
    }

    pub fn program_entries(&self) -> usize {
        self.program_header_entries as usize
    }
}
