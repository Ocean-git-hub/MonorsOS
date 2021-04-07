use crate::{PhysicalAddress, VirtualAddress};
use core::ops::BitOr;

pub const ALLOCATE_PAGE_SIZE: usize = 4096;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum MemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    MaxMemoryType,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AllocateType {
    AnyPages,
    MaxAddress,
    Address,
    MaxAllocateType,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryAttribute(pub u64);

impl MemoryAttribute {
    pub const UN_CACHEABLE: Self = Self(1 << 0);
    pub const WRITE_COMBINE: Self = Self(1 << 1);
    pub const WRITE_THROUGH: Self = Self(1 << 2);
    pub const WRITE_BACK: Self = Self(1 << 3);
    pub const UN_CACHEABLE_EXPORTED: Self = Self(1 << 4);
    pub const WRITE_PROTECTED: Self = Self(1 << 12);
    pub const READ_PROTECTED: Self = Self(1 << 13);
    pub const EXECUTE_PROTECTED: Self = Self(1 << 14);
    pub const NON_VOLATILE: Self = Self(1 << 15);
    pub const MORE_RELIABLE: Self = Self(1 << 16);
    pub const READ_ONLY: Self = Self(1 << 17);
    pub const SPECIFIC_PURPOSE: Self = Self(1 << 18);
    pub const CPU_CRYPTO: Self = Self(1 << 19);
    pub const RUNTIME: Self = Self(1 << 63);
}

impl BitOr for MemoryAttribute {
    type Output = MemoryAttribute;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryDescriptor {
    pub memory_type: MemoryType,
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: MemoryAttribute,
}
