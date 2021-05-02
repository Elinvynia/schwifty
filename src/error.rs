//! Module holding the error type.

use std::fmt::{self, Display};

/// Error type for errors originating from this crate.
#[derive(Debug)]
pub enum ValidationError {
    /// IBAN can be at most 34 characters long.
    TooLong,
    /// IBAN cannot contain non-alphanumeric characters.
    InvalidChar,
    /// The IBAN checksum was invalid (remainder of mod 97 was not 1).
    InvalidIban,
    /// Input didn't contain a supported country code.
    InvalidCountryCode,
    /// IBAN length didn't match the detected country.
    InvalidLength,
    /// The format was wrong for the detected country.
    InvalidFormat,
    /// A custom check this country implements has failed.
    CountryCheckFailed,
}

impl std::error::Error for ValidationError {}

impl Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ValidationError::*;
        let msg = match self {
            TooLong => "Input is longer than 34 characters.",
            InvalidChar => "Input contains at least one invalid character.",
            InvalidIban => "IBAN mod 97 checksum is invalid.",
            InvalidCountryCode => "Input doesn't contain a supported country.",
            InvalidLength => "Input is invalid length for the detected country.",
            InvalidFormat => "IBAN has the wrong format for the detected country.",
            CountryCheckFailed => "Failed custom country-specific check.",
        };
        write!(f, "{}", msg)
    }
}
