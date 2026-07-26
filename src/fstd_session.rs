//! The `FstdSession` aggregate root — a training session flown on a flight
//! simulation training device.

use serde::{Deserialize, Serialize};
use time::UtcDateTime;
use uuid::Uuid;

use crate::aircraft::AircraftType;
use crate::error::ValidationError;
use crate::flight::flight_duration::FlightDuration;
use crate::flight::significant_point::SignificantPoint;

/// Shared record identifying a session flown on a
/// [`FlightSimulationTrainingDevice`](crate::FlightSimulationTrainingDevice).
///
/// The sibling of [`Flight`](crate::Flight), not a variant of it: a `Flight` is
/// the authoritative record of a *physical* flight event, and a simulator
/// session is not one. Both are thin hubs referenced by [`Uuid`] from
/// consumers' own aggregates.
///
/// Every field but the device reference is optional, because a session may be
/// as little as an hour of manoeuvre practice representing no particular
/// aircraft and flying no particular route.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FstdSession {
    pub id: Uuid,
    pub device_id: Uuid,
    /// The aircraft type the session simulated.
    ///
    /// On the session rather than on the device, because the two cases differ:
    /// a *qualified* device is bound to one simulated aircraft by its
    /// qualification certificate (CASR 60.035(2)(b) requires the certificate to
    /// "specify the aircraft that is simulated"), but an *unqualified* device —
    /// a personal simulator — is routinely flown as a different type from one
    /// session to the next. Recording it per session serves both with one
    /// field.
    ///
    /// `None` when the device represented no specific type, or when the type
    /// flown is not known.
    pub simulated_aircraft_type: Option<AircraftType>,
    /// The simulated departure point, if the session flew a route.
    pub departure: Option<SignificantPoint>,
    /// The simulated arrival point, if the session flew a route.
    pub arrival: Option<SignificantPoint>,
    pub start: Option<UtcDateTime>,
    pub end: Option<UtcDateTime>,
}

impl FstdSession {
    /// Build a session from already-validated parts, assigning a fresh id.
    pub fn new(
        device_id: Uuid,
        simulated_aircraft_type: Option<AircraftType>,
        departure: Option<SignificantPoint>,
        arrival: Option<SignificantPoint>,
        start: Option<UtcDateTime>,
        end: Option<UtcDateTime>,
    ) -> Self {
        Self::with(
            Uuid::new_v4(),
            device_id,
            simulated_aircraft_type,
            departure,
            arrival,
            start,
            end,
        )
    }

    /// Build a session from already-validated parts with an explicit id — for
    /// rehydrating a previously persisted session, where [`new`](Self::new)
    /// would mint a different id than the one it was stored under.
    #[allow(clippy::too_many_arguments)]
    pub fn with(
        id: Uuid,
        device_id: Uuid,
        simulated_aircraft_type: Option<AircraftType>,
        departure: Option<SignificantPoint>,
        arrival: Option<SignificantPoint>,
        start: Option<UtcDateTime>,
        end: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            device_id,
            simulated_aircraft_type,
            departure,
            arrival,
            start,
            end,
        }
    }

    /// Convenience constructor for a session with no route and no times
    /// recorded, validating the simulated type designator.
    pub fn create(device_id: Uuid, simulated_type: &str) -> Result<Self, ValidationError> {
        Ok(Self::new(
            device_id,
            Some(AircraftType::parse(simulated_type)?),
            None,
            None,
            None,
            None,
        ))
    }

    /// Canonical short route string, e.g. `YSBK-YSCN` — `Some` only when both
    /// endpoints were recorded.
    pub fn route_summary(&self) -> Option<String> {
        match (&self.departure, &self.arrival) {
            (Some(departure), Some(arrival)) => Some(format!("{departure}-{arrival}")),
            _ => None,
        }
    }

    /// Session time, derived from the timestamps — `Some` only when both are
    /// recorded. Rounded down to whole minutes, matching
    /// [`Flight::duration`](crate::Flight::duration).
    pub fn duration(&self) -> Option<FlightDuration> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => Some(FlightDuration::new((end - start).whole_minutes())),
            _ => None,
        }
    }
}

impl std::fmt::Display for FstdSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.simulated_aircraft_type, self.route_summary()) {
            (Some(aircraft_type), Some(route)) => {
                write!(f, "FSTD session ({aircraft_type}): {route}")
            }
            (Some(aircraft_type), None) => write!(f, "FSTD session ({aircraft_type})"),
            (None, Some(route)) => write!(f, "FSTD session: {route}"),
            (None, None) => write!(f, "FSTD session"),
        }
    }
}
