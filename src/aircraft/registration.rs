//! The ICAO Annex 7 registration marks value object.

use serde::{Deserialize, Serialize};

use crate::error::{check_length, DomainError};

/// Aircraft nationality and registration marks (ICAO Annex 7).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "RegistrationData", into = "RegistrationData")]
pub struct AircraftRegistration {
    nationality: String,
    registration: String,
}

impl AircraftRegistration {
    /// Build registration marks, validating the Annex 7 length bounds.
    pub fn new(
        nationality: impl Into<String>,
        registration: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let nationality = nationality.into();
        let registration = registration.into();
        check_length("nationality", &nationality, 1, 2)?;
        check_length("registration", &registration, 1, 5)?;
        Ok(Self {
            nationality,
            registration,
        })
    }

    /// The one- or two-character nationality mark (e.g. `VH`, `N`, `G`).
    pub fn nationality(&self) -> &str {
        &self.nationality
    }

    /// The alphanumeric registration mark following the nationality mark.
    pub fn registration(&self) -> &str {
        &self.registration
    }
}

impl std::fmt::Display for AircraftRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.nationality, self.registration)
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
    nationality: String,
    registration: String,
}

impl TryFrom<RegistrationData> for AircraftRegistration {
    type Error = DomainError;

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
