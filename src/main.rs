#![no_std]
#![no_main]
#![allow(dead_code)]

mod error;
mod io;

use io::vgatext::{put_text, put_text_offset_color, VgaColor};

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kern_main() {
    let _ = put_text_offset_color("aaa", 4, VgaColor::Magenta, VgaColor::DarkGrey, true);
    let _ = put_text_offset_color("aaa", 8, VgaColor::Magenta, VgaColor::DarkGrey, false);
    loop{}
}

#[panic_handler]
fn panic(_pi: &PanicInfo)-> !{
    loop{}
}
