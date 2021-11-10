use crate::vga::color::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Character {
    ascii_code: u8,
    color_code: ColorCode,
}

impl Character {
    pub fn new(ascii_code: u8, color_code: ColorCode) -> Self {
        Character {
            ascii_code,
            color_code,
        }
    }
}
