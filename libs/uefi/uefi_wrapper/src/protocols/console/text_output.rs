use crate::result::Result;
use core::fmt;

pub struct SimpleTextOutputProtocol(uefi_core::protocols::console::text_output::SimpleTextOutputProtocol);

impl SimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> Result {
        (self.0.reset)(&self.0, extended_verification).into_result(())
    }

    pub fn output_string(&self, string: *const u16) -> Result {
        (self.0.output_string)(&self.0, string).into_result(())
    }

    pub fn test_string(&self, string: *const u16) -> Result {
        (self.0.test_string)(&self.0, string).into_result(())
    }

    pub fn query_mode(&self, mode_number: usize) -> Result<Geometry> {
        let mut column = 0;
        let mut row = 0;

        (self.0.query_mode)(&self.0, mode_number, &mut column, &mut row)
            .into_result(Geometry { column, row })
    }

    pub fn set_mode(&self, mode_number: usize) -> Result {
        (self.0.set_mode)(&self.0, mode_number).into_result(())
    }

    pub fn set_attribute(&self, attribute: usize) -> Result {
        (self.0.set_attribute)(&self.0, attribute).into_result(())
    }

    pub fn clear_screen(&self) -> Result {
        (self.0.clear_screen)(&self.0).into_result(())
    }

    pub fn set_cursor_position(&self, cursor: Geometry) -> Result {
        (self.0.set_cursor_position)(&self.0, cursor.column, cursor.row).into_result(())
    }

    pub fn mode(&self) -> &SimpleTextOutputMode {
        unsafe { &*(self.0.mode as *const SimpleTextOutputMode) }
    }
}

impl fmt::Write for SimpleTextOutputProtocol {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        #[cfg(not(feature = "alloc"))]
            {
                const BUFFER_SIZE: usize = 256;
                let mut utf16 = [0u16; BUFFER_SIZE + 1];
                let mut i = 0;
                for char8 in s.chars() {
                    utf16[i] = char8 as u16;
                    i += 1;
                    if i == BUFFER_SIZE {
                        utf16[BUFFER_SIZE] = 0;
                        i = 0;
                        match self.output_string(utf16.as_ptr()) {
                            Ok(_) => {}
                            Err(_) => return Err(core::fmt::Error::default())
                        }
                    }
                    if char8 == '\n' {
                        utf16[i] = '\r' as u16;
                        i += 1;
                        if i == BUFFER_SIZE {
                            utf16[BUFFER_SIZE] = 0;
                            i = 0;
                            match self.output_string(utf16.as_ptr()) {
                                Ok(_) => {}
                                Err(_) => return Err(core::fmt::Error::default())
                            }
                        }
                    }
                }

                if i != 0 {
                    utf16[i] = 0;
                    match self.output_string(utf16.as_ptr()) {
                        Ok(_) => {}
                        Err(_) => return Err(core::fmt::Error::default())
                    }
                }
                Ok(())
            }
        #[cfg(feature = "alloc")]
            {
                let mut s: alloc::vec::Vec<u16> = s.encode_utf16().collect();
                s.push(0);

                let mut index = 0;
                while index < s.len() {
                    if s[index] == '\n' as u16 {
                        index += 1;
                        s.insert(index, '\r' as u16);
                    }
                    index += 1;
                }

                match self.output_string(s.as_ptr()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(fmt::Error::default())
                }
            }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Geometry {
    column: usize,
    row: usize,
}

impl Geometry {
    pub fn column(&self) -> usize {
        self.column
    }

    pub fn row(&self) -> usize {
        self.row
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct SimpleTextOutputMode(uefi_core::protocols::console::text_output::SimpleTextOutputMode);

impl SimpleTextOutputMode {
    pub fn max_mode(&self) -> i32 {
        self.0.max_mode
    }

    pub fn mode_number(&self) -> i32 {
        self.0.mode
    }

    pub fn attribute(&self) -> i32 {
        self.0.attribute
    }

    pub fn cursor(&self) -> Geometry {
        Geometry { column: self.0.cursor_column as usize, row: self.0.cursor_row as usize }
    }
}
