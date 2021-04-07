use crate::Event;
use crate::status::Status;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: u16,
}

#[repr(C)]
pub struct SimpleTextInputProtocol {
    pub input_reset:
    extern "efiapi" fn(this: &SimpleTextInputProtocol, extended_verification: bool) -> Status,

    pub input_read_key:
    extern "efiapi" fn(this: &SimpleTextInputProtocol, input_key: &mut InputKey) -> Status,

    pub wait_for_key: Event,
}
