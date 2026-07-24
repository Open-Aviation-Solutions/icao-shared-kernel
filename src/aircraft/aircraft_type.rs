//! The ICAO Doc 8643 aircraft type designator value object.

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

/// An ICAO Doc 8643 aircraft type designator (e.g. `C172`, `B738`, `R44`).
///
/// 2–4 characters, a leading letter followed by letters or digits. `ZZZZ` is
/// the accepted sentinel for an aircraft with no assigned designator.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AircraftType(String);

impl AircraftType {
    /// Parse and validate a type designator: 2–4 characters, a leading letter
    /// followed by letters or digits.
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        let len = value.chars().count();
        let leads_with_letter = value.chars().next().is_some_and(|c| c.is_ascii_uppercase());
        let rest_alnum_upper = value
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
        if !(2..=4).contains(&len) || !leads_with_letter || !rest_alnum_upper {
            return Err(ValidationError::AircraftType(value));
        }
        Ok(Self(value))
    }

    /// Borrow the underlying designator string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AircraftType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for AircraftType {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<AircraftType> for String {
    fn from(value: AircraftType) -> Self {
        value.0
    }
}
