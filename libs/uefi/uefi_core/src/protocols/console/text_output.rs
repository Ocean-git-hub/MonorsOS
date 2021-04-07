use crate::status::Status;

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset:
    extern "efiapi" fn(this: &SimpleTextOutputProtocol, extended_verification: bool) -> Status,

    pub output_string:
    extern "efiapi" fn(this: &SimpleTextOutputProtocol, string: *const u16) -> Status,

    pub test_string:
    extern "efiapi" fn(this: &SimpleTextOutputProtocol, string: *const u16) -> Status,

    pub query_mode: extern "efiapi" fn(
        this: &SimpleTextOutputProtocol,
        mode_number: usize,
        columns: &mut usize,
        rows: &mut usize,
    ) -> Status,

    pub set_mode: extern "efiapi" fn(this: &SimpleTextOutputProtocol, mode_number: usize) -> Status,

    pub set_attribute:
    extern "efiapi" fn(this: &SimpleTextOutputProtocol, attribute: usize) -> Status,

    pub clear_screen: extern "efiapi" fn(this: &SimpleTextOutputProtocol) -> Status,

    pub set_cursor_position:
    extern "efiapi" fn(this: &SimpleTextOutputProtocol, column: usize, row: usize) -> Status,

    _pad: [usize; 1],

    pub mode: *const SimpleTextOutputMode,
}

#[repr(C)]
#[derive(Debug)]
pub struct SimpleTextOutputMode {
    pub max_mode: i32,
    pub mode: i32,
    pub attribute: i32,
    pub cursor_column: i32,
    pub cursor_row: i32,
    pub cursor_visible: bool,
}
