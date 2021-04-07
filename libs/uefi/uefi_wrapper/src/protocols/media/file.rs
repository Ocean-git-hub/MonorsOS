use crate::result::Result;
use crate::guid::GUID;
use crate::protocols::{FileInformationType, Protocol};
#[allow(unused_imports)]
use uefi_core::status::{Error, Status};
use uefi_core::time::Time;
use core::{ptr, mem};
use core::ffi::c_void;

pub use uefi_core::protocols::media::file::FileAttributes;
pub use uefi_core::protocols::media::file::FileOpenModes;

#[repr(transparent)]
pub struct FileProtocol<'a>(&'a uefi_core::protocols::media::file::FileProtocol);

impl<'a> FileProtocol<'a> {
    pub fn open(
        &self,
        file_name: &str,
        open_mode: FileOpenModes,
        attribute: FileAttributes,
    ) -> Result<FileProtocol> {
        let mut file_pointer = ptr::null();
        let mut file_name_buffer;
        #[cfg(not(feature = "alloc"))]
            {
                const BUFFER_SIZE: usize = 255;
                if file_name.len() > BUFFER_SIZE {
                    return Err(Error::InvalidParameter);
                }
                file_name_buffer = [0; BUFFER_SIZE + 1];
                for (i, char) in file_name.chars().enumerate() {
                    file_name_buffer[i] = char as u16;
                }
            }
        #[cfg(feature = "alloc")]
            {
                file_name_buffer = file_name.encode_utf16().collect::<alloc::vec::Vec<u16>>();
                file_name_buffer.push(0);
            }

        (self.0.open)(
            &self.0,
            &mut file_pointer,
            file_name_buffer.as_ptr(),
            open_mode,
            attribute,
        ).into_result(FileProtocol(unsafe { &*file_pointer }))
    }

    fn close(&self) {
        match (self.0.close)(&self.0) {
            Status::Success => {}
            status => panic!("Could not close FileProtocol: {:?}", status)
        }
    }

    pub fn read_to_buffer(&self, buffer: &mut [u8]) -> Result<usize> {
        let mut read_size = buffer.len();
        (self.0.read)(&self.0, &mut read_size, buffer.as_mut_ptr() as *mut c_void)
            .into_result(read_size)
    }

    #[cfg(feature = "alloc")]
    pub fn read(&self) -> Result<alloc::vec::Vec<u8>> {
        let mut file_size = self.get_info::<FileInfo>()?.file_size() as usize;
        let mut buffer: alloc::vec::Vec<u8> = alloc::vec::Vec::with_capacity(file_size);
        unsafe { buffer.set_len(file_size); }
        (self.0.read)(&self.0, &mut file_size, buffer.as_mut_ptr() as *mut _)
            .into_result(buffer)
    }

    pub fn get_info<T: FileInformationType>(&self) -> Result<T> {
        let mut buffer_size = mem::size_of::<T>();
        let mut file_info = mem::MaybeUninit::<T>::uninit();

        (self.0.get_info)(
            &self.0,
            &T::guid().0,
            &mut buffer_size,
            file_info.as_mut_ptr() as *mut c_void,
        ).into_result(unsafe { file_info.assume_init() })
    }
}

impl<'a> Drop for FileProtocol<'a> {
    fn drop(&mut self) {
        self.close()
    }
}

#[repr(transparent)]
pub struct SimpleFileSystemProtocol(uefi_core::protocols::media::file::SimpleFileSystemProtocol);

impl SimpleFileSystemProtocol {
    pub fn open_volume(&self) -> Result<FileProtocol> {
        let mut protocol_pointer = ptr::null();

        (self.0.open_volume)(&self.0, &mut protocol_pointer)
            .into_result(FileProtocol(unsafe { &*protocol_pointer }))
    }
}

impl Protocol for SimpleFileSystemProtocol {
    fn guid() -> GUID {
        crate::guid::SIMPLE_FILE_SYSTEM_PROTOCOL
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct FileInfo(uefi_core::protocols::media::file::FileInfo);

impl FileInfo {
    pub fn file_size(&self) -> u64 {
        self.0.file_size
    }

    pub fn create_time(&self) -> &Time {
        &self.0.create_time
    }

    pub fn access_time(&self) -> &Time {
        &self.0.last_access_time
    }

    pub fn attribute(&self) -> FileAttributes {
        self.0.attribute
    }

    pub fn file_name(&self) -> *const u16 {
        self.0.file_name.as_ptr()
    }
}

impl Protocol for FileInfo {
    fn guid() -> GUID {
        crate::guid::FILE_INFO
    }
}

impl FileInformationType for FileInfo {}

#[repr(transparent)]
#[derive(Debug)]
pub struct FileSystemInfo(uefi_core::protocols::media::file::FileSystemInfo);

impl FileSystemInfo {
    pub fn volume_size(&self) -> u64 {
        self.0.volume_size
    }

    pub fn free_space(&self) -> u64 {
        self.0.free_space
    }

    pub fn block_size(&self) -> u32 {
        self.0.block_size
    }

    pub fn volume_label(&self) -> *const u16 {
        self.0.volume_label.as_ptr()
    }
}

impl Protocol for FileSystemInfo {
    fn guid() -> GUID {
        crate::guid::FILE_SYSTEM_INFO
    }
}

impl FileInformationType for FileSystemInfo {}
