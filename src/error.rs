//! The single domain error type shared by every value-object constructor.
//!
//! This is the Rust analogue of the typed `variant` the Python kernel raises
//! via Pydantic — one variant per rule, never a stringly-typed error. The
//! waypoint messages match the Python `validate_waypoint_code` strings exactly
//! (that validator is our own code, asserted on in the pytest suite); the other
//! variants use their own clear messages, since the equivalent Python messages
//! are Pydantic framework strings and matching them byte-for-byte would be
//! brittle. The parity bar for those is *behavioural*: reject the same inputs.

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
