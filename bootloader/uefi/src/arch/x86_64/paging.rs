use uefi_wrapper::memory::{ALLOCATE_PAGE_SIZE, AllocateType, MemoryType};
use x86_64::address::*;
use x86_64::control::*;
use x86_64::paging::*;
use x86_64::paging::page::*;
use x86_64::paging::page_directory::*;
use x86_64::paging::pdp::*;
use x86_64::paging::pml4::*;

pub const PAGE_SIZE: usize = ALLOCATE_PAGE_SIZE;

pub unsafe fn map_page(virtual_start: usize, page_address: usize) {
    let virtual_address = VirtualAddress::new(virtual_start as u64);
    let page_address = PhysicalAddress::new(page_address as u64);
    assert!(page_address.is_aligned(PAGE_SIZE_4KB));

    CR0::read().set_write_protect(false);

    map_page_inner(virtual_address, page_address);

    CR0::read().set_write_protect(true);
}

fn allocate_page() -> u64 {
    crate::boot_services().allocate_pages(
        AllocateType::AnyPages, MemoryType::LoaderData, 1)
        .expect("Could not allocate page").0
}

unsafe fn map_page_inner(virtual_address: VirtualAddress, page_address: PhysicalAddress) {
    let mut table = PML4Table::from_address(CR3::read().pml4_table_address());
    let entry = &mut table[virtual_address.pml4_table_index()];
    if !entry.flags().contains(PML4EntryFlags::PRESENT) {
        let allocate_page = PhysicalAddress::new(allocate_page());
        core::ptr::write_bytes(
            allocate_page.as_mut_ptr::<u8>(),
            0,
            ALLOCATE_PAGE_SIZE,
        );
        entry.set_unused();
        entry.set_page(allocate_page);
        entry.set_flags(PML4EntryFlags::PRESENT | PML4EntryFlags::WRITABLE);
    }

    let mut table = PDPTable::from_address(entry.address());
    let entry = &mut table[virtual_address.pdp_table_index()];
    if entry.flags().contains(PDPTEntryFlags::PAGE_SIZE) {
        panic!("Virtual address was already exist: {:#x}", virtual_address.as_u64());
    }
    if !entry.flags().contains(PDPTEntryFlags::PRESENT) {
        let allocate_page = PhysicalAddress::new(allocate_page());
        core::ptr::write_bytes(
            allocate_page.as_mut_ptr::<u8>(),
            0,
            ALLOCATE_PAGE_SIZE,
        );
        entry.set_unused();
        entry.set_page(allocate_page);
        entry.set_flags(PDPTEntryFlags::PRESENT | PDPTEntryFlags::WRITABLE);
    }

    let mut table = PageDirectory::from_address(entry.address());
    let entry: &mut PDEntry = &mut table[virtual_address.pd_table_index()];
    if entry.flags().contains(PDEntryFlags::PAGE_SIZE) {
        panic!("Virtual address was already exist: {:#x}", virtual_address.as_u64());
    }
    if !entry.flags().contains(PDEntryFlags::PRESENT) {
        let allocate_page = PhysicalAddress::new(allocate_page());
        core::ptr::write_bytes(
            allocate_page.as_mut_ptr::<u8>(),
            0,
            ALLOCATE_PAGE_SIZE,
        );
        entry.set_unused();
        entry.set_page(allocate_page);
        entry.set_flags(PDEntryFlags::PRESENT | PDEntryFlags::WRITABLE);
    }

    let mut table = PageTable::from_address(entry.address());
    let entry = &mut table[virtual_address.page_table_index()];
    if entry.flags().contains(PTEntryFlags::PRESENT) {
        panic!("Virtual address was already exist: {:#x}", virtual_address.as_u64());
    } else {
        entry.set_unused();
        entry.set_page(page_address);
        entry.set_flags(PTEntryFlags::PRESENT | PTEntryFlags::WRITABLE);
    }
}
