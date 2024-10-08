use serde_json::Error as SerdeError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum NamedColorsError {
    ParseError(SerdeError),
    Custom(String), // Error message for custom
}

impl fmt::Display for NamedColorsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            NamedColorsError::ParseError(ref err) => write!(f, "Parse error: {}", err),
            NamedColorsError::Custom(ref message) => write!(f, "Error: {}", message), // View error message for custom
        }
    }
}

impl Error for NamedColorsError {}

impl From<SerdeError> for NamedColorsError {
    fn from(err: SerdeError) -> NamedColorsError {
        NamedColorsError::ParseError(err)
    }
}
