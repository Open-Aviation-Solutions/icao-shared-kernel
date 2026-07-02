//! The single domain error type shared by every value-object constructor.
//!
//! One typed variant per validation rule, never a stringly-typed error. The
//! waypoint variants carry the domain's canonical, user-facing messages; the
//! length-bound variant is fully structured (`field`, `min`, `max`, `actual`)
//! so callers can render or match on it however they need.

use thiserror::Error;

/// Every way a domain value object can fail to validate.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    /// Waypoint designator outside the 2–5 character range.
    #[error("Waypoint '{0}' must be 2-5 characters long")]
    WaypointLength(String),

    /// Waypoint designator that is not uppercase alphanumeric (or has no letter).
    #[error("Waypoint '{0}' must be uppercase alphanumeric")]
    WaypointCharset(String),

    /// Aircraft type that does not match the ICAO Doc 8643 designator shape.
    #[error(
        "Aircraft type '{0}' must be 2-4 characters: a leading letter \
         followed by letters or digits"
    )]
    AircraftType(String),

    /// A string field whose length falls outside its `[min, max]` bound.
    #[error("{field} must be {min}-{max} characters, got {actual}")]
    FieldLength {
        field: &'static str,
        min: usize,
        max: usize,
        actual: usize,
    },
}

/// Enforce an inclusive `[min, max]` length bound on a string field.
///
/// Length is counted in Unicode scalar values, matching Python's `len(str)`.
pub(crate) fn check_length(
    field: &'static str,
    value: &str,
    min: usize,
    max: usize,
) -> Result<(), DomainError> {
    let actual = value.chars().count();
    if actual < min || actual > max {
        return Err(DomainError::FieldLength {
            field,
            min,
            max,
            actual,
        });
    }
    Ok(())
}
