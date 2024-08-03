/**
* VGA text mode functionality.
* This may be removed at some point in favor of UEFI framebuffer rendering.
*/

use core::ptr::write_volatile;
use core::str;

use crate::error::kerror::KError;
use crate::error::kerror::KErrorType;


const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

const VGA_BUF: *mut u16 = 0xB8000 as *mut u16;
const TERM_COLOR: u8 = get_vga_color(VgaColor::Green, VgaColor::Black, false);

pub enum VgaColor {
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


const fn get_vga_color(fg: VgaColor, bg: VgaColor, blink: bool) -> u8 {
     ((bg as u8) << 4) | fg as u8 | ((blink as u8) << 7)
}

/**
* Returns the VGA text mode byte pair formed from the specified character and color.
* Character should be from the 7-bit ascii table.
*/
fn get_byte_pair(character: char, color: u8) -> u16 {
    ((color as u16) << 8) | (character as u16)
}

/**
* Puts a VGA text mode byte pair to the buffer at the specified index.
* Index must not be outside the bounds of the buffer (80x25)
*/
unsafe fn put_char(pair: u16, idx: usize) {
    write_volatile(VGA_BUF.wrapping_add(idx), pair);
}

/**
* Write text to the vga text buffer.
* Input string must be smaller than the buffer size.
*/
pub fn put_text(text: &str) -> Result<(), KError> {
    put_text_offset(text, 0)
}

/**
* Write text to the vga text buffer.
* Input string plus offset must be smaller than the buffer size.
*/
pub fn put_text_offset(text: &str, offset: usize) -> Result<(), KError> {
    put_text_internal(text, offset, TERM_COLOR)
}

/**
* Write text to the vga text buffer.
* Input string size plus offset must be smaller than the buffer size.
*/
pub fn put_text_offset_color(text: &str, offset: usize, fg: VgaColor, bg: VgaColor, blink: bool) -> Result<(), KError> {
    put_text_internal(text, offset, get_vga_color(fg, bg, blink))
}

fn put_text_internal(text: &str, offset: usize, color: u8) -> Result<(), KError> {
    if text.len() + offset > VGA_WIDTH * VGA_HEIGHT {
        return Err(KError::new(KErrorType::InvalidInput, "Text length must be less than VGA_WIDTH"));
    }
    /*
    * Guaranteed to write within buffer bounds due to above length check 
    */     
    unsafe {
        text.chars().enumerate().for_each(|(idx, character)| { put_char(get_byte_pair(character, color), idx + offset) });
    }
    return Ok(());
}
