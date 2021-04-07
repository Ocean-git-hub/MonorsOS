#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GUID(pub u32, pub u16, pub u16, pub [u8; 8]);
