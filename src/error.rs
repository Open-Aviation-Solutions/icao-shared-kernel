// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The single domain error type shared by every value-object constructor.
//!
//! One typed variant per validation rule, never a stringly-typed error. The
//! waypoint variants carry the domain's canonical, user-facing messages; the
//! length-bound variant is fully structured (`field`, `min`, `max`, `actual`)
//! so callers can render or match on it however they need.

use rust_decimal::Decimal;
use thiserror::Error;
use time::Date;

use crate::pilot::{AircraftCategory, LicenceTitle};

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
    #[error("issuing State '{0}' must be a two-letter ISO 3166-1 alpha-2 code")]
    IssuingState(String),

    /// A location indicator that is not four uppercase letters, the shape of
    /// ICAO Doc 7910's.
    #[error("location indicator '{0}' must be four uppercase letters")]
    LocationIndicator(String),

    /// An unlisted aerodrome with neither a name nor a position: Doc 4444,
    /// Appendix 2, Item 18 (`DEP/`, `DEST/`) describes one by either.
    #[error("an unlisted aerodrome must have a name or a position")]
    UnlistedAerodrome,

    /// A title of licence and a category of aircraft that Annex 1 does not
    /// issue together (Annex 1, 2.3.1, 2.4.1, 2.5, 2.6.3 to 2.6.5, 2.9 and
    /// 2.10). The licence is its name: without its category it is no value
    /// of the public API, as a `LicenceTitle` always holds one.
    #[error("the {licence} is not issued for the {category} category")]
    LicenceCategory {
        licence: &'static str,
        category: AircraftCategory,
    },

    /// The same title of licence listed twice under one licence: each is one
    /// licence (Annex 1, 5.2.1 II, with its category under 2.1.1.2), with one
    /// date of first issue.
    #[error("{0} is listed more than once")]
    RepeatedLicenceTitle(LicenceTitle),

    /// The same licence, by its identity (Annex 1, 5.2.1 I, III and VIII: its
    /// State, serial number and issuing authority), held twice by one pilot.
    #[error("licence {number} ({issuing_authority}, {issuing_state}) is listed more than once")]
    RepeatedLicence {
        issuing_state: String,
        issuing_authority: String,
        number: String,
    },

    /// A title of licence first issued before the holder reached its minimum
    /// age: Annex 1's (2.3.1.1, 2.4.1.1, 2.5.1.1, 2.6.1.1, 2.9.1.1 and
    /// 2.10.1.1), or a State's own (`MinimumAges`).
    #[error(
        "{title} under licence {number} ({issuing_authority}, {issuing_state}) was first issued \
         on {date_of_first_issue}, before the holder was {minimum_age}"
    )]
    UnderMinimumAge {
        issuing_state: String,
        issuing_authority: String,
        number: String,
        title: LicenceTitle,
        minimum_age: u8,
        date_of_first_issue: Date,
    },

    /// A validity period whose end date precedes its start date.
    #[error("valid_until {until} must not precede valid_from {from}")]
    ValidityPeriod { from: Date, until: Date },

    /// A flight duration below zero minutes. Annex 1's definition of "flight
    /// time" (Chapter 1) runs from the moment the aircraft first moves to the
    /// moment it comes to rest, so it is never negative.
    #[error("a flight duration must not be negative, got {0} minutes")]
    FlightDuration(i64),

    /// Hours and minutes whose total, in minutes, does not fit in an `i64`.
    #[error("{hours} hours and {minutes} minutes are too many minutes for a flight duration")]
    FlightDurationOverflow { hours: i64, minutes: i64 },

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

/// Whether `value` is exactly `len` ASCII uppercase letters: the shape of a
/// code such as an issuing State or a location indicator.
pub(crate) fn is_ascii_uppercase_of_len(value: &str, len: usize) -> bool {
    value.len() == len && value.bytes().all(|b| b.is_ascii_uppercase())
}

/// Require a string field to be non-empty, with no upper bound on its length.
pub(crate) fn check_non_empty(field: &'static str, value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::Empty { field });
    }
    Ok(())
}

/// Require a validity period's end date not to precede its start date.
pub(crate) fn check_validity_period(from: Date, until: Date) -> Result<(), ValidationError> {
    if until < from {
        return Err(ValidationError::ValidityPeriod { from, until });
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
