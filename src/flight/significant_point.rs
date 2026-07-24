//! The `SignificantPoint` value object — an ICAO route/flight-path location.

use serde::{Deserialize, Serialize};

use super::coordinate::Coordinate;
use super::waypoint::Waypoint;
use crate::error::ValidationError;

/// A specified geographical location used to define an ATS route or an
/// aircraft's flight path (ICAO "significant point").
///
/// Expressed either as a coded designator (a [`Waypoint`]) or as lat/long
/// [`Coordinate`]s. On the wire it is untagged: a designator serialises as a
/// bare string (`"YSBK"`) and a coordinate as an object
/// (`{"latitude": …, "longitude": …}`), so the two are told apart by shape.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SignificantPoint {
    /// A coded waypoint designator.
    Designator(Waypoint),
    /// An explicit geographic coordinate.
    Coordinate(Coordinate),
}

impl SignificantPoint {
    /// Parse a coded designator into a significant point.
    pub fn designator(value: impl Into<String>) -> Result<Self, ValidationError> {
        Ok(Self::Designator(Waypoint::parse(value)?))
    }
}

impl std::fmt::Display for SignificantPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Designator(waypoint) => waypoint.fmt(f),
            Self::Coordinate(coordinate) => coordinate.fmt(f),
        }
    }
}

impl From<Waypoint> for SignificantPoint {
    fn from(waypoint: Waypoint) -> Self {
        Self::Designator(waypoint)
    }
}

impl From<Coordinate> for SignificantPoint {
    fn from(coordinate: Coordinate) -> Self {
        Self::Coordinate(coordinate)
    }
}
