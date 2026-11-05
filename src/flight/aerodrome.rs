// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `Aerodrome` value object — the place a flight departs from or arrives
//! at, listed in ICAO Doc 7910 or not.

use serde::{Deserialize, Serialize};

use super::coordinate::Coordinate;
use super::location_indicator::LocationIndicator;
use crate::error::ValidationError;

/// The aerodrome a flight departs from or arrives at.
///
/// Follows ICAO Doc 4444 (PANS-ATM), Appendix 2: a flight plan gives the
/// departure and destination aerodromes by their Doc 7910 location
/// indicators (Items 13 and 16), and for one that has none it gives `ZZZZ`
/// there and describes the aerodrome in Item 18 (`DEP/`, `DEST/`), by its
/// name or its position. `ZZZZ` is not held here: an [`Unlisted`] aerodrome
/// holds the description directly.
///
/// On the wire it is untagged: a listed aerodrome is its bare location
/// indicator (`"YSBK"`), and an unlisted one an object (see
/// [`UnlistedAerodrome`]).
///
/// [`Unlisted`]: Self::Unlisted
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Aerodrome {
    /// Listed in ICAO Doc 7910, by its location indicator.
    Listed(LocationIndicator),
    /// Not listed (Doc 4444's `ZZZZ`), described instead.
    Unlisted(UnlistedAerodrome),
}

impl Aerodrome {
    /// Parse a location indicator into a listed aerodrome.
    pub fn listed(location_indicator: impl Into<String>) -> Result<Self, ValidationError> {
        Ok(Self::Listed(LocationIndicator::parse(location_indicator)?))
    }

    /// Describe an unlisted aerodrome; see [`UnlistedAerodrome::new`].
    pub fn unlisted(
        name: Option<String>,
        position: Option<Coordinate>,
    ) -> Result<Self, ValidationError> {
        Ok(Self::Unlisted(UnlistedAerodrome::new(name, position)?))
    }
}

/// Its location indicator if listed; if not, its name, or else its position.
impl std::fmt::Display for Aerodrome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Listed(location_indicator) => location_indicator.fmt(f),
            Self::Unlisted(unlisted) => unlisted.fmt(f),
        }
    }
}

impl From<LocationIndicator> for Aerodrome {
    fn from(location_indicator: LocationIndicator) -> Self {
        Self::Listed(location_indicator)
    }
}

impl From<UnlistedAerodrome> for Aerodrome {
    fn from(unlisted: UnlistedAerodrome) -> Self {
        Self::Unlisted(unlisted)
    }
}

/// An aerodrome with no location indicator, described as Doc 4444's Item 18
/// describes one (`DEP/`, `DEST/`): by its name, its position, or both.
///
/// At least one of the two is given, and a name is not empty or only
/// whitespace. Item 18's third form, a bearing and distance from a
/// significant point, is not held.
///
/// On the wire it is an object, `{"name": …, "position": {"latitude": …,
/// "longitude": …}}`, with an absent part left out. A bare coordinate
/// (`{"latitude": …, "longitude": …}`), as a flight's departure or arrival
/// was once recorded, is read as a position with no name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "UnlistedAerodromeData", into = "DescribedData")]
pub struct UnlistedAerodrome {
    name: Option<String>,
    position: Option<Coordinate>,
}

impl UnlistedAerodrome {
    /// Describe an unlisted aerodrome by its name, its position, or both.
    ///
    /// Refuses one with neither, and a name that is empty or only
    /// whitespace. The name is kept as given.
    pub fn new(
        name: Option<String>,
        position: Option<Coordinate>,
    ) -> Result<Self, ValidationError> {
        if name.as_deref().is_some_and(|name| name.trim().is_empty()) {
            return Err(ValidationError::Empty {
                field: "aerodrome name",
            });
        }
        if name.is_none() && position.is_none() {
            return Err(ValidationError::UnlistedAerodrome);
        }
        Ok(Self { name, position })
    }

    /// Its name, if given.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Its position, if given.
    pub fn position(&self) -> Option<&Coordinate> {
        self.position.as_ref()
    }
}

/// Its name, or else its position.
impl std::fmt::Display for UnlistedAerodrome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.name, &self.position) {
            (Some(name), _) => f.write_str(name),
            (None, Some(position)) => position.fmt(f),
            (None, None) => unreachable!("an unlisted aerodrome has a name or a position"),
        }
    }
}

// Unchecked mirrors of the wire shapes; deserialisation funnels through `new`
// so an aerodrome with neither can never be built from JSON. The bare
// coordinate is tried first, and the description refuses unknown fields, so
// that a coordinate is never read as an empty description.
#[derive(Deserialize)]
#[serde(untagged)]
enum UnlistedAerodromeData {
    Position(Coordinate),
    Described(DescribedData),
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DescribedData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    position: Option<Coordinate>,
}

impl TryFrom<UnlistedAerodromeData> for UnlistedAerodrome {
    type Error = ValidationError;

    fn try_from(data: UnlistedAerodromeData) -> Result<Self, Self::Error> {
        match data {
            UnlistedAerodromeData::Position(position) => Self::new(None, Some(position)),
            UnlistedAerodromeData::Described(DescribedData { name, position }) => {
                Self::new(name, position)
            }
        }
    }
}

impl From<UnlistedAerodrome> for DescribedData {
    fn from(value: UnlistedAerodrome) -> Self {
        Self {
            name: value.name,
            position: value.position,
        }
    }
}
