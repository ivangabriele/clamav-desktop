use std::error::Error;
use std::fmt;

pub mod utils;

// https://stackoverflow.com/a/51345372/2736233
// TODO Add a logger in this macro?
#[macro_export]
macro_rules! ok_or_return_none {
    ( $e:expr ) => {
        match $e {
            Some(value) => value,
            None => return None,
        }
    };
}

#[derive(Debug)]
pub struct CommonError {
    message: String,
}

impl fmt::Display for CommonError {
    fn fmt(&self, fornatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fornatter, "{}", self.message)
    }
}

impl Error for CommonError {}
impl CommonError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
