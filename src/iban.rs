//! Module for the IBAN representation and methods.

use crate::country::Country;

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

    /// Returns the country code as a string, for example "GB".
    pub fn country_code(&self) -> String {
        self.country.to_string()
    }
}
