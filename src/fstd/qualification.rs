//! The `DeviceQualification` value object — a device's current qualification.

use serde::{Deserialize, Serialize};
use time::Date;

use crate::error::{check_issuing_state, check_non_empty, check_validity_period, ValidationError};

/// A qualification granted to a training device by a national aviation
/// authority, valid over a bounded period.
///
/// **Qualifications expire.** CASR 60.040 puts a flight simulator or flight
/// training device qualification in force for 12 months from the date the
/// certificate is issued (or a shorter period if the certificate says so), and
/// CASR 60.050 allows it to be varied, cancelled or suspended. "Is this device
/// qualified?" therefore has no answer without a date — see
/// [`is_in_force_on`](Self::is_in_force_on).
///
/// The identity is `(issuing_state, issuing_authority)`, matching
/// [`Licence`](crate::Licence): the issuing State disambiguates authorities
/// that share an acronym across countries.
///
/// Whether a qualification *counts* toward a given licensing requirement is a
/// national question and is deliberately not answered here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "DeviceQualificationData", into = "DeviceQualificationData")]
pub struct DeviceQualification {
    issuing_state: String,
    issuing_authority: String,
    level: String,
    valid_from: Date,
    valid_until: Date,
}

impl DeviceQualification {
    /// Build a qualification, validating the issuing State is an ISO 3166-1
    /// alpha-2 code, that the authority and level are non-empty, and that the
    /// validity period does not end before it starts.
    pub fn new(
        issuing_state: impl Into<String>,
        issuing_authority: impl Into<String>,
        level: impl Into<String>,
        valid_from: Date,
        valid_until: Date,
    ) -> Result<Self, ValidationError> {
        let issuing_state = issuing_state.into();
        let issuing_authority = issuing_authority.into();
        let level = level.into();
        check_issuing_state(&issuing_state)?;
        check_non_empty("issuing_authority", &issuing_authority)?;
        check_non_empty("level", &level)?;
        check_validity_period(valid_from, valid_until)?;
        Ok(Self {
            issuing_state,
            issuing_authority,
            level,
            valid_from,
            valid_until,
        })
    }

    /// The ISO 3166-1 alpha-2 code of the State whose authority qualified the
    /// device.
    pub fn issuing_state(&self) -> &str {
        &self.issuing_state
    }

    /// The coded qualifying authority, e.g. `CASA`, `FAA`, `EASA`.
    pub fn issuing_authority(&self) -> &str {
        &self.issuing_authority
    }

    /// The qualification level as the authority expressed it.
    ///
    /// A string rather than an enum, deliberately: nothing in the domain
    /// branches on the level yet, and the levels in use span several schemes
    /// that a single enum would have to unify prematurely — ICAO Doc 9625
    /// Types I–VII, simulator Levels A–D, and the FAA Level 4–7 / EASA Level
    /// 1–3 flight training device levels that CASR 60.020 adopts by reference.
    pub fn level(&self) -> &str {
        &self.level
    }

    /// First date the qualification is in force.
    pub fn valid_from(&self) -> Date {
        self.valid_from
    }

    /// Last date the qualification is in force.
    pub fn valid_until(&self) -> Date {
        self.valid_until
    }

    /// Whether the qualification was in force on `date`, inclusive of both
    /// end points.
    pub fn is_in_force_on(&self, date: Date) -> bool {
        self.valid_from <= date && date <= self.valid_until
    }
}

// A plain, unchecked mirror of the wire shape. `DeviceQualification` keeps its
// fields private so `new` is the only way to build one; serde's derived
// `Deserialize` would bypass it and let an invalid validity period in from
// JSON, so deserialisation funnels through `try_from` instead. Same pattern as
// `Licence`.
#[derive(Serialize, Deserialize)]
struct DeviceQualificationData {
    issuing_state: String,
    issuing_authority: String,
    level: String,
    valid_from: Date,
    valid_until: Date,
}

impl TryFrom<DeviceQualificationData> for DeviceQualification {
    type Error = ValidationError;

    fn try_from(data: DeviceQualificationData) -> Result<Self, Self::Error> {
        Self::new(
            data.issuing_state,
            data.issuing_authority,
            data.level,
            data.valid_from,
            data.valid_until,
        )
    }
}

impl From<DeviceQualification> for DeviceQualificationData {
    fn from(value: DeviceQualification) -> Self {
        Self {
            issuing_state: value.issuing_state,
            issuing_authority: value.issuing_authority,
            level: value.level,
            valid_from: value.valid_from,
            valid_until: value.valid_until,
        }
    }
}
