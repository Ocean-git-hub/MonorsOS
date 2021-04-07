use crate::address::PhysicalAddress;
use crate::paging::*;
use core::ops::{BitOr, Index, IndexMut};
use core::{fmt, slice};
use core::convert::TryInto;

pub struct PDPTEntryFlags(u64);

impl PDPTEntryFlags {
    pub const PRESENT: Self = Self(1 << 0);
    pub const WRITABLE: Self = Self(1 << 1);
    pub const USER: Self = Self(1 << 2);
    pub const PAGE_LEVEL_WRITE_THROUGH: Self = Self(1 << 3);
    pub const PAGE_LEVEL_CACHE_DISABLE: Self = Self(1 << 4);
    pub const ACCESSED: Self = Self(1 << 5);
    pub const DIRTY: Self = Self(1 << 6);
    pub const PAGE_SIZE: Self = Self(1 << 7);
    pub const GLOBAL: Self = Self(1 << 8);
    pub const PAT: Self = Self(1 << 12);
    pub const EXECUTE_DISABLE: Self = Self(1 << 63);
}

impl BitOr for PDPTEntryFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl PageEntryFlags for PDPTEntryFlags {
    fn from_bits_truncate(bits: u64) -> Self {
        Self(bits)
    }

    fn bits(&self) -> u64 {
        self.0
    }

    fn contains(&self, flags: Self) -> bool {
        self.bits() & flags.bits() == flags.bits()
    }
}

#[repr(transparent)]
pub struct PDPTEntry(u64);

impl PageEntry for PDPTEntry {
    type Flags = PDPTEntryFlags;

    fn address(&self) -> PhysicalAddress {
        if self.flags().contains(PDPTEntryFlags::PAGE_SIZE) {
            PhysicalAddress::new(self.0 & ADDRESS_MASK_1GB)
        } else {
            PhysicalAddress::new(self.0 & ADDRESS_MASK_4KB)
        }
    }

    fn set_page(&mut self, address: PhysicalAddress) {
        if self.flags().contains(PDPTEntryFlags::PAGE_SIZE) {
            assert!(address.is_aligned(PAGE_SIZE_1GB));
            self.set_bits((self.0 & !ADDRESS_MASK_1GB) | (address.as_u64() & ADDRESS_MASK_1GB));
        } else {
            assert!(address.is_aligned(PAGE_SIZE_4KB));
            self.set_bits((self.0 & !ADDRESS_MASK_4KB) | (address.as_u64() & ADDRESS_MASK_4KB));
        }
    }

    fn bits(&self) -> u64 {
        self.0
    }

    fn set_bits(&mut self, bits: u64) {
        self.0 = bits;
    }
}

impl fmt::Debug for PDPTEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

#[repr(C)]
pub struct PDPTable<'a> {
    entries: &'a mut [PDPTEntry; TABLE_ENTRIES]
}

impl<'a> PDPTable<'a> {
    pub unsafe fn from_address(address: PhysicalAddress) -> Self {
        Self {
            entries: slice::from_raw_parts_mut(
                address.as_u64() as *mut PDPTEntry,
                TABLE_ENTRIES,
            ).try_into().unwrap()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&PDPTEntry> {
        self.entries.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut PDPTEntry> {
        self.entries.iter_mut()
    }
}

impl<'a> Index<usize> for PDPTable<'a> {
    type Output = PDPTEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<'a> IndexMut<usize> for PDPTable<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}
