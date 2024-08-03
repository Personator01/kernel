use core::ptr;
use core::str;

use crate::error::kerror::KError;
use crate::error::kerror::KErrorType;


const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

const VGA_BUF: *const u16 = 0xB8000 as *const u16;

enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}


fn get_vga_color(fg: VgaColor, bg: VgaColor) -> u16 {
     ((bg as u16) << 4) | fg as u16 
}

pub fn put_text(text: &str) -> Result<(), KError> {
    if text.len() > VGA_WIDTH {
        return Err(KError::new(KErrorType::InvalidInput, "Text length must be less than VGA_WIDTH"));
    }
    Ok(())
}
