use crate::guid::GUID;

pub mod console;
pub mod media;

pub trait Protocol {
    fn guid() -> GUID;
}

pub trait FileInformationType: Protocol {}
