//! The `Flight` aggregate root — the Shared Kernel thin hub for the physical
//! flight event — and its flight-scoped value objects.

pub mod coordinate;
pub mod flight_duration;
pub mod flight_rules;
pub mod significant_point;
pub mod waypoint;

use serde::{Deserialize, Serialize};
use time::UtcDateTime;
use uuid::Uuid;

use crate::error::ValidationError;
use flight_duration::FlightDuration;
use flight_rules::FlightRules;
use significant_point::SignificantPoint;

/// Shared record identifying a physical flight event.
///
/// A thin hub: it carries only fields universally relevant to every consuming
/// domain — which aircraft flew, from where to where, and when the flight
/// began and ended. Consumers reference a `Flight` by [`Uuid`] from their own
/// aggregates; they do not subclass, embed, or extend it.
///
/// `first_movement` and `last_movement` are the start and the end of the
/// flight as the applicable flight-time definition measures it: the one in the
/// rules the flight is recorded under. They are *not* when the aircraft first
/// and last moved under its own power, which such a definition need not turn
/// on. CASR 1998 reg 61.010's "duration, of a flight", for one, runs for an
/// aeroplane from when it "begins moving, whether or not under its own power,
/// in preparation for flight", for a helicopter from when its rotor blades
/// start turning until they stop, and for a glider from when it begins moving
/// "whether being towed or not". Which definition applies, and so what it says
/// for each kind of aircraft, is for a national layer: this crate holds the
/// two instants and does not check them against one.
///
/// Both are optional because the route (the departure and arrival points) is
/// often known before the times are.
/// `flight_rules` is optional because a record may not say: it is never
/// inferred from anything else. The constructors leave it unrecorded; set it
/// with [`with_flight_rules`](Self::with_flight_rules).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flight {
    pub id: Uuid,
    pub aircraft_id: Uuid,
    pub departure: SignificantPoint,
    pub arrival: SignificantPoint,
    /// The start of the flight, as the applicable flight-time definition
    /// measures it, if recorded.
    pub first_movement: Option<UtcDateTime>,
    /// The end of the flight, as the applicable flight-time definition
    /// measures it, if recorded.
    pub last_movement: Option<UtcDateTime>,
    /// The flight rules it was flown under, if recorded.
    pub flight_rules: Option<FlightRules>,
}

impl Flight {
    /// Build a flight from already-validated parts, assigning a fresh id.
    pub fn new(
        aircraft_id: Uuid,
        departure: SignificantPoint,
        arrival: SignificantPoint,
        first_movement: Option<UtcDateTime>,
        last_movement: Option<UtcDateTime>,
    ) -> Self {
        Self::with(
            Uuid::new_v4(),
            aircraft_id,
            departure,
            arrival,
            first_movement,
            last_movement,
        )
    }

    /// Build a flight from already-validated parts with an explicit id — for
    /// rehydrating a previously persisted flight, where [`new`](Self::new)
    /// would mint a different id than the one it was stored under.
    #[allow(clippy::too_many_arguments)]
    pub fn with(
        id: Uuid,
        aircraft_id: Uuid,
        departure: SignificantPoint,
        arrival: SignificantPoint,
        first_movement: Option<UtcDateTime>,
        last_movement: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            aircraft_id,
            departure,
            arrival,
            first_movement,
            last_movement,
            flight_rules: None,
        }
    }

    /// The same flight, flown under `flight_rules` (`None`: not recorded).
    pub fn with_flight_rules(self, flight_rules: Option<FlightRules>) -> Self {
        Self {
            flight_rules,
            ..self
        }
    }

    /// Convenience constructor from coded designator strings, validating them
    /// and assigning a fresh id with no movement times recorded.
    pub fn create(
        aircraft_id: Uuid,
        departure: &str,
        arrival: &str,
    ) -> Result<Self, ValidationError> {
        Ok(Self::new(
            aircraft_id,
            SignificantPoint::designator(departure)?,
            SignificantPoint::designator(arrival)?,
            None,
            None,
        ))
    }

    /// Canonical short route string, e.g. `YSBK-YSCN`.
    pub fn route_summary(&self) -> String {
        format!("{}-{}", self.departure, self.arrival)
    }

    /// Block time, derived from the movement timestamps — `Some` only when both
    /// are recorded. Rounded down to whole minutes.
    pub fn duration(&self) -> Option<FlightDuration> {
        match (self.first_movement, self.last_movement) {
            (Some(first), Some(last)) => Some(FlightDuration::new((last - first).whole_minutes())),
            _ => None,
        }
    }
}

impl std::fmt::Display for Flight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date_fmt = time::macros::format_description!("[year]-[month]-[day]");
        match self.first_movement {
            Some(first) => {
                let date = first
                    .date()
                    .format(&date_fmt)
                    .map_err(|_| std::fmt::Error)?;
                write!(f, "Flight {}: {}", date, self.route_summary())
            }
            None => write!(f, "Flight: {}", self.route_summary()),
        }
    }
}
