#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]

mod error;
mod io;

use io::vgatext::{put_text, put_text_offset_color, VgaColor};
use io::serial;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kern_main() {
    // let _ = match serial::configure_default(serial::SerialPort::COM1) {
    //     Ok(_) => put_text("initialized COM1"),
    //     Err(_) => put_text("failed to initialize COM1")
    // };
    let _ = match serial::configure_default(serial::SerialPort::COM1) {
        Ok(_) => put_text("A"),
        Err(_) => put_text("B")
    };
    let _ = put_text("we live in a society");
    loop{}
}

#[panic_handler]
fn panic(_pi: &PanicInfo)-> !{
    loop{}
}
