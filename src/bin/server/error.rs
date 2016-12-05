use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ServerError(pub String);

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server error encountered. Description: '{}'", self.0)
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        "General server error."
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
