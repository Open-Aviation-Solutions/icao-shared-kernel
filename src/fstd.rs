//! The `FlightSimulationTrainingDevice` aggregate root and its value objects.

mod designation;
mod qualification;

pub use designation::DeviceDesignation;
pub use qualification::DeviceQualification;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ValidationError;

/// The kind of apparatus a flight simulation training device is.
///
/// ICAO Annex 1 (Personnel Licensing), Chapter 1, defines a flight simulation
/// training device as "any one of the following three types of apparatus in
/// which flight conditions are simulated on the ground", and these are those
/// three.
///
/// National taxonomies diverge from this and must map onto it rather than
/// replace it — CASR 1998's equivalent triple, for instance, names its middle
/// kind a *flight training device* rather than a flight procedures trainer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FstdKind {
    /// Provides an accurate representation of the flight deck of a particular
    /// aircraft *type*, realistically simulating its systems, the normal
    /// environment of flight crew members, and its performance and flight
    /// characteristics.
    FlightSimulator,
    /// Provides a realistic flight deck environment, simulating instrument
    /// responses, simple system control functions, and the performance and
    /// flight characteristics of aircraft of a particular *class*.
    FlightProceduresTrainer,
    /// Equipped with appropriate instruments, simulating the flight deck
    /// environment of an aircraft in flight in instrument flight conditions.
    BasicInstrumentFlightTrainer,
}

/// A ground-based apparatus in which flight conditions are simulated (ICAO
/// Annex 1, Chapter 1).
///
/// An aggregate root, referenced by [`Uuid`] from
/// [`FstdSession`](crate::FstdSession) and from consumers' own aggregates. It
/// is deliberately *not* an [`Aircraft`](crate::Aircraft): a device has no Doc
/// 8643 type designator and no Annex 7 registration marks, and modelling one
/// as an aircraft was what the earlier Python kernel got wrong.
///
/// The ICAO term is generic — approval is a separate matter for each State's
/// licensing authority. So `qualification` is optional, and `None` is a
/// first-class case rather than missing data: an unqualified personal device
/// is perfectly loggable, it simply earns no regulatory credit. Some national
/// definitions are *not* generic in this way (CASR 1998 reg 61.010's "flight
/// simulation training device" means specifically a *recognised* device), so a
/// national layer may need to narrow this.
///
/// Only the device's *current* qualification is held here — a consumer needing
/// the qualification that applied to a past session snapshots it onto its own
/// record at the time.
///
/// Which aircraft a session simulated is recorded on the *session*, not here:
/// see [`FstdSession::simulated_aircraft_type`](crate::FstdSession).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlightSimulationTrainingDevice {
    pub id: Uuid,
    pub kind: FstdKind,
    pub designation: DeviceDesignation,
    pub qualification: Option<DeviceQualification>,
}

impl FlightSimulationTrainingDevice {
    /// Build a device from already-validated parts, assigning a fresh id.
    pub fn new(
        kind: FstdKind,
        designation: DeviceDesignation,
        qualification: Option<DeviceQualification>,
    ) -> Self {
        Self::with(Uuid::new_v4(), kind, designation, qualification)
    }

    /// Build a device from already-validated parts with an explicit id — for
    /// rehydrating a previously persisted device, where [`new`](Self::new)
    /// would mint a different id than the one it was stored under.
    pub fn with(
        id: Uuid,
        kind: FstdKind,
        designation: DeviceDesignation,
        qualification: Option<DeviceQualification>,
    ) -> Self {
        Self {
            id,
            kind,
            designation,
            qualification,
        }
    }

    /// Convenience constructor for an unqualified device, validating the
    /// designation and assigning a fresh id.
    pub fn create(kind: FstdKind, designation: &str) -> Result<Self, ValidationError> {
        Ok(Self::new(
            kind,
            DeviceDesignation::parse(designation)?,
            None,
        ))
    }
}

impl std::fmt::Display for FlightSimulationTrainingDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.designation.fmt(f)
    }
}
