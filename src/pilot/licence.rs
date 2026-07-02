//! The `Licence` value object — the ICAO-universal identity of a pilot licence.

use serde::{Deserialize, Serialize};

use crate::error::{check_length, DomainError};

/// The ICAO-universal identity of a pilot licence (Annex 1).
///
/// A value object owned by the [`Pilot`](crate::Pilot) aggregate: the issuing
/// authority and the licence number. National detail lives downstream, keyed by
/// this identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "LicenceData", into = "LicenceData")]
pub struct Licence {
    issuing_authority: String,
    number: String,
}

impl Licence {
    /// Build a licence identity, validating both fields are 1–50 characters.
    pub fn new(
        issuing_authority: impl Into<String>,
        number: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let issuing_authority = issuing_authority.into();
        let number = number.into();
        check_length("issuing_authority", &issuing_authority, 1, 50)?;
        check_length("number", &number, 1, 50)?;
        Ok(Self {
            issuing_authority,
            number,
        })
    }

    /// The coded issuing authority, e.g. `CASA`, `FAA`.
    pub fn issuing_authority(&self) -> &str {
        &self.issuing_authority
    }

    /// The licence number as issued (formats may be alphanumeric).
    pub fn number(&self) -> &str {
        &self.number
    }
}

// `Licence` keeps its fields private so the only way to build one is through
// `new`, which enforces the length bounds. Serde's derived `Deserialize` would
// bypass that constructor and populate the fields directly, letting invalid
// values in from JSON. `LicenceData` is a plain, unchecked mirror of the wire
// shape: serde deserialises into it, then `try_from` funnels it through `new`
// so a deserialised licence is validated exactly like a hand-built one. The
// reverse `From` provides the serialise direction.
#[derive(Serialize, Deserialize)]
struct LicenceData {
    issuing_authority: String,
    number: String,
}

impl TryFrom<LicenceData> for Licence {
    type Error = DomainError;

    fn try_from(data: LicenceData) -> Result<Self, Self::Error> {
        Self::new(data.issuing_authority, data.number)
    }
}

impl From<Licence> for LicenceData {
    fn from(value: Licence) -> Self {
        Self {
            issuing_authority: value.issuing_authority,
            number: value.number,
        }
    }
}
