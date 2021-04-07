#[repr(usize)]
#[derive(Debug)]
pub enum TimerDelay {
    Cancel,
    Periodic,
    Relative,
}

#[repr(C)]
#[derive(Debug)]
pub struct TimeCapabilities {
    pub resolution: u32,
    pub accuracy: u32,
    pub sets_to_zero: bool,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    _pad1: u8,
    pub nanosecond: u32,
    pub timezone: i16,
    pub daylight: u8,
    _pad2: u8,
}
