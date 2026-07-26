//! The `DeviceDesignation` value object — how a physical device is identified.

use serde::{Deserialize, Serialize};

use crate::error::{check_non_empty, ValidationError};

/// Free-text identification of a physical training device — typically
/// manufacturer, model, and serial number.
///
/// A device has no coded designator of its own: ICAO Doc 8643 designators
/// identify aircraft *types*, and no device is an aircraft. What a device does
/// have is whatever its qualifying authority uses to tell it apart from every
/// other device (CASR 60.035(2)(a) requires a qualification certificate to
/// "include information identifying the simulator or device"), plus, for an
/// unqualified device, whatever its owner calls it.
///
/// No format is imposed because no regulation prescribes one — only
/// non-emptiness is enforced.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct DeviceDesignation(String);

impl DeviceDesignation {
    /// Parse a device designation, rejecting an empty one.
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        check_non_empty("designation", &value)?;
        Ok(Self(value))
    }

    /// The designation as written.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for DeviceDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<String> for DeviceDesignation {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<DeviceDesignation> for String {
    fn from(value: DeviceDesignation) -> Self {
        value.0
    }
}
