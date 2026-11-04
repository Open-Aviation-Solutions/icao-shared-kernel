// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Flight duration value object — whole minutes with formatting helpers.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

/// A flight duration in whole minutes, never negative. Serialises
/// transparently as a plain integer, and deserialising refuses a negative one.
///
/// Never negative because Annex 1 (Chapter 1, the definition of "flight
/// time") measures it from one moment, when the aircraft first moves for the
/// purpose of taking off (or, for the other categories, the moment their
/// definition names), to a later one, when it comes to rest at the end of the
/// flight: a span from an earlier moment to a later one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "i64")]
#[serde(into = "i64")]
pub struct FlightDuration(i64);

impl FlightDuration {
    /// Wrap a total number of minutes, refusing a negative one.
    pub const fn new(total_minutes: i64) -> Result<Self, ValidationError> {
        if total_minutes < 0 {
            return Err(ValidationError::FlightDuration(total_minutes));
        }
        Ok(Self(total_minutes))
    }

    /// Build a duration from whole hours and minutes (e.g. `1h 30m` → 90),
    /// refusing a negative total or one too large for `i64`. The components may
    /// be outside 0..60 for minutes or have mixed signs: only the total is
    /// checked, so `(2, -90)` is 30 minutes.
    pub const fn from_hours_minutes(hours: i64, minutes: i64) -> Result<Self, ValidationError> {
        let total = match hours.checked_mul(60) {
            Some(hour_minutes) => hour_minutes.checked_add(minutes),
            None => None,
        };
        match total {
            Some(total_minutes) => Self::new(total_minutes),
            None => Err(ValidationError::FlightDurationOverflow { hours, minutes }),
        }
    }

    /// The total duration in minutes.
    pub const fn total_minutes(self) -> i64 {
        self.0
    }

    /// Whole hours component (`total_minutes / 60`).
    pub const fn hours(self) -> i64 {
        self.0 / 60
    }

    /// Leftover minutes component (`total_minutes % 60`).
    pub const fn minutes(self) -> i64 {
        self.0 % 60
    }

    /// The duration as a decimal number of hours (exact, e.g. 90 → `1.5`).
    pub fn decimal_hours(self) -> Decimal {
        Decimal::from(self.0) / Decimal::from(60)
    }
}

impl std::fmt::Display for FlightDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:02}", self.hours(), self.minutes())
    }
}

impl TryFrom<i64> for FlightDuration {
    type Error = ValidationError;

    fn try_from(total_minutes: i64) -> Result<Self, Self::Error> {
        Self::new(total_minutes)
    }
}

impl From<FlightDuration> for i64 {
    fn from(duration: FlightDuration) -> Self {
        duration.0
    }
}
