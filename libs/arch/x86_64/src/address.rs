pub fn is_aligned(address: u64, align: u64) -> bool {
    assert!(align.is_power_of_two());
    address & !(align - 1) == address
}

pub fn align_up(address: u64, align: u64) -> u64 {
    assert!(align.is_power_of_two());
    (address + align - 1) & !(align - 1)
}

pub fn align_down(address: u64, align: u64) -> u64 {
    assert!(align.is_power_of_two());
    address & !(align - 1)
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct PhysicalAddress(u64);

impl PhysicalAddress {
    #[inline]
    pub fn new(address: u64) -> Self {
        Self::try_new(address).expect("Could not convert physical address")
    }

    #[inline]
    pub fn try_new(address: u64) -> Result<Self, ()> {
        match address >> 52 {
            0 => Ok(Self(address)),
            _ => Err(())
        }
    }

    #[inline]
    fn new_truncate(address: u64) -> Self {
        Self(address & 0xfffffffffffff)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn as_ptr<T>(self) -> *const T {
        self.0 as *const T
    }

    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }

    pub fn is_aligned(self, align: u64) -> bool {
        is_aligned(self.0, align)
    }

    pub fn align_up(&mut self, align: u64) {
        self.0 = align_up(self.0, align);
    }

    pub fn align_down(&mut self, align: u64) {
        self.0 = align_down(self.0, align);
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    #[inline]
    pub fn new(address: u64) -> Self {
        Self::try_new(address).expect("Could not convert virtual address")
    }

    #[inline]
    pub fn try_new(address: u64) -> Result<Self, ()> {
        match address >> 47 {
            0 | 0x1ffff => Ok(Self(address)),
            1 => Ok(Self::new_sign_extension(address).unwrap()),
            _ => Err(())
        }
    }

    #[inline]
    fn new_sign_extension(address: u64) -> Result<Self, ()> {
        if address & (1 << 47) == 1 {
            Ok(Self(address | 0xffff << 48))
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn from_ptr<T>(pointer: *const T) -> Self {
        Self::new(pointer as u64)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn as_ptr<T>(self) -> *const T {
        self.0 as *const T
    }

    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }

    pub fn is_aligned(self, align: u64) -> bool {
        is_aligned(self.0, align)
    }

    pub fn align_up(&mut self, align: u64) {
        self.0 = align_up(self.0, align);
    }

    pub fn align_down(&mut self, align: u64) {
        self.0 = align_down(self.0, align);
    }

    pub fn pml4_table_index(self) -> usize {
        (self.0 >> 39 & 0x1ff) as usize
    }

    pub fn pdp_table_index(self) -> usize {
        (self.0 >> 30 & 0x1ff) as usize
    }

    pub fn pd_table_index(self) -> usize {
        (self.0 >> 21 & 0x1ff) as usize
    }

    pub fn page_table_index(self) -> usize {
        (self.0 >> 12 & 0x1ff) as usize
    }

    pub fn offset(self) -> u32 {
        unimplemented!();
    }
}
