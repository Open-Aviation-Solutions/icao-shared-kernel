// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `Aerodrome` value object — the place a flight departs from or arrives
//! at, listed in ICAO Doc 7910 or not.

use rust_decimal::Decimal;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

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
/// On the wire a listed aerodrome is its bare location indicator (`"YSBK"`),
/// and an unlisted one an object (see [`UnlistedAerodrome`]). Reading one
/// runs the same checks as building it, and a value they refuse fails with
/// their error.
///
/// [`Unlisted`]: Self::Unlisted
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(try_from = "AerodromeData")]
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

/// Its location indicator if listed; if not, as [`UnlistedAerodrome`] shows
/// itself.
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
/// At least one of the two is given. A name is held without its surrounding
/// whitespace, and is refused when nothing is left or when it holds a
/// control character (a line break, for one). Item 18's third form, a
/// bearing and distance from a significant point, is not held.
///
/// On the wire it is an object, `{"name": …, "position": {"latitude": …,
/// "longitude": …}}`, with an absent part left out. As an [`Aerodrome`], a
/// bare coordinate (`{"latitude": …, "longitude": …}`, with no other key),
/// as a flight's departure or arrival was once recorded, is read as a
/// position with no name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "DescribedData", into = "DescribedData")]
pub struct UnlistedAerodrome {
    name: Option<String>,
    position: Option<Coordinate>,
}

impl UnlistedAerodrome {
    /// Describe an unlisted aerodrome by its name, its position, or both.
    ///
    /// The name's surrounding whitespace is removed. Refuses one with
    /// neither, a name with nothing else, and a name holding a control
    /// character.
    pub fn new(
        name: Option<String>,
        position: Option<Coordinate>,
    ) -> Result<Self, ValidationError> {
        let name = name.map(|name| name.trim().to_owned());
        if let Some(name) = &name {
            if name.is_empty() {
                return Err(ValidationError::Empty {
                    field: "aerodrome name",
                });
            }
            if name.chars().any(char::is_control) {
                return Err(ValidationError::ControlCharacter {
                    field: "aerodrome name",
                });
            }
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

/// Its name, its position, or both (`Farm strip (-33.9461,151.1772)`).
impl std::fmt::Display for UnlistedAerodrome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.name, &self.position) {
            (Some(name), Some(position)) => write!(f, "{name} ({position})"),
            (Some(name), None) => f.write_str(name),
            (None, Some(position)) => position.fmt(f),
            (None, None) => unreachable!("an unlisted aerodrome has a name or a position"),
        }
    }
}

// Unchecked mirrors of the wire shapes. Deserialisation funnels through the
// constructors, so that nothing invalid is built from JSON and a refused value
// fails with the constructor's own error, not serde's "did not match any
// variant" (which is why `Aerodrome` is not read as an untagged enum).
enum AerodromeData {
    Listed(String),
    Position {
        latitude: Decimal,
        longitude: Decimal,
    },
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

// Every key an unlisted aerodrome may be read with: a description's, or a
// bare coordinate's. Any other key is refused, so that none is dropped.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ObjectData {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    position: Option<Coordinate>,
    #[serde(default)]
    latitude: Option<Decimal>,
    #[serde(default)]
    longitude: Option<Decimal>,
}

impl<'de> Deserialize<'de> for AerodromeData {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct AerodromeVisitor;

        impl<'de> Visitor<'de> for AerodromeVisitor {
            type Value = AerodromeData;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a location indicator, or an object describing an unlisted aerodrome")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(AerodromeData::Listed(value.to_owned()))
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                let object = ObjectData::deserialize(de::value::MapAccessDeserializer::new(map))?;
                match object {
                    ObjectData {
                        name: None,
                        position: None,
                        latitude: Some(latitude),
                        longitude: Some(longitude),
                    } => Ok(AerodromeData::Position {
                        latitude,
                        longitude,
                    }),
                    ObjectData {
                        name,
                        position,
                        latitude: None,
                        longitude: None,
                    } => Ok(AerodromeData::Described(DescribedData { name, position })),
                    _ => Err(de::Error::custom(
                        "an unlisted aerodrome is either {\"name\", \"position\"} \
                         or a bare {\"latitude\", \"longitude\"}, not a mix of them",
                    )),
                }
            }
        }

        deserializer.deserialize_any(AerodromeVisitor)
    }
}

impl TryFrom<AerodromeData> for Aerodrome {
    type Error = ValidationError;

    fn try_from(data: AerodromeData) -> Result<Self, Self::Error> {
        match data {
            AerodromeData::Listed(location_indicator) => Self::listed(location_indicator),
            AerodromeData::Position {
                latitude,
                longitude,
            } => Self::unlisted(None, Some(Coordinate::new(latitude, longitude)?)),
            AerodromeData::Described(described) => Ok(Self::Unlisted(described.try_into()?)),
        }
    }
}

impl TryFrom<DescribedData> for UnlistedAerodrome {
    type Error = ValidationError;

    fn try_from(data: DescribedData) -> Result<Self, Self::Error> {
        Self::new(data.name, data.position)
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
