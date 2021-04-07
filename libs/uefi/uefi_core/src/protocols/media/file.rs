use crate::status::Status;
use crate::guid::GUID;
use crate::time::Time;
use core::ffi::c_void;
use core::ops::BitOr;

#[repr(C)]
pub struct FileProtocol {
    pub revision: u64,
    pub open: extern "efiapi" fn(
        this: &FileProtocol,
        new_handle: &mut *const FileProtocol,
        file_name: *const u16,
        open_mode: FileOpenModes,
        attribute: FileAttributes,
    ) -> Status,

    pub close: extern "efiapi" fn(this: &FileProtocol) -> Status,

    _pad: usize,

    pub read:
    extern "efiapi" fn(this: &FileProtocol, buffer_size: &mut usize, buffer: *mut c_void) -> Status,

    _pad2: [usize; 3],

    pub get_info: extern "efiapi" fn(
        this: &FileProtocol,
        information_type: &GUID,
        buffer_size: &mut usize,
        buffer: *mut c_void,
    ) -> Status,
}

#[repr(C)]
pub struct SimpleFileSystemProtocol {
    pub revision: u64,
    pub open_volume:
    extern "efiapi" fn(this: &SimpleFileSystemProtocol, root: &mut *const FileProtocol) -> Status,
}

#[repr(C)]
#[derive(Debug)]
pub struct FileInfo {
    pub size: u64,
    pub file_size: u64,
    pub physical_size: u64,
    pub create_time: Time,
    pub last_access_time: Time,
    pub modification_time: Time,
    pub attribute: FileAttributes,
    pub file_name: [u16; 256],
}

#[repr(C)]
#[derive(Debug)]
pub struct FileSystemInfo {
    pub size: u64,
    pub read_only: bool,
    pub volume_size: u64,
    pub free_space: u64,
    pub block_size: u32,
    pub volume_label: [u16; 12],
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct FileOpenModes(pub u64);

impl FileOpenModes {
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
    pub const CREATE: Self = Self(1 << 63);
}

impl BitOr for FileOpenModes {
    type Output = FileOpenModes;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct FileAttributes(pub u64);

impl FileAttributes {
    pub const NONE: Self = Self(0);
    pub const READ_ONLY: Self = Self(1 << 0);
    pub const HIDDEN: Self = Self(1 << 1);
    pub const SYSTEM: Self = Self(1 << 2);
    pub const DIRECTORY: Self = Self(1 << 4);
    pub const ARCHIVE: Self = Self(1 << 5);
}

impl BitOr for FileAttributes {
    type Output = FileAttributes;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
