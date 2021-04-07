use crate::address::PhysicalAddress;

pub mod pml4;
pub mod pdp;
pub mod page_directory;
pub mod page;

pub const TABLE_ENTRIES: usize = 512;
pub const PAGE_SIZE_1GB: u64 = 0x40000000;
pub const PAGE_SIZE_2MB: u64 = 0x200000;
pub const PAGE_SIZE_4KB: u64 = 4096;

pub const ADDRESS_MASK_1GB: u64 = 0xf_ffff_c000_0000;
pub const ADDRESS_MASK_2MB: u64 = 0xf_ffff_ffe0_0000;
pub const ADDRESS_MASK_4KB: u64 = 0xf_ffff_ffff_f000;

pub trait PageEntryFlags {
    fn from_bits_truncate(bits: u64) -> Self;

    fn bits(&self) -> u64;

    fn contains(&self, flags: Self) -> bool;
}

pub trait PageEntry {
    type Flags: PageEntryFlags;

    fn address(&self) -> PhysicalAddress;

    fn set_page(&mut self, address: PhysicalAddress);

    fn bits(&self) -> u64;

    fn set_bits(&mut self, bits: u64);

    fn flags(&self) -> Self::Flags {
        Self::Flags::from_bits_truncate(self.bits())
    }

    fn set_flags(&mut self, flags: Self::Flags) {
        self.set_bits(self.bits() | flags.bits());
    }

    fn clear_flags(&mut self, flags: Self::Flags) {
        self.set_bits(self.bits() & !flags.bits())
    }

    fn set_unused(&mut self) {
        self.set_bits(0);
    }
}
