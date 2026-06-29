//! The `Flight` aggregate root — the Shared Kernel thin hub for the physical
//! flight event — and its flight-scoped value objects.

pub mod flight_time;
pub mod waypoint;

use serde::{Deserialize, Serialize};
use time::{Date, Time};
use uuid::Uuid;

use crate::error::DomainError;
use waypoint::Waypoint;

// Serialise dates/times in the ISO-like shapes Pydantic emits, so the JSON wire
// form stays close to the Python kernel. The exact UUID/date representation is
// an open question in task 0014; these are the pragmatic defaults for the spike.
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");
time::serde::format_description!(time_format, Time, "[hour]:[minute]:[second]");

/// Shared record identifying a physical flight event.
///
/// A thin hub: it carries only fields universally relevant to every consuming
/// domain. Consumers reference a `Flight` by [`Uuid`] from their own
/// aggregates; they do not subclass, embed, or extend it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flight {
    pub id: Uuid,
    pub aircraft_id: Uuid,
    #[serde(with = "date_format")]
    pub flight_date: Date,
    #[serde(
        with = "time_format::option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flight_time: Option<Time>,
    pub start_waypoint: Waypoint,
    pub end_waypoint: Waypoint,
}

impl Flight {
    /// Build a flight from already-validated parts, assigning a fresh id.
    pub fn new(
        aircraft_id: Uuid,
        flight_date: Date,
        flight_time: Option<Time>,
        start_waypoint: Waypoint,
        end_waypoint: Waypoint,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            aircraft_id,
            flight_date,
            flight_time,
            start_waypoint,
            end_waypoint,
        }
    }

    /// Convenience constructor from raw waypoint strings, validating them and
    /// assigning a fresh id with no flight time.
    pub fn create(
        aircraft_id: Uuid,
        flight_date: Date,
        start_waypoint: &str,
        end_waypoint: &str,
    ) -> Result<Self, DomainError> {
        Ok(Self::new(
            aircraft_id,
            flight_date,
            None,
            Waypoint::parse(start_waypoint)?,
            Waypoint::parse(end_waypoint)?,
        ))
    }

    /// Canonical short route string, e.g. `YSBK-YSCN`.
    pub fn route_summary(&self) -> String {
        format!("{}-{}", self.start_waypoint, self.end_waypoint)
    }
}

impl std::fmt::Display for Flight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = time::macros::format_description!("[year]-[month]-[day]");
        let date_str = self.flight_date.format(&fmt).map_err(|_| std::fmt::Error)?;
        write!(f, "Flight {}: {}", date_str, self.route_summary())
    }
}
