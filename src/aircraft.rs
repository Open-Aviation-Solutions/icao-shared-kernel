//! The `Aircraft` aggregate root and its value objects.

mod registration;

pub use registration::{AircraftRegistration, AircraftType};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::DomainError;

/// Aircraft reference entity with ICAO-universal identity and descriptors.
///
/// Carries only ICAO-universal facts: the surrogate id, the Doc 8643 type
/// designator, and the current Annex 7 registration marks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Aircraft {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub aircraft_type: AircraftType,
    pub registration: AircraftRegistration,
}

impl Aircraft {
    /// Build an aircraft from already-validated parts, assigning a fresh id.
    pub fn new(aircraft_type: AircraftType, registration: AircraftRegistration) -> Self {
        Self {
            id: Uuid::new_v4(),
            aircraft_type,
            registration,
        }
    }

    /// Convenience constructor from raw strings, validating each part and
    /// assigning a fresh id.
    pub fn create(
        type_designator: &str,
        nationality: &str,
        registration: &str,
    ) -> Result<Self, DomainError> {
        Ok(Self::new(
            AircraftType::parse(type_designator)?,
            AircraftRegistration::new(nationality, registration)?,
        ))
    }
}

impl std::fmt::Display for Aircraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.aircraft_type, self.registration)
    }
}
