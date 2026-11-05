// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The ICAO Annex 7 registration marks value object.

use serde::{Deserialize, Serialize};

use super::NationalityMark;
use crate::error::{is_capital_or_digit, ValidationError};

/// Aircraft nationality and registration marks (ICAO Annex 7).
///
/// Displays as the nationality mark and the registration mark joined by a
/// hyphen (`VH-ABC`), or with no hyphen when the registration mark begins with
/// a digit (`N12345`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "RegistrationData", into = "RegistrationData")]
pub struct AircraftRegistration {
    nationality: NationalityMark,
    registration: String,
}

impl AircraftRegistration {
    /// Build registration marks from a nationality mark and a registration
    /// mark, validating the registration mark: 1 to 5 capital letters or digits
    /// (lowercase is refused, not upper-cased).
    pub fn new(
        nationality: NationalityMark,
        registration: impl Into<String>,
    ) -> Result<Self, ValidationError> {
        let registration = registration.into();
        if !(1..=5).contains(&registration.len()) || !registration.bytes().all(is_capital_or_digit)
        {
            return Err(ValidationError::RegistrationMark(registration));
        }
        Ok(Self {
            nationality,
            registration,
        })
    }

    /// The nationality mark (e.g. `VH`, `N`, `G`, `A9C`).
    pub fn nationality(&self) -> &NationalityMark {
        &self.nationality
    }

    /// The registration mark (capital letters and digits) following the nationality mark.
    pub fn registration(&self) -> &str {
        &self.registration
    }
}

impl std::fmt::Display for AircraftRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hyphen = if self.registration.starts_with(|c: char| c.is_ascii_digit()) {
            ""
        } else {
            "-"
        };
        write!(f, "{}{hyphen}{}", self.nationality, self.registration)
    }
}

// `AircraftRegistration` keeps its fields private so the only way to build one
// is through `new`, which enforces the length bounds. Serde's derived
// `Deserialize` would bypass that constructor and populate the fields directly,
// letting invalid marks in from JSON. `RegistrationData` is a plain, unchecked
// mirror of the wire shape: serde deserialises into it, then `try_from` funnels
// it through `new` so a deserialised value is validated exactly like a
// hand-built one. The reverse `From` provides the serialise direction.
#[derive(Serialize, Deserialize)]
struct RegistrationData {
    nationality: NationalityMark,
    registration: String,
}

impl TryFrom<RegistrationData> for AircraftRegistration {
    type Error = ValidationError;

    fn try_from(data: RegistrationData) -> Result<Self, Self::Error> {
        Self::new(data.nationality, data.registration)
    }
}

impl From<AircraftRegistration> for RegistrationData {
    fn from(value: AircraftRegistration) -> Self {
        Self {
            nationality: value.nationality,
            registration: value.registration,
        }
    }
}
