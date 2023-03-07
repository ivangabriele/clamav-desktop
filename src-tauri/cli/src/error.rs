use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use colored::Colorize;

#[derive(Debug, Clone)]
pub struct CliError {
    message: String,
}

impl CliError {
    pub fn new(message: String, error: Box<dyn Error>) -> CliError {
        println!("{} {:?}", "Message:".red(), error);
        println!("{} {:?}", "Error:".bold().purple(), error);

        CliError { message }
    }
}

impl Display for CliError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", self.message)
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        &self.message
    }
}
