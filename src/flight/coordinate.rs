//! A geographic coordinate value object (decimal degrees, WGS-84).

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::error::{check_coordinate_ranges, ValidationError};

/// A latitude/longitude position in decimal degrees.
///
/// Latitude is constrained to −90..=90 and longitude to −180..=180. Stored as
/// [`Decimal`] to keep the recorded precision exact (no binary-float drift).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "CoordinateData", into = "CoordinateData")]
pub struct Coordinate {
    latitude: Decimal,
    longitude: Decimal,
}

impl Coordinate {
    /// Build a coordinate, validating both components are in range.
    pub fn new(latitude: Decimal, longitude: Decimal) -> Result<Self, ValidationError> {
        check_coordinate_ranges(latitude, longitude)?;
        Ok(Self {
            latitude,
            longitude,
        })
    }

    /// Latitude in decimal degrees (−90..=90).
    pub fn latitude(&self) -> Decimal {
        self.latitude
    }

    /// Longitude in decimal degrees (−180..=180).
    pub fn longitude(&self) -> Decimal {
        self.longitude
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.latitude, self.longitude)
    }
}

// Unchecked mirror of the wire shape; deserialisation funnels through `new` so
// an out-of-range coordinate can never be constructed from JSON.
#[derive(Serialize, Deserialize)]
struct CoordinateData {
    latitude: Decimal,
    longitude: Decimal,
}

impl TryFrom<CoordinateData> for Coordinate {
    type Error = ValidationError;

    fn try_from(data: CoordinateData) -> Result<Self, Self::Error> {
        Self::new(data.latitude, data.longitude)
    }
}

impl From<Coordinate> for CoordinateData {
    fn from(value: Coordinate) -> Self {
        Self {
            latitude: value.latitude,
            longitude: value.longitude,
        }
    }
}
