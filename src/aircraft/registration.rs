//! Aircraft value objects: the ICAO Doc 8643 type designator and the
//! Annex 7 registration marks.

use serde::{Deserialize, Serialize};

use crate::error::{check_length, DomainError};

/// An ICAO Doc 8643 aircraft type designator (e.g. `C172`, `B738`, `R44`).
///
/// 2–4 characters, a leading letter followed by letters or digits. `ZZZZ` is
/// the accepted sentinel for an aircraft with no assigned designator.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AircraftType(String);

impl AircraftType {
    /// Parse and validate a type designator (Python regex `^[A-Z][A-Z0-9]{1,3}$`).
    pub fn parse(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        let len = value.chars().count();
        let leads_with_letter = value.chars().next().is_some_and(|c| c.is_ascii_uppercase());
        let rest_alnum_upper = value
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
        if !(2..=4).contains(&len) || !leads_with_letter || !rest_alnum_upper {
            return Err(DomainError::AircraftType(value));
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
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<AircraftType> for String {
    fn from(value: AircraftType) -> Self {
        value.0
    }
}

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

/// Serde shadow used to validate registration marks on deserialisation.
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
