use crate::result::Result;
use crate::error::Error;
use core::ops::BitOr;
use core::fmt;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SegmentType(u32);

impl SegmentType {
    pub const NULL: Self = Self(0);
    pub const LOAD: Self = Self(1);
    pub const DYNAMIC: Self = Self(2);
    pub const INTERPRETER: Self = Self(3);
    pub const NOTE: Self = Self(4);
    pub const RESERVED: Self = Self(5);
    pub const PROGRAM_HEADER_TABLE: Self = Self(6);
    pub const PT_GNU_EH_FRAME: Self = Self(0x6474e550);
    pub const PT_GNU_STACK: Self = Self(0x6474e551);
    pub const PT_GNU_RELRO: Self = Self(0x6474e552);
}

impl fmt::Debug for SegmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NULL => write!(f, "Null"),
            Self::LOAD => write!(f, "Load"),
            Self::DYNAMIC => write!(f, "Dynamic"),
            Self::INTERPRETER => write!(f, "Interpreter"),
            Self::NOTE => write!(f, "Note"),
            Self::RESERVED => write!(f, "Reserved"),
            Self::PROGRAM_HEADER_TABLE => write!(f, "ProgramHeaderTable"),
            Self::PT_GNU_EH_FRAME => write!(f, "PT_GNU_EH_FRAME"),
            Self::PT_GNU_STACK => write!(f, "PT_GNU_STACK"),
            Self::PT_GNU_RELRO => write!(f, "PT_GNU_RELRO"),
            _ => write!(f, "{:#x}(Unknown)", self.0)
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ProgramHeaderFlags(u32);

impl ProgramHeaderFlags {
    pub const EXECUTABLE: Self = Self(1 << 0);
    pub const WRITABLE: Self = Self(1 << 1);
    pub const READABLE: Self = Self(1 << 2);
}

impl BitOr for ProgramHeaderFlags {
    type Output = ProgramHeaderFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ProgramHeader {
    segment_type: SegmentType,
    flags: ProgramHeaderFlags,
    offset: u64,
    virtual_address: u64,
    physical_address: u64,
    segment_file_size: u64,
    segment_memory_size: u64,
    segment_alignment: u64,
}

impl ProgramHeader {
    pub fn segment_type(&self) -> SegmentType {
        self.segment_type
    }

    pub fn flags(&self) -> ProgramHeaderFlags {
        self.flags
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn start_address(&self) -> u64 {
        self.virtual_address
    }

    pub fn segment_file_size(&self) -> u64 {
        self.segment_file_size
    }

    pub fn segment_memory_size(&self) -> u64 {
        self.segment_memory_size
    }

    pub fn segment_pages(&self, page_size: u64) -> u64 {
        (self.segment_memory_size() + page_size - 1) / page_size
    }
}

pub struct ProgramHeaderIter<'a> {
    buffer: &'a [u8],
    entries: usize,
    entry_size: usize,
    index: usize,
}

impl<'a> ProgramHeaderIter<'a> {
    pub fn new(buffer: &'a [u8], entries: usize, entry_size: usize) -> Result<Self> {
        if buffer.len() >= entries * entry_size {
            Ok(Self {
                buffer,
                entries,
                entry_size,
                index: 0,
            })
        } else {
            Err(Error::BufferSizeTooSmall)
        }
    }
}

impl<'a> Iterator for ProgramHeaderIter<'a> {
    type Item = &'a ProgramHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.entries > self.index && self.buffer.len() > self.entry_size * self.index {
            let buffer_start = self.entry_size * self.index;
            self.index += 1;
            let buffer_end = self.entry_size * self.index;
            let program_header = unsafe {
                ((&self.buffer[buffer_start..buffer_end]).as_ptr() as *const ProgramHeader)
                    .as_ref()
                    .unwrap()
            };
            Some(program_header)
        } else {
            None
        }
    }
}
