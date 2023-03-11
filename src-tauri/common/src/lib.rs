use std::error::Error;
use std::fmt;

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
