// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The categories of aircraft a pilot licence is issued for.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A category of aircraft for which a pilot licence is required (Annex 1,
/// 2.1.1.1), and which the title of the licence includes or which is endorsed
/// on it as a category rating (2.1.1.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AircraftCategory {
    Aeroplane,
    /// An airship of a volume of more than 4 600 cubic metres: 2.1.1.1 requires
    /// a pilot licence for no smaller one.
    Airship,
    FreeBalloon,
    Glider,
    Helicopter,
    PoweredLift,
}

impl fmt::Display for AircraftCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Aeroplane => "aeroplane",
            Self::Airship => "airship",
            Self::FreeBalloon => "free balloon",
            Self::Glider => "glider",
            Self::Helicopter => "helicopter",
            Self::PoweredLift => "powered-lift",
        })
    }
}
