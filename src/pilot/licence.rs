// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `Licence` value object: the ICAO-universal identity of a pilot licence,
//! and the titles of licence held under it.

use serde::{Deserialize, Serialize};

use super::licence_title::IssuedTitle;
use crate::error::{check_issuing_state, check_non_empty, ValidationError};

/// A pilot licence as its State numbers it (Annex 1, 5.2.1 I and III), and the
/// titles of licence held under it, each from the date it was first issued.
///
/// A value object owned by the [`Pilot`](crate::Pilot) aggregate. The licence
/// is identified by `(issuing_state, issuing_authority, number)`: the issuing
/// State disambiguates authorities that share an acronym across countries, so
/// the triple is globally unique. National detail lives downstream, keyed by
/// this identity. Equality and hashing are a value object's, over every field,
/// the titles included: two values that differ only in their titles are the
/// same licence but not equal, so to ask whether two values are the same
/// licence, compare the triple.
///
/// `number` is a string, not an integer: while ICAO Annex 1, 5.2.1 III,
/// describes the serial number in Arabic numerals, real national identifiers
/// vary — some States are purely numeric (e.g. Australia, the US) while EASA
/// States embed letters (a country prefix and `FCL`). Annex 1 sets no length
/// limit, so only non-emptiness is enforced.
///
/// `titles` is a list because a State can hold several licences under one
/// number, each first issued on its own date: a private licence for aeroplanes
/// and, later, a commercial one, or a licence for an additional category of
/// aircraft endorsed on the original (2.1.1.2.1 b)). Each title is one licence
/// (5.2.1 II, with its category under 2.1.1.2), so the same title listed twice
/// is refused: it would carry two dates for one licence. The titles are kept
/// in `LicenceTitle`'s order, not as given, so that equality does not depend
/// on the order they were listed in. The list may be empty: a State's licence
/// can hold only national licences that are not Annex 1's (CASA's recreational
/// pilot licence, for one), which belong downstream, keyed by this licence's
/// identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "LicenceData", into = "LicenceData")]
pub struct Licence {
    issuing_state: String,
    issuing_authority: String,
    number: String,
    titles: Vec<IssuedTitle>,
}

impl Licence {
    /// Build a licence, validating the issuing State is an ISO 3166-1 alpha-2
    /// code, that the authority and number are non-empty, and that no title of
    /// licence (with its category of aircraft) is listed twice.
    pub fn new(
        issuing_state: impl Into<String>,
        issuing_authority: impl Into<String>,
        number: impl Into<String>,
        mut titles: Vec<IssuedTitle>,
    ) -> Result<Self, ValidationError> {
        let issuing_state = issuing_state.into();
        let issuing_authority = issuing_authority.into();
        let number = number.into();
        check_issuing_state(&issuing_state)?;
        check_non_empty("issuing_authority", &issuing_authority)?;
        check_non_empty("number", &number)?;
        titles.sort_by_key(|issued| issued.title);
        if let Some(pair) = titles
            .windows(2)
            .find(|pair| pair[0].title == pair[1].title)
        {
            return Err(ValidationError::RepeatedLicenceTitle(pair[0].title));
        }
        Ok(Self {
            issuing_state,
            issuing_authority,
            number,
            titles,
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

    /// The titles of licence held under this licence, in `LicenceTitle`'s
    /// order: the order they were given in means nothing, so it is not kept,
    /// and two licences holding the same titles are equal however listed.
    pub fn titles(&self) -> &[IssuedTitle] {
        &self.titles
    }
}

// `Licence` keeps its fields private so the only way to build one is through
// `new`, which enforces the invariants. Serde's derived `Deserialize` would
// bypass that constructor and populate the fields directly, letting invalid
// values in from JSON. `LicenceData` is a plain, unchecked mirror of the wire
// shape: serde deserialises into it, then `try_from` funnels it through `new`
// so a deserialised licence is validated exactly like a hand-built one. The
// reverse `From` provides the serialise direction. `titles` has no default:
// a licence without the field is refused.
#[derive(Serialize, Deserialize)]
struct LicenceData {
    issuing_state: String,
    issuing_authority: String,
    number: String,
    titles: Vec<IssuedTitle>,
}

impl TryFrom<LicenceData> for Licence {
    type Error = ValidationError;

    fn try_from(data: LicenceData) -> Result<Self, Self::Error> {
        Self::new(
            data.issuing_state,
            data.issuing_authority,
            data.number,
            data.titles,
        )
    }
}

impl From<Licence> for LicenceData {
    fn from(value: Licence) -> Self {
        Self {
            issuing_state: value.issuing_state,
            issuing_authority: value.issuing_authority,
            number: value.number,
            titles: value.titles,
        }
    }
}
