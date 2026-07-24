//! Flight duration value object — whole minutes with formatting helpers.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A flight duration in whole minutes. Serialises transparently as a plain
/// integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FlightDuration(i64);

impl FlightDuration {
    /// Wrap a total number of minutes.
    pub const fn new(total_minutes: i64) -> Self {
        Self(total_minutes)
    }

    /// Build a duration from whole hours and minutes (e.g. `1h 30m` → 90).
    pub const fn from_hours_minutes(hours: i64, minutes: i64) -> Self {
        Self(hours * 60 + minutes)
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

impl From<i64> for FlightDuration {
    fn from(total_minutes: i64) -> Self {
        Self(total_minutes)
    }
}
