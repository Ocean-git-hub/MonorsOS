use uefi_wrapper::protocols::media::file::*;
use uefi_wrapper::result::Result;

use crate::boot_services;

fn open_root_volume<'a>() -> Result<FileProtocol<'a>> {
    boot_services()
        .locate_protocol::<SimpleFileSystemProtocol>(None)?
        .open_volume()
}

pub fn file_info(file_name: &str) -> Result<FileInfo> {
    open_root_volume()?
        .open(file_name, FileOpenModes::READ, FileAttributes::NONE)?
        .get_info::<FileInfo>()
}

pub fn read_file_to_buffer(file_name: &str, buffer: &mut [u8]) -> Result<usize> {
    open_root_volume()?
        .open(file_name, FileOpenModes::READ, FileAttributes::NONE)?
        .read_to_buffer(buffer)
}

pub fn read_file(file_name: &str) -> Result<alloc::vec::Vec<u8>> {
    open_root_volume()?
        .open(file_name, FileOpenModes::READ, FileAttributes::NONE)?
        .read()
}
