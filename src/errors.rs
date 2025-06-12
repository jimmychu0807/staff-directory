use std::{error, fmt};

#[derive(Debug)]
pub struct ApplicationError(pub String);

impl fmt::Display for ApplicationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Application Error: {}", self.0)
	}
}

impl error::Error for ApplicationError {}
