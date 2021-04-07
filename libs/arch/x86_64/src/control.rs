use crate::address::PhysicalAddress;
use crate::instructions::{read_cr0, read_cr3, write_cr0, write_cr3};

pub struct CR0(u64);

impl CR0 {
    const WRITE_PROTECT: u64 = 0x10000;

    pub fn read() -> Self {
        Self(read_cr0())
    }

    unsafe fn write(&self) {
        write_cr0(self.0)
    }

    pub fn set_write_protect(&mut self, enable: bool) {
        if enable {
            self.0 |= CR0::WRITE_PROTECT;
        } else {
            self.0 &= !CR0::WRITE_PROTECT;
        }
        unsafe { self.write(); }
    }
}

pub struct CR3(u64);

impl CR3 {
    const PML4_MASK: u64 = 0xf_ffff_ffff_f000;

    pub fn read() -> Self {
        Self(read_cr3())
    }

    unsafe fn write(&self) {
        write_cr3(self.0);
    }

    pub fn pml4_table_address(&self) -> PhysicalAddress {
        PhysicalAddress::new(self.0 & Self::PML4_MASK)
    }

    pub unsafe fn set_pml4_table_address(&mut self, address: PhysicalAddress) {
        self.0 = (self.0 & !Self::PML4_MASK) | (address.as_u64() & Self::PML4_MASK);
        self.write();
    }
}
