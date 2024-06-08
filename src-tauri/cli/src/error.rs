use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[allow(unused_imports)] // It's not unused, it's a trait augmenting `&str` and `String`.
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct CliError {
    message: String,
}

impl CliError {
    pub fn new(message: String, error: Box<dyn Error>) -> CliError {
        println!(
            "{} {} {:?}",
            "[ERROR]".bold().red(),
            "[MESSAGE]".red(),
            message
        );
        println!(
            "{} {} {:?}",
            "[ERROR]".bold().red(),
            "[BOX]".purple(),
            error
        );

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
