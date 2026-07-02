//! The `Pilot` aggregate root and the licences it holds.

mod licence;

pub use licence::Licence;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{check_length, DomainError};

/// Pilot identity and the licences they hold.
///
/// A pilot can hold licences from multiple States simultaneously (ICAO Annex 1
/// imposes no single-State restriction).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "PilotData", into = "PilotData")]
pub struct Pilot {
    pub id: Uuid,
    display_name: String,
    legal_name: Option<String>,
    pub licences: Vec<Licence>,
}

impl Pilot {
    /// Build a pilot with a fresh id, no legal name, and no licences.
    pub fn new(display_name: impl Into<String>) -> Result<Self, DomainError> {
        Self::with(Uuid::new_v4(), display_name, None, Vec::new())
    }

    /// Build a pilot from explicit parts, validating name lengths.
    pub fn with(
        id: Uuid,
        display_name: impl Into<String>,
        legal_name: Option<String>,
        licences: Vec<Licence>,
    ) -> Result<Self, DomainError> {
        let display_name = display_name.into();
        check_length("display_name", &display_name, 1, 100)?;
        if let Some(ref legal_name) = legal_name {
            check_length("legal_name", legal_name, 0, 100)?;
        }
        Ok(Self {
            id,
            display_name,
            legal_name,
            licences,
        })
    }

    /// Name as the pilot prefers to be addressed.
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Full legal name, if recorded and different from the display name.
    pub fn legal_name(&self) -> Option<&str> {
        self.legal_name.as_deref()
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
    display_name: String,
    #[serde(default)]
    legal_name: Option<String>,
    #[serde(default)]
    licences: Vec<Licence>,
}

impl TryFrom<PilotData> for Pilot {
    type Error = DomainError;

    fn try_from(data: PilotData) -> Result<Self, Self::Error> {
        Self::with(data.id, data.display_name, data.legal_name, data.licences)
    }
}

impl From<Pilot> for PilotData {
    fn from(value: Pilot) -> Self {
        Self {
            id: value.id,
            display_name: value.display_name,
            legal_name: value.legal_name,
            licences: value.licences,
        }
    }
}
