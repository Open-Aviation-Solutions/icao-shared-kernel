// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The types of instrument approach procedure, and whether each is a 2D or a
//! 3D approach operation.

use serde::{Deserialize, Serialize};

/// The type of an instrument approach procedure, in ICAO's terms. Whether
/// flying it is a 2D or a 3D approach operation (the classes of approach
/// operation in Annex 6) is derived by [`is_3d`](Self::is_3d), not stored; how
/// an authority counts it is that authority's reading.
///
/// This is the procedure, not whether the approach ended in a circling
/// manoeuvre, which is not recorded here. Annex 6 Part I names a "circling
/// approach operation" beside a 2D one (in its definition of minimum descent
/// altitude or height) and does not class a circled approach as 2D, so
/// [`is_3d`](Self::is_3d) is the class of the procedure flown, circled or not.
///
/// A procedure particular to one State (an Australian DME or GNSS arrival, say)
/// is recorded in that authority's section, not here. There is no `Mls` value
/// until one is needed.
///
/// Not `#[non_exhaustive]`, deliberately: an authority mapping approaches to
/// its rules should be made to handle each new value, not fall into a
/// wildcard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum ApproachType {
    /// A non-directional beacon approach: a 2D approach operation.
    Ndb,
    /// A VOR approach: a 2D approach operation.
    Vor,
    /// A localizer approach, without the glide path of an instrument landing
    /// system: a 2D approach operation.
    Localizer,
    /// An instrument landing system approach (Annex 10, Volume I), with
    /// vertical guidance from the glide path: a 3D approach operation.
    Ils,
    /// A GBAS landing system approach (Annex 10, Volume I): a 3D approach
    /// operation.
    Gls,
    /// An RNP APCH approach (Doc 9613, the Performance-based Navigation
    /// Manual), flown to the given minima.
    RnpApch { minima: RnpApchMinima },
    /// An RNP AR APCH approach (Doc 9613): a 3D approach operation.
    RnpAr,
}

/// The minima an RNP APCH approach is flown to (Doc 9613, the
/// Performance-based Navigation Manual).
///
/// Not `#[non_exhaustive]`, deliberately, for the reason [`ApproachType`] is
/// not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RnpApchMinima {
    /// LNAV minima: lateral guidance only, a 2D approach operation.
    Lnav,
    /// LNAV/VNAV minima: lateral and vertical guidance, a 3D approach
    /// operation.
    LnavVnav,
    /// LP minima ("localizer performance"): a 2D approach operation.
    Lp,
    /// LPV minima ("localizer performance with vertical guidance"): a 3D
    /// approach operation.
    Lpv,
}

impl ApproachType {
    /// Whether this procedure is a 3D approach operation (lateral and
    /// vertical guidance, Annex 6), as against a 2D one (lateral guidance
    /// only). Whether the approach ended in a circling manoeuvre does not
    /// change it.
    pub fn is_3d(self) -> bool {
        match self {
            Self::Ndb | Self::Vor | Self::Localizer => false,
            Self::Ils | Self::Gls | Self::RnpAr => true,
            Self::RnpApch { minima } => minima.is_3d(),
        }
    }
}

impl RnpApchMinima {
    /// Whether these minima make the approach a 3D approach operation.
    pub fn is_3d(self) -> bool {
        matches!(self, Self::LnavVnav | Self::Lpv)
    }
}
