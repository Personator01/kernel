
/**
* Functionality for writing to a serial port.
*/


pub enum SerialPort {
    COM1,
}

fn get_addr(ad: SerialPort) -> *mut u8 {
    (match ad {
        SerialPort::COM1 => 0x3f8
    }) as *mut u8 
}


/*
* Sets divisor latch access bit, in order to access baud rate register of the serial controller
*/
fn set_dlab(addr: *mut u8, bit: bool) {
    let lcr_addr = addr.wrapping_add(3);
    let mask = 0b01111111;
    let val = (bit as u8) << 7;
    unsafe {
        let prev_value = lcr_addr.read_volatile();
        lcr_addr.write_volatile(prev_value & mask | val);
    }
}

/*
* Sets the baud rate divisor for the serial controller
*/
fn set_divisor(addr: *mut u8, divisor: u16) {
    set_dlab(addr, true);
    let least_significant_byte = divisor as u8;
    let most_significant_byte = (divisor >> 8) as u8;
    let lsb_addr = addr;
    let msb_addr = addr.wrapping_add(1);
    unsafe {
        lsb_addr.write_volatile(least_significant_byte);
        msb_addr.write_volatile(most_significant_byte);
    }
    set_dlab(addr, false);
}

/*
* Sets the character bit width.
* All but the two least significant bits of width will be ignored, that is, the bit width will be
* set to (width mod 4) + 4. 
*/
fn set_char_width(addr: *mut u8, width: u8) {
    let width_masked = width & 0b00000011;
    let lcr_addr = addr.wrapping_add(3);
    let prev_val_mask: u8 = 0b11111100;
    unsafe{
        let prev_value = lcr_addr.read_volatile();
        lcr_addr.write_volatile(prev_value & prev_val_mask | width_masked); 
    }

}

