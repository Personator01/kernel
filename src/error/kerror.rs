pub struct KError<'a> {
    error_type: KErrorType,
    message: &'a str
}

pub enum KErrorType {
    InvalidInput,
    HardwareError
}

impl<'a> KError<'a> {
    pub fn new(error_type: KErrorType, message: &str) -> KError {
        KError { error_type, message }
    }
}
