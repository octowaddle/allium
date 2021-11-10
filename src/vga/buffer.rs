use crate::vga::character::Character;
use volatile::Volatile;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
