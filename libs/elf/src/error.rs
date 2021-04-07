#[derive(Debug)]
pub enum Error {
    BufferSizeTooSmall,
    NotELF64,
    ProgramHeaderNotExist,
    PageSizeNotPowerOfTwo,
    PageNotAlignment
}
