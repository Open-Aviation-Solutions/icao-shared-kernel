// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The issuing State value object, shared by a pilot's licence and a training
//! device's qualification.

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

/// The State that issued a licence or qualification, as its ISO 3166-1
/// alpha-2 code (e.g. `AU`, `US`).
///
/// Checked by shape: exactly two ASCII uppercase letters. Membership of the
/// real code list is intentionally left for a later step. Serialises as the
/// plain code.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct IssuingState(String);

impl IssuingState {
    /// Parse an ISO 3166-1 alpha-2 code: exactly two ASCII uppercase letters.
    pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        let is_alpha2 = value.len() == 2 && value.bytes().all(|b| b.is_ascii_uppercase());
        if !is_alpha2 {
            return Err(ValidationError::IssuingState(value));
        }
        Ok(Self(value))
    }

    /// Borrow the underlying code.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for IssuingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for IssuingState {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<IssuingState> for String {
    fn from(value: IssuingState) -> Self {
        value.0
    }
}
