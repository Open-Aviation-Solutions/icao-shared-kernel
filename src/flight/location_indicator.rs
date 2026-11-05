// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The ICAO Doc 7910 location indicator value object.

use serde::{Deserialize, Serialize};

use crate::error::{is_ascii_uppercase_of_len, ValidationError};

/// Doc 4444's mark for an aerodrome with no location indicator.
const NO_LOCATION_INDICATOR: &str = "ZZZZ";

/// An ICAO location indicator, such as an aerodrome's (e.g. `YSBK`, `KJFK`):
/// the four-letter code group that ICAO Doc 7910 lists.
///
/// Checked by shape: exactly four ASCII uppercase letters. Its leading
/// letters are not checked against Doc 7910's regions, nor the indicator
/// against the list. `ZZZZ` is refused: it is not an indicator but ICAO Doc
/// 4444's mark, in a flight plan (Appendix 2, Items 13 and 16), for an
/// aerodrome that has none, which is then described in Item 18 (an
/// [`UnlistedAerodrome`](crate::UnlistedAerodrome)). Serialises as the plain
/// code.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct LocationIndicator(String);

impl LocationIndicator {
    /// Parse a location indicator: exactly four ASCII uppercase letters,
    /// other than `ZZZZ`.
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if !is_ascii_uppercase_of_len(&value, 4) {
            return Err(ValidationError::LocationIndicator(value));
        }
        if value == NO_LOCATION_INDICATOR {
            return Err(ValidationError::NoLocationIndicator);
        }
        Ok(Self(value))
    }

    /// Borrow the underlying code.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for LocationIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for LocationIndicator {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for LocationIndicator {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for LocationIndicator {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl TryFrom<String> for LocationIndicator {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<LocationIndicator> for String {
    fn from(value: LocationIndicator) -> Self {
        value.0
    }
}
