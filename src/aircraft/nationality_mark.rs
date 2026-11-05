// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The ICAO Annex 7 nationality mark value object.

use serde::{Deserialize, Serialize};

use crate::error::{is_capital_or_digit, ValidationError};

/// The nationality mark of an aircraft's registration marks (ICAO Annex 7),
/// such as `VH`, `N`, `G` or `A9C`. Annex 7 takes it from the nationality
/// symbols of the ITU radio call-sign series, so it is one to three
/// characters.
///
/// Checked by shape: 1 to 3 capital Roman letters or Arabic numerals, with at
/// least one letter (an all-digit mark would make a registration's rendering
/// ambiguous). Lowercase
/// is refused, not upper-cased. The mark is not checked against the States'
/// allocations. Serialises as the plain mark.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct NationalityMark(String);

impl NationalityMark {
    /// Parse a nationality mark, 1 to 3 capital letters or digits, with a letter.
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        let in_bounds = (1..=3).contains(&value.len());
        if !in_bounds
            || !value.bytes().all(is_capital_or_digit)
            || !value.bytes().any(|b| b.is_ascii_uppercase())
        {
            return Err(ValidationError::NationalityMark(value));
        }
        Ok(Self(value))
    }

    /// Borrow the underlying mark.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NationalityMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for NationalityMark {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for NationalityMark {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for NationalityMark {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl TryFrom<String> for NationalityMark {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<NationalityMark> for String {
    fn from(value: NationalityMark) -> Self {
        value.0
    }
}
