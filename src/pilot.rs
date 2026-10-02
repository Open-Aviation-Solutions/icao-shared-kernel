//! The `Pilot` aggregate root and the licences it holds.

mod licence;

pub use licence::Licence;

use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use crate::error::{check_length, ValidationError};

/// Pilot identity and the licences they hold.
///
/// The full name and date of birth are those every licence carries (ICAO Annex
/// 1, §5.2.1 IV and IVa), and that a logbook records (CASA reg 61.345(2)). They
/// belong to the pilot, not to a licence: a pilot holds them before holding any.
///
/// A pilot can hold licences from multiple States simultaneously (ICAO Annex 1
/// imposes no single-State restriction).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "PilotData", into = "PilotData")]
pub struct Pilot {
    pub id: Uuid,
    full_name: String,
    date_of_birth: Date,
    pub licences: Vec<Licence>,
}

impl Pilot {
    /// Build a pilot with a fresh id and no licences.
    pub fn new(full_name: impl Into<String>, date_of_birth: Date) -> Result<Self, ValidationError> {
        Self::with(Uuid::new_v4(), full_name, date_of_birth, Vec::new())
    }

    /// Build a pilot from explicit parts, validating the full name's length.
    ///
    /// The date of birth is not checked against today: the kernel has no
    /// clock, so that is for a consumer that has one.
    pub fn with(
        id: Uuid,
        full_name: impl Into<String>,
        date_of_birth: Date,
        licences: Vec<Licence>,
    ) -> Result<Self, ValidationError> {
        let full_name = full_name.into();
        check_length("full_name", &full_name, 1, 100)?;
        Ok(Self {
            id,
            full_name,
            date_of_birth,
            licences,
        })
    }

    /// The name of the holder in full (Annex 1, §5.2.1 IV).
    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    /// Annex 1, §5.2.1 IVa.
    pub fn date_of_birth(&self) -> Date {
        self.date_of_birth
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
    date_of_birth: Date,
    #[serde(default)]
    licences: Vec<Licence>,
}

impl TryFrom<PilotData> for Pilot {
    type Error = ValidationError;

    fn try_from(data: PilotData) -> Result<Self, Self::Error> {
        Self::with(data.id, data.full_name, data.date_of_birth, data.licences)
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
