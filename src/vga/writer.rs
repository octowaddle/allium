use crate::vga::{
    buffer::{self, Buffer},
    character::Character,
    color::{Color, ColorCode},
};
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::default());
}

pub struct Writer {
    column: usize,
    row: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn default() -> Self {
        Writer {
            column: 0,
            row: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.write_new_line(),
            ascii_code => {
                if self.column >= buffer::BUFFER_WIDTH {
                    self.write_new_line();
                }

                let screen_character = Character::new(ascii_code, self.color_code);

                self.buffer.chars[self.row][self.column].write(screen_character);
                self.column += 1;
            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_new_line(&mut self) {
        if self.row < buffer::BUFFER_HEIGHT - 1 {
            self.row += 1;
        } else {
            for row in 1..buffer::BUFFER_HEIGHT {
                for column in 0..buffer::BUFFER_WIDTH {
                    let character = self.buffer.chars[row][column].read();
                    self.buffer.chars[row - 1][column].write(character);
                }
            }

            self.write_clean_row(buffer::BUFFER_HEIGHT - 1);
        }

        self.column = 0;
    }

    fn write_clean_row(&mut self, row: usize) {
        let blank_screen_character = Character::new(b' ', self.color_code);

        for column in 0..buffer::BUFFER_WIDTH {
            self.buffer.chars[row][column].write(blank_screen_character);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_string(string);
        Ok(())
    }
}
