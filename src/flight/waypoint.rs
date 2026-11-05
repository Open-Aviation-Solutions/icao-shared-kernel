// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Waypoint designator value object.

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

/// A validated waypoint designator (e.g. `YSBK`, `RIVET`, `DCT01`).
///
/// Constructed only through [`Waypoint::parse`], so an existing `Waypoint` is
/// always valid. Serialises transparently as its string form.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Waypoint(String);

impl Waypoint {
    /// Parse and validate a waypoint designator: uppercase alphanumeric,
    /// length 2–5.
    ///
    /// Every character must be ASCII `A`–`Z` or `0`–`9`, and there must be at
    /// least one letter (an all-digit code is rejected). Length is checked
    /// first, so the error distinguishes the two failure modes.
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        let len = value.chars().count();
        if !(2..=5).contains(&len) {
            return Err(ValidationError::WaypointLength(value));
        }
        let all_upper_alnum = value
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
        let has_letter = value.chars().any(|c| c.is_ascii_uppercase());
        if !all_upper_alnum || !has_letter {
            return Err(ValidationError::WaypointCharset(value));
        }
        Ok(Self(value))
    }

    /// Borrow the underlying designator string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Waypoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for Waypoint {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<Waypoint> for String {
    fn from(waypoint: Waypoint) -> Self {
        waypoint.0
    }
}
