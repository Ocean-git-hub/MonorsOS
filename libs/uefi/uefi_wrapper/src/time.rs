use core::fmt;

pub use uefi_core::time::TimerDelay;

#[repr(transparent)]
#[derive(Debug)]
pub struct TimeCapabilities(uefi_core::time::TimeCapabilities);

#[repr(transparent)]
#[derive(Debug)]
pub struct Time(pub(crate) uefi_core::time::Time);

impl Time {
    pub fn year(&self) -> u16 {
        self.0.year
    }

    pub fn month(&self) -> u8 {
        self.0.month
    }

    pub fn day(&self) -> u8 {
        self.0.day
    }

    pub fn hour(&self) -> u8 {
        self.0.hour
    }

    pub fn minute(&self) -> u8 {
        self.0.minute
    }

    pub fn second(&self) -> u8 {
        self.0.second
    }

    pub fn nanosecond(&self) -> u32 {
        self.0.nanosecond
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = &self.0;
        write!(f, "{}/{:02}/{:02} {:02}:{:02}:{:02}",
               inner.year, inner.month, inner.day, inner.hour, inner.minute, inner.second)
    }
}
