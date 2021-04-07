use crate::address::PhysicalAddress;
use crate::paging::*;
use core::ops::{BitOr, Index, IndexMut};
use core::{fmt, slice};
use core::convert::TryInto;

pub struct PML4EntryFlags(u64);

impl PML4EntryFlags {
    pub const PRESENT: Self = Self(1 << 0);
    pub const WRITABLE: Self = Self(1 << 1);
    pub const USER: Self = Self(1 << 2);
    pub const PAGE_LEVEL_WRITE_THROUGH: Self = Self(1 << 3);
    pub const PAGE_LEVEL_CACHE_DISABLE: Self = Self(1 << 4);
    pub const ACCESSED: Self = Self(1 << 5);
    pub const EXECUTE_DISABLE: Self = Self(1 << 63);
}

impl BitOr for PML4EntryFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl PageEntryFlags for PML4EntryFlags {
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
pub struct PML4Entry(u64);

impl PageEntry for PML4Entry {
    type Flags = PML4EntryFlags;

    fn address(&self) -> PhysicalAddress {
        PhysicalAddress::new(self.0 & ADDRESS_MASK_4KB)
    }

    fn set_page(&mut self, address: PhysicalAddress) {
        assert!(address.is_aligned(PAGE_SIZE_4KB));
        self.set_bits((self.0 & !ADDRESS_MASK_4KB) | (address.as_u64() & ADDRESS_MASK_4KB));
    }

    fn bits(&self) -> u64 {
        self.0
    }

    fn set_bits(&mut self, bits: u64) {
        self.0 = bits;
    }
}

impl fmt::Debug for PML4Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

pub struct PML4Table<'a> {
    entries: &'a mut [PML4Entry; TABLE_ENTRIES]
}

impl<'a> PML4Table<'a> {
    pub unsafe fn from_address(address: PhysicalAddress) -> Self {
        Self {
            entries: slice::from_raw_parts_mut(
                address.as_u64() as *mut PML4Entry,
                TABLE_ENTRIES,
            ).try_into().unwrap()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&PML4Entry> {
        self.entries.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut PML4Entry> {
        self.entries.iter_mut()
    }
}

impl<'a> Index<usize> for PML4Table<'a> {
    type Output = PML4Entry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<'a> IndexMut<usize> for PML4Table<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}
