//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! # schwifty
//!
//! A simple IBAN validation library inspired by Python's `schwifty`.
//!
//! ## Sample Usage
//! ```rust
//!assert!(schwifty::validate("GB82 WEST 1234 5698 7654 32").is_ok());
//! ```
//!
//! [ci]: https://github.com/Elinvynia/schwifty/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/schwifty/Rust/master?style=flat-square
//! [docs]: https://docs.rs/schwifty
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/schwifty
//! [crate-version]: https://img.shields.io/crates/v/schwifty.svg?style=flat-square

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

pub use crate::country::Country;
pub use crate::error::ValidationError;
use std::str::FromStr;

pub mod country;
pub(crate) mod country_specific;
pub mod error;

#[allow(clippy::all)]
pub(crate) mod u256 {
    uint::construct_uint! {
        pub(crate) struct U256(4);
    }
}

/// Represents an IBAN and provides helpful methods.
#[derive(Debug)]
#[non_exhaustive]
pub struct IBan {
    /// The country of this IBAN.
    pub country: Country,
    pub(crate) raw: String,
}

impl IBan {
    /// Returns the account number of the IBAN.
    pub fn account_number(&self) -> u128 {
        self.country.account_number(&self.raw)
    }

    /// Returns the national bank code of the IBAN.
    pub fn bank_code(&self) -> String {
        self.country.bank_code(&self.raw)
    }

    /// Returns the country code as a String, for example "GB".
    pub fn country_code(&self) -> String {
        self.country.to_string()
    }

    /// Access the raw String this IBAN was made from.
    pub fn raw(&self) -> &str {
        &self.raw
    }
}

/// Checks if the provided string is a valid IBAN, or tells you why it isn't.
pub fn validate<I: AsRef<str>>(input: I) -> Result<IBan, ValidationError> {
    let input = input.as_ref();

    // Remove the whitespace.
    let input: String = input.split_whitespace().collect();

    // IBAN can be at most 34 characters(bytes) long.
    if input.len() > 34 {
        return Err(ValidationError::TooLong);
    };

    // All of the characters must be alphanumeric.
    if !input.chars().all(|ch| ch.is_alphanumeric()) {
        return Err(ValidationError::InvalidChar);
    };

    // See if it is a valid Country
    let country_code = &input[0..2];
    let country = match Country::from_str(country_code) {
        Ok(c) => c,
        Err(_) => return Err(ValidationError::InvalidCountryCode),
    };

    // Since it is a valid country, check if it is the proper length.
    if input.len() != country.length() {
        return Err(ValidationError::InvalidLength);
    }

    // Also check if the format matches
    if !country.format().is_match(&input) {
        return Err(ValidationError::InvalidFormat);
    }

    // Do country-specifich checks.
    if !country.custom_validation(&input) {
        return Err(ValidationError::CountryCheckFailed);
    }

    // Put the country code to the end of the string.
    let (start, rest) = input.split_at(4);
    let mut rearranged = String::with_capacity(34);
    rearranged.push_str(rest);
    rearranged.push_str(start);

    // Convert ASCII letters to their code, don't modify numbers.
    let mut integer_string = String::with_capacity(34);
    for ch in rearranged.chars() {
        if ch.is_numeric() {
            integer_string.push(ch);
        } else {
            // This will not panic as we are guaranteed A-Z, a-z
            let x = ch.to_digit(36).unwrap().to_string();
            integer_string.push_str(&x)
        }
    }

    // This will not panic as u256 can hold any IBAN.
    let integer = u256::U256::from_dec_str(&integer_string).unwrap();

    // Make sure that the remainder is one.
    if integer % 97 != 1.into() {
        return Err(ValidationError::InvalidIBAN);
    }

    Ok(IBan {
        country,
        raw: input,
    })
}
