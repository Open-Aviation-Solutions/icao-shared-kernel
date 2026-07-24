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
    /// Parse and validate a waypoint designator.
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        validate_waypoint_code(&value)?;
        Ok(Self(value))
    }

    /// Borrow the underlying designator string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validate a single waypoint designator: uppercase alphanumeric, length 2–5.
///
/// Public so consumer domains that store their own waypoint lists can call it
/// and stay in lockstep with the kernel.
///
/// Every character must be ASCII `A`–`Z` or `0`–`9`, and there must be at least
/// one letter (an all-digit code is rejected). Length is checked first, so the
/// error distinguishes the two failure modes.
pub fn validate_waypoint_code(waypoint: &str) -> Result<(), ValidationError> {
    let len = waypoint.chars().count();
    if !(2..=5).contains(&len) {
        return Err(ValidationError::WaypointLength(waypoint.to_string()));
    }
    let all_upper_alnum = waypoint
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
    let has_letter = waypoint.chars().any(|c| c.is_ascii_uppercase());
    if !all_upper_alnum || !has_letter {
        return Err(ValidationError::WaypointCharset(waypoint.to_string()));
    }
    Ok(())
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
