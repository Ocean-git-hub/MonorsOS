use crate::PhysicalAddress;
pub use uefi_core::memory::MemoryAttribute;
pub use uefi_core::memory::AllocateType;
pub use uefi_core::memory::MemoryType;
pub use uefi_core::memory::ALLOCATE_PAGE_SIZE;

#[repr(transparent)]
#[derive(Debug)]
pub struct MemoryDescriptor(uefi_core::memory::MemoryDescriptor);

impl MemoryDescriptor {
    pub fn memory_type(&self) -> MemoryType {
        self.0.memory_type
    }

    pub fn start_address(&self) -> PhysicalAddress {
        self.0.physical_start
    }

    pub fn pages(&self) -> u64 {
        self.0.number_of_pages
    }

    pub fn attribute(&self) -> MemoryAttribute {
        self.0.attribute
    }
}

#[derive(Debug)]
pub struct MemoryMap<'a> {
    pub(crate) buffer: &'a [u8],
    pub(crate) descriptor_size: usize,
    pub(crate) num_descriptors: usize,
}

impl<'a> MemoryMap<'a> {
    pub fn iter(&self) -> MemoryMapIter {
        MemoryMapIter {
            memory_map: self,
            index: 0,
        }
    }
}

pub struct MemoryMapIter<'a> {
    memory_map: &'a MemoryMap<'a>,
    index: usize,
}

impl<'a> Iterator for MemoryMapIter<'a> {
    type Item = &'a MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.memory_map.num_descriptors > self.index {
            let desc_size = self.memory_map.descriptor_size;
            let descriptor = self.memory_map
                .buffer[(self.index * desc_size)..((self.index + 1) * desc_size)]
                .as_ptr() as *const _;
            self.index += 1;
            Some(unsafe { &*descriptor })
        } else {
            None
        }
    }
}

