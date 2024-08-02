#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[no_mangle]
pub extern "C" fn kern_main() {
    unsafe{
        asm! {
            "nop",
            "mov %ebx, %esp",
            options(att_syntax)
        }
    }
}

#[panic_handler]
fn panic(_pi: &PanicInfo)-> !{
    loop{}
}
