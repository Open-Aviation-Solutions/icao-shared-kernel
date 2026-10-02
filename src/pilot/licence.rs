//! The `Licence` value object — the ICAO-universal identity of a pilot licence.

use serde::{Deserialize, Serialize};

use crate::error::{check_issuing_state, check_non_empty, ValidationError};

/// The ICAO-universal identity of a pilot licence (Annex 1).
///
/// A value object owned by the [`Pilot`](crate::Pilot) aggregate. The identity
/// is `(issuing_state, issuing_authority, number)`: the issuing State
/// disambiguates authorities that share an acronym across countries, so the
/// triple is globally unique. National detail lives downstream, keyed by this
/// identity.
///
/// `number` is a string, not an integer: while ICAO Annex 1 §5.2.1 III)
/// describes the serial number in Arabic numerals, real national identifiers
/// vary — some States are purely numeric (e.g. Australia, the US) while EASA
/// States embed letters (a country prefix and `FCL`). Annex 1 sets no length
/// limit, so only non-emptiness is enforced.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "LicenceData", into = "LicenceData")]
pub struct Licence {
    issuing_state: String,
    issuing_authority: String,
    number: String,
}

impl Licence {
    /// Build a licence identity, validating the issuing State is an ISO 3166-1
    /// alpha-2 code and that the authority and number are non-empty.
    pub fn new(
        issuing_state: impl Into<String>,
        issuing_authority: impl Into<String>,
        number: impl Into<String>,
    ) -> Result<Self, ValidationError> {
        let issuing_state = issuing_state.into();
        let issuing_authority = issuing_authority.into();
        let number = number.into();
        check_issuing_state(&issuing_state)?;
        check_non_empty("issuing_authority", &issuing_authority)?;
        check_non_empty("number", &number)?;
        Ok(Self {
            issuing_state,
            issuing_authority,
            number,
        })
    }

    /// The ISO 3166-1 alpha-2 code of the State that issued the licence.
    pub fn issuing_state(&self) -> &str {
        &self.issuing_state
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
// `new`, which enforces the invariants. Serde's derived `Deserialize` would
// bypass that constructor and populate the fields directly, letting invalid
// values in from JSON. `LicenceData` is a plain, unchecked mirror of the wire
// shape: serde deserialises into it, then `try_from` funnels it through `new`
// so a deserialised licence is validated exactly like a hand-built one. The
// reverse `From` provides the serialise direction.
#[derive(Serialize, Deserialize)]
struct LicenceData {
    issuing_state: String,
    issuing_authority: String,
    number: String,
}

impl TryFrom<LicenceData> for Licence {
    type Error = ValidationError;

    fn try_from(data: LicenceData) -> Result<Self, Self::Error> {
        Self::new(data.issuing_state, data.issuing_authority, data.number)
    }
}

impl From<Licence> for LicenceData {
    fn from(value: Licence) -> Self {
        Self {
            issuing_state: value.issuing_state,
            issuing_authority: value.issuing_authority,
            number: value.number,
        }
    }
}
