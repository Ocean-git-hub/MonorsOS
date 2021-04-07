use crate::result::Result;
use crate::error::Error;
use crate::{SegmentType, ProgramHeader};
use super::{FileHeader64, Class, ProgramHeaderIter};
use core::{mem, ptr, slice};

pub struct ELF64Loader<'a> {
    buffer: &'a [u8],
    file_header: &'a FileHeader64,
}

impl<'a> ELF64Loader<'a> {
    pub fn new(buffer: &'a [u8]) -> Result<Self> {
        if buffer.len() < mem::size_of::<FileHeader64>() {
            return Err(Error::BufferSizeTooSmall);
        }

        let file_header = unsafe { &*(buffer.as_ptr() as *const FileHeader64) };
        let elf_ident = file_header.identification();
        if !(elf_ident.is_elf() && elf_ident.class() == Class::Class64) {
            return Err(Error::NotELF64);
        }

        Ok(ELF64Loader { buffer, file_header })
    }

    pub fn file_header(&self) -> &FileHeader64 {
        &self.file_header
    }

    pub unsafe fn load_programs(&self) -> Result {
        for program_header in self.program_header_iter()? {
            if program_header.segment_type() != SegmentType::LOAD {
                continue;
            }
            let memory_size = program_header.segment_memory_size() as usize;
            let file_size = program_header.segment_file_size() as usize;
            let offset_start = program_header.offset() as usize;
            let offset_end = offset_start + file_size;
            let address_start = program_header.start_address() as usize;

            slice::from_raw_parts_mut(address_start as *mut u8, file_size)
                .copy_from_slice(&self.buffer[offset_start..offset_end]);

            ptr::write_bytes(
                (address_start + file_size) as *mut u8,
                0,
                memory_size - file_size,
            );
        }
        Ok(())
    }

    pub unsafe fn load_program(&self, program_header: &ProgramHeader) {
        if program_header.segment_type() != SegmentType::LOAD {
            return
        }
        let memory_size = program_header.segment_memory_size() as usize;
        let file_size = program_header.segment_file_size() as usize;
        let offset_start = program_header.offset() as usize;
        let offset_end = offset_start + file_size;
        let address_start = program_header.start_address() as usize;

        slice::from_raw_parts_mut(address_start as *mut u8, file_size)
            .copy_from_slice(&self.buffer[offset_start..offset_end]);

        ptr::write_bytes(
            (address_start + file_size) as *mut u8,
            0,
            memory_size - file_size,
        );
    }

    pub fn program_header_iter(&self) -> Result<ProgramHeaderIter> {
        let headers = self.file_header.program_entries();
        if headers == 0 {
            return Err(Error::ProgramHeaderNotExist);
        }

        let header_entry_size = self.file_header.program_entry_size();
        let header_start_offset = self.file_header.program_header_offset();
        let header_end_offset = header_start_offset + header_entry_size * headers;

        ProgramHeaderIter::new(
            &self.buffer[header_start_offset..header_end_offset],
            headers,
            header_entry_size,
        )
    }
}
