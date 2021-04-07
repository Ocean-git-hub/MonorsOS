#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileType {
    NO,
    RELOCATABLE,
    EXECUTABLE,
    SHARED,
    CORE,
}
