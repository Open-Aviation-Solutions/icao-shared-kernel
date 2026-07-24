//! The single domain error type shared by every value-object constructor.
//!
//! One typed variant per validation rule, never a stringly-typed error. The
//! waypoint variants carry the domain's canonical, user-facing messages; the
//! length-bound variant is fully structured (`field`, `min`, `max`, `actual`)
//! so callers can render or match on it however they need.

use rust_decimal::Decimal;
use thiserror::Error;

/// Every way a domain value object can fail to validate.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidationError {
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

    /// A required string field that was empty.
    #[error("{field} must not be empty")]
    Empty { field: &'static str },

    /// An issuing state that is not a two-letter ISO 3166-1 alpha-2 code.
    #[error("issuing_state '{0}' must be a two-letter ISO 3166-1 alpha-2 code")]
    IssuingState(String),

    /// A latitude outside the valid −90..=90 degree range.
    #[error("latitude {0} is out of range (must be -90..=90 degrees)")]
    Latitude(Decimal),

    /// A longitude outside the valid −180..=180 degree range.
    #[error("longitude {0} is out of range (must be -180..=180 degrees)")]
    Longitude(Decimal),
}

/// Enforce an inclusive `[min, max]` length bound on a string field.
///
/// Length is counted in Unicode scalar values, matching Python's `len(str)`.
pub(crate) fn check_length(
    field: &'static str,
    value: &str,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
    let actual = value.chars().count();
    if actual < min || actual > max {
        return Err(ValidationError::FieldLength {
            field,
            min,
            max,
            actual,
        });
    }
    Ok(())
}

/// Require a string field to be non-empty, with no upper bound on its length.
pub(crate) fn check_non_empty(field: &'static str, value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::Empty { field });
    }
    Ok(())
}

/// Enforce the valid latitude (−90..=90) and longitude (−180..=180) ranges.
pub(crate) fn check_coordinate_ranges(
    latitude: Decimal,
    longitude: Decimal,
) -> Result<(), ValidationError> {
    if latitude < Decimal::from(-90) || latitude > Decimal::from(90) {
        return Err(ValidationError::Latitude(latitude));
    }
    if longitude < Decimal::from(-180) || longitude > Decimal::from(180) {
        return Err(ValidationError::Longitude(longitude));
    }
    Ok(())
}
