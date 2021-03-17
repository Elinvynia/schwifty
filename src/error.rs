//! Module holding the error type.

use std::fmt::{self, Display};

/// Error type for errors originating from this crate.
#[derive(Debug)]
pub enum ValidationError {
    /// IBAN can be at most 34 characters long.
    TooLong,
    /// IBAN cannot contain non-alphanumeric characters.
    InvalidChar,
    /// The IBAN checksum was invalid (remainder was not 1).
    InvalidIBAN,
    /// Input didn't contain a supported country code.
    InvalidCountryCode,
    /// IBAN length didn't match the detected country.
    InvalidLength,
    /// The format was wrong for the detected country.
    InvalidFormat,
}

impl std::error::Error for ValidationError {}

impl Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ValidationError::*;
        match self {
            TooLong => write!(f, "Input is too long."),
            InvalidChar => write!(f, "Input contains invalid character."),
            InvalidIBAN => write!(f, "IBAN checksum is invalid."),
            InvalidCountryCode => write!(f, "Input doesn't contain a country."),
            InvalidLength => write!(f, "Input is invalid length for the country."),
            InvalidFormat => write!(f, "IBAN has the wrong format for the country."),
        }
    }
}
