use crate::result::Result;
use crate::Event;
use core::mem;

#[repr(transparent)]
pub struct SimpleTextInputProtocol(
    uefi_core::protocols::console::text_input::SimpleTextInputProtocol);

impl SimpleTextInputProtocol {
    pub fn input_reset(&self, extended_verification: bool) -> Result {
        (self.0.input_reset)(&self.0, extended_verification).into_result(())
    }

    pub fn input_read_key(&self) -> Result<InputKey> {
        let mut input_key = mem::MaybeUninit::<InputKey>::uninit();
        unsafe {
            (self.0.input_read_key)(&self.0, &mut (*input_key.as_mut_ptr()).0)
                .into_result(input_key.assume_init())
        }
    }

    pub fn wait_for_key(&self) -> Event {
        Event(self.0.wait_for_key)
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct InputKey(uefi_core::protocols::console::text_input::InputKey);

impl InputKey {
    pub fn scan_code(&self) -> u16 {
        self.0.scan_code
    }

    pub fn unicode_char(&self) -> u16 {
        self.0.unicode_char
    }
}


