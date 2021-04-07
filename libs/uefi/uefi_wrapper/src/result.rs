use uefi_core::status::Error;

pub type Result<T = ()> = core::result::Result<T, Error>;
