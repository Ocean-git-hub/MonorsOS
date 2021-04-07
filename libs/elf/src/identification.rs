#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Class {
    Class32 = 1 ,
    Class64
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataEncoding {
    LittleEndian = 1,
    BigEndian
}

#[repr(C)]
#[derive(Debug)]
pub struct Identification {
    magic_number: [u8; 4],
    file_class: Class,
    data_encoding: DataEncoding,
    file_version: u8,
    os_abi_ident: u8,
    abi_version: u8,
    _pad: [u8; 7],
}

impl Identification {
    pub fn is_elf(&self) -> bool {
        const ELF_MAGIC_NUMBER: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
        self.magic_number == ELF_MAGIC_NUMBER
    }

    pub fn class(&self) -> Class {
        self.file_class
    }

    pub fn data_encoding(&self) -> DataEncoding {
        self.data_encoding
    }
}
