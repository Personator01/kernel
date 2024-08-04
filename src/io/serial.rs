
use crate::error::kerror::{KError, KErrorType};

/**
* Functionality for writing to a serial port.
*
* All unsafe blocks are guaranteed to function correctly as long as get_addr maps to valid serial
* controller addresses.
*/


#[derive(Clone)]
#[derive(Copy)]
pub enum SerialPort {
    COM1,
}

fn get_addr(port: SerialPort) -> *mut u8 {
    (match port {
        SerialPort::COM1 => 0x3f8
    }) as *mut u8 
}


/*
* Writes into the line control register of the serial controller.
* zero_mask is a bit containing 0s for the bit positions to be zeroed out
* value mask is a bit value containing the value to write into the register
*/
fn write_lcr(port: SerialPort, zero_mask: u8, value_mask: u8){
    let addr = get_addr(port);
    let lcr_addr = addr.wrapping_add(3);
    unsafe {
        let prev_value = lcr_addr.read_volatile();
        lcr_addr.write_volatile((prev_value & zero_mask) | value_mask);
    }
}

/*
* Writes into the fifo control register of the serial controller
* value mask is a bit value containing the value to write into the register
* Since the fcr is write-only, there is no need for a zero mask
*/
fn write_fcr(port: SerialPort, value_mask: u8) {
    let addr = get_addr(port);
    let fcr_addr = addr.wrapping_add(2);
    unsafe {
        fcr_addr.write_volatile(value_mask);
    }
}

/*
* Writes into the modem control register of the serial controller
* value mask is a bit value containing the value to write into the register
* zero mask is a bit mask containing 0s for the bit positions to be zeroed out
*/
fn write_mcr(port: SerialPort, zero_mask: u8, value_mask: u8) {
    let addr = get_addr(port);
    let mcr_addr = addr.wrapping_add(4);
    unsafe {
        let prev_value = mcr_addr.read_volatile();
        mcr_addr.write_volatile((prev_value & zero_mask) | value_mask);
    }
}

/**
* Gets the value of the line status register
*/
fn get_lsr(port: SerialPort) -> u8 {
    let lsr_addr = get_addr(port).wrapping_add(5);
    unsafe {
        return lsr_addr.read_volatile();
    }
}

/*
* Sets divisor latch access bit, in order to access baud rate register of the serial controller
*/
fn set_dlab(port: SerialPort, bit: bool) {
    let zero_mask = 0b01111111;
    let value_mask = (bit as u8) << 7;
    write_lcr(port, zero_mask, value_mask);
}


/*
* Sets the baud rate divisor for the serial controller
*/
pub fn set_divisor(port: SerialPort, divisor: u16) {
    set_dlab(port, true);
    let addr = get_addr(port);
    let least_significant_byte = divisor as u8;
    let most_significant_byte = (divisor >> 8) as u8;
    let lsb_addr = addr;
    let msb_addr = addr.wrapping_add(1);
    unsafe {
        lsb_addr.write_volatile(least_significant_byte);
        msb_addr.write_volatile(most_significant_byte);
    }
    set_dlab(port, false);
}

/*
* Sets the character bit width.
* All but the two least significant bits of width will be ignored, that is, the bit width will be
* set to (width mod 4) + 4. 
* That is, the value passed in maps to the actual width value as shown below
* | Given value | Actual char width |
* |      0      |        5          |
* |      1      |        6          |
* |      2      |        7          |
* |      3      |        8          |
*/
pub fn set_char_width(port: SerialPort, width: u8) {
    let value_mask = width & 0b00000011;
    let zero_mask: u8 = 0b11111100;
    write_lcr(port, zero_mask, value_mask)
}

/**
* Sets the stop bits of the serial connection.
* false -> 1
* true -> 1.5/2
*/
pub fn set_stop_bit(port: SerialPort, value: bool) {
    let zero_mask = 0b1111101;
    let value_mask = (value as u8) << 1;
    write_lcr(port, zero_mask, value_mask);
}

#[derive(Clone)]
#[derive(Copy)]
pub enum Parity {
    None,
    Odd,
    Even,
    Mark,
    Space
}

/**
* Sets the parity bits of the serial connection
*/
pub fn set_parity(port: SerialPort, value: Parity) {
    let zero_mask = 0b11000111;
    let value_mask = match value{
        Parity::None => 0b00000000,
        Parity::Odd => 0b00001000,
        Parity::Even => 0b00011000,
        Parity::Mark => 0b00101000,
        Parity::Space => 0b00111000
    };
    write_lcr(port, zero_mask, value_mask);
}

/**
* Sets whether or not the fifo buffers are enabled for the serial controller
*/
pub fn enable_fifo(port: SerialPort, enable: bool) {
    let value_mask = enable as u8;
    write_fcr(port, value_mask);
}

/**
* Clears transmit fifo buffer for the serial controller
*/
pub fn clear_transmit_fifo(port: SerialPort) {
    let value_mask = 0b00000100;
    write_fcr(port, value_mask);
}

/**
* Clears receive fifo buffer for the serial controller
*/
pub fn clear_receive_fifo(port: SerialPort) {
    let value_mask = 0b00000010;
    write_fcr(port, value_mask);
}

/**
* Sets the number of bytes received in a fifo before an interrupt is raised.
* Input should be within the range 0-3, otherwise it will be truncated to fit within that range.
* | Value | Trigger Level |
* |   0   |      1B       |
* |   1   |      4B       |
* |   2   |      8B       |
* |   3   |      14B      |
*/
pub fn set_interrupt_trigger_level(port: SerialPort, value: u8) {
    let value_mask = (value & 0b00000011) << 6;
    write_fcr(port, value_mask);
}


/**
* Enables or disables interrupts from a serial controller
*/
pub fn enable_interrupts(port: SerialPort, value: bool) {
    set_dlab(port, false);
    let ier_addr = get_addr(port).wrapping_add(1);
    unsafe {
        ier_addr.write_volatile(value as u8);
    }
}

/**
* Set data terminal ready bit of the modem control register
*/
pub fn set_dtr(port: SerialPort, value: bool) {
    let value_mask = value as u8;
    let zero_mask = 0b11111110;
    write_mcr(port, zero_mask, value_mask);
}

/**
* Set request to send bit of the modem control register
*/
pub fn set_rts(port: SerialPort, value: bool) {
    let value_mask = (value as u8) << 1;
    let zero_mask = 0b11111101;
    write_mcr(port, zero_mask, value_mask);
}

/**
* Set out 1 bit of the modem control register
*/
pub fn set_out_1(port: SerialPort, value: bool) {
    let value_mask = (value as u8) << 2;
    let zero_mask = 0b11111011;
    write_mcr(port, zero_mask, value_mask);
}

/**
* Set out 2 bit of the modem control register
*/
pub fn set_out_2(port: SerialPort, value: bool) {
    let value_mask = (value as u8) << 3;
    let zero_mask = 0b11110111;
    write_mcr(port, zero_mask, value_mask);
}

/**
* Set loopback bit of the modem control register
*/
pub fn set_loopback(port: SerialPort, value: bool) {
    let value_mask = (value as u8) << 4;
    let zero_mask = 0b11101111;
    write_mcr(port, zero_mask, value_mask);
}

/**
* Gets line status register data ready bit
*/
pub fn is_data_ready(port: SerialPort) -> bool {
    (get_lsr(port) & 0b00000001) != 0
}

/**
* Gets line status register overrun error bit
*/
pub fn is_overrun_error(port: SerialPort) -> bool {
    (get_lsr(port) & 0b00000010) != 0
}

/**
* Gets line status register parity error bit
*/
pub fn is_parity_error(port: SerialPort) -> bool {
    (get_lsr(port) & 0b00000100) != 0
}

/**
* Gets line status register framing error bit
*/
pub fn is_framing_error(port: SerialPort) -> bool {
    (get_lsr(port) & 0b00001000) != 0
}

/**
* Gets line status register break indicator bit
*/
pub fn is_break(port: SerialPort) -> bool {
    (get_lsr(port) & 0b00010000) != 0
}

/**
* Gets line status register tramitter holding register empty bit
*/
pub fn is_transmitter_holding_empty(port: SerialPort) -> bool {
    (get_lsr(port) & 0b00100000) != 0
}

/**
* Gets line status register transmitter empty bit
*/
pub fn is_transmitter_empty(port: SerialPort) -> bool {
    (get_lsr(port) & 0b01000000) != 0
}

/**
* Gets line status register impending error bit
*/
pub fn is_impending_error(port: SerialPort) -> bool {
    (get_lsr(port) & 0b10000000) != 0
}

/**
* Writes a byte to the serial controller. Does not check buffers or if they are ready.
*/
fn write_byte(port: SerialPort, data: u8) {
    unsafe {
        get_addr(port).write_volatile(data);
    }
}

/**
* Reads a byte from the serial controller. Does not check buffers or if they are ready.
*/
fn read_byte(port: SerialPort) -> u8 {
    unsafe {
        get_addr(port).read_volatile()
    }
}

/**
* Sets a default configuration for the given port, and tests that it is functional.
*/
pub fn configure_default(port: SerialPort) -> Result<(), KError<'static>>{
    enable_interrupts(port, false);
    set_divisor(port, 3);
    set_char_width(port, 8);
    set_parity(port, Parity::None);
    set_stop_bit(port, false);
    
    enable_fifo(port, true);
    clear_receive_fifo(port);
    clear_transmit_fifo(port);
    set_interrupt_trigger_level(port, 3);
    enable_interrupts(port, true);
    set_dtr(port, true);
    set_rts(port, true);
    set_loopback(port, true);

    let test_val = 0xAE;
    write_byte(port, test_val);

    if read_byte(port) != test_val {
        return Err(KError::new(KErrorType::HardwareError, "Serial controller failed loopback test"));
    }

    set_loopback(port, false);
    set_out_1(port, true);
    set_out_2(port, true);

    return Ok(());
}
