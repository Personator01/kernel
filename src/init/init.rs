#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kern_main(){
}

#[panic_handler]
fn panic(_pi: &PanicInfo)-> !{
    loop{}
}
