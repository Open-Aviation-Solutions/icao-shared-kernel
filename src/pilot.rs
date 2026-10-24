// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `Pilot` aggregate root and the licences it holds.

mod aircraft_category;
mod licence;
mod licence_title;
mod minimum_ages;

pub use aircraft_category::AircraftCategory;
pub use licence::Licence;
pub use licence_title::{
    AirlineTransportCategory, IssuedTitle, LicenceTitle, PrivateOrCommercialCategory,
};
pub use minimum_ages::MinimumAges;

use std::collections::HashSet;

use serde::{de, Deserialize, Deserializer, Serialize};
use time::Date;
use uuid::Uuid;

use crate::error::{check_length, ValidationError};

/// Pilot identity and the licences they hold.
///
/// The full name and date of birth are those every licence carries (ICAO Annex
/// 1: §5.2.1 IV and IVa on paper or card, Appendix 4 IVb and IVc for an
/// electronic licence), and that a logbook records (CASA reg 61.345(2)). They
/// belong to the pilot, not to a licence: a pilot holds them before holding any.
///
/// A pilot can hold licences from multiple States simultaneously (ICAO Annex 1
/// imposes no single-State restriction).
///
/// No title of licence the pilot holds may have been first issued before they
/// reached its minimum age (see [`MinimumAges`]). [`Pilot::with`] and plain
/// `Deserialize` check Annex 1's ages; a State whose ages differ builds with
/// [`Pilot::with_minimum_ages`] and reads with [`Pilot::deserialize_with`],
/// passing its own table. Read with plain `Deserialize`, a pilot of a State
/// whose age is below Annex 1's is refused, which is seen at once; one of a
/// State whose age is above Annex 1's is accepted when the State's own age
/// would refuse them, which is not. A pilot does not keep the table it was
/// checked against: one built with a State's lower ages serialises to JSON
/// that plain `Deserialize` refuses, and a type that embeds a `Pilot` with
/// derived `Deserialize` checks it against Annex 1's ages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "PilotData", into = "PilotData")]
pub struct Pilot {
    pub id: Uuid,
    full_name: String,
    date_of_birth: Date,
    licences: Vec<Licence>,
}

impl Pilot {
    /// Build a pilot with a fresh id and no licences.
    pub fn new(full_name: impl Into<String>, date_of_birth: Date) -> Result<Self, ValidationError> {
        Self::with(Uuid::new_v4(), full_name, date_of_birth, Vec::new())
    }

    /// Build a pilot from explicit parts. The full name is trimmed, and must
    /// then hold a name: 61.345(2) requires one. Its 100-character limit is a
    /// bound on input, not a regulatory one (neither Annex 1 nor Part 61 sets a
    /// length).
    ///
    /// The same licence, by its identity, is refused if listed twice: two
    /// entries could disagree on its titles and their dates.
    ///
    /// No title of licence may have been first issued before the holder
    /// reached Annex 1's minimum age for it ([`MinimumAges::default`]).
    ///
    /// The date of birth is not checked against today: the kernel has no
    /// clock, so that is for a consumer that has one. Nor is a title's date
    /// checked against today, or against the other titles' dates.
    pub fn with(
        id: Uuid,
        full_name: impl Into<String>,
        date_of_birth: Date,
        licences: Vec<Licence>,
    ) -> Result<Self, ValidationError> {
        Self::with_minimum_ages(
            id,
            full_name,
            date_of_birth,
            licences,
            &MinimumAges::default(),
        )
    }

    /// Build a pilot as [`Pilot::with`] does, checking each title's date of
    /// first issue against a State's own minimum ages in place of Annex 1's.
    pub fn with_minimum_ages(
        id: Uuid,
        full_name: impl Into<String>,
        date_of_birth: Date,
        licences: Vec<Licence>,
        minimum_ages: &MinimumAges,
    ) -> Result<Self, ValidationError> {
        let full_name = full_name.into().trim().to_owned();
        check_length("full_name", &full_name, 1, 100)?;
        let mut seen = HashSet::new();
        let repeated = licences.iter().find(|licence| {
            !seen.insert((
                licence.issuing_state(),
                licence.issuing_authority(),
                licence.number(),
            ))
        });
        if let Some(licence) = repeated {
            return Err(ValidationError::RepeatedLicence {
                issuing_state: licence.issuing_state().to_owned(),
                issuing_authority: licence.issuing_authority().to_owned(),
                number: licence.number().to_owned(),
            });
        }
        for licence in &licences {
            for issued in licence.titles() {
                let minimum_age = minimum_ages.for_title(issued.title);
                let age = minimum_ages::age_on(date_of_birth, issued.date_of_first_issue);
                if age < i32::from(minimum_age) {
                    return Err(ValidationError::UnderMinimumAge {
                        issuing_state: licence.issuing_state().to_owned(),
                        issuing_authority: licence.issuing_authority().to_owned(),
                        number: licence.number().to_owned(),
                        title: issued.title,
                        minimum_age,
                        date_of_first_issue: issued.date_of_first_issue,
                    });
                }
            }
        }
        Ok(Self {
            id,
            full_name,
            date_of_birth,
            licences,
        })
    }

    /// The name of the holder in full (Annex 1, §5.2.1 IV; Appendix 4 IVb).
    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    /// Annex 1, §5.2.1 IVa; Appendix 4 IVc.
    pub fn date_of_birth(&self) -> Date {
        self.date_of_birth
    }

    /// The licences the pilot holds, no two with the same identity.
    pub fn licences(&self) -> &[Licence] {
        &self.licences
    }

    /// Read a pilot from any serde deserializer, checking each title's date of
    /// first issue against a State's own minimum ages, as
    /// [`Pilot::with_minimum_ages`] does. The wire shape is plain
    /// `Deserialize`'s, and so is the error: a serde error carrying the
    /// [`ValidationError`]'s message.
    ///
    /// ```
    /// use icao_shared_kernel::{MinimumAges, Pilot};
    ///
    /// let ages = MinimumAges { private: 16, ..MinimumAges::default() };
    /// let json = r#"{
    ///     "full_name": "Jane Citizen",
    ///     "date_of_birth": "2000-01-01",
    ///     "licences": [{
    ///         "issuing_state": "AU", "issuing_authority": "CASA", "number": "12345",
    ///         "titles": [{"licence": "private", "category": "aeroplane",
    ///                     "date_of_first_issue": "2016-06-01"}]
    ///     }]
    /// }"#;
    /// let mut deserializer = serde_json::Deserializer::from_str(json);
    /// assert!(Pilot::deserialize_with(&mut deserializer, &ages).is_ok());
    /// // As `serde_json::from_str` does, refuse anything after the pilot.
    /// deserializer.end().unwrap();
    /// assert!(serde_json::from_str::<Pilot>(json).is_err());
    /// ```
    pub fn deserialize_with<'de, D: Deserializer<'de>>(
        deserializer: D,
        minimum_ages: &MinimumAges,
    ) -> Result<Self, D::Error> {
        PilotData::deserialize(deserializer)?
            .into_pilot(minimum_ages)
            .map_err(de::Error::custom)
    }
}

// Unchecked mirror of the wire shape, for the same reason as the value-object
// shadows: serde deserialises into `PilotData`, then `try_from` funnels it
// through `with` so a deserialised pilot is validated exactly like a hand-built
// one. It also supplies the `id` default factory when the field is absent.
#[derive(Serialize, Deserialize)]
struct PilotData {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    full_name: String,
    #[serde(with = "crate::iso_date")]
    date_of_birth: Date,
    #[serde(default)]
    licences: Vec<Licence>,
}

impl PilotData {
    fn into_pilot(self, minimum_ages: &MinimumAges) -> Result<Pilot, ValidationError> {
        Pilot::with_minimum_ages(
            self.id,
            self.full_name,
            self.date_of_birth,
            self.licences,
            minimum_ages,
        )
    }
}

impl TryFrom<PilotData> for Pilot {
    type Error = ValidationError;

    fn try_from(data: PilotData) -> Result<Self, Self::Error> {
        data.into_pilot(&MinimumAges::default())
    }
}

impl From<Pilot> for PilotData {
    fn from(value: Pilot) -> Self {
        Self {
            id: value.id,
            full_name: value.full_name,
            date_of_birth: value.date_of_birth,
            licences: value.licences,
        }
    }
}
