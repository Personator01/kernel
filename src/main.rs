#![no_std]
#![no_main]

mod error;
mod init;

use init::vgatext::put_text;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kern_main() {
    let _ = put_text("aaaa");
    loop{}
}

#[panic_handler]
fn panic(_pi: &PanicInfo)-> !{
    loop{}
}
