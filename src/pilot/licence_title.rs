// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The titles of licence Annex 1 establishes for pilots, each with the category
//! of aircraft it is for, and the date a holder was first issued one.

use std::fmt;

use serde::{Deserialize, Serialize};
use time::Date;

use super::aircraft_category::AircraftCategory;
use crate::error::ValidationError;

// A licence's own categories of aircraft: a subset of `AircraftCategory`,
// whose variants they share by name, with the conversion into it and, for
// the crate, back.
macro_rules! categories {
    ($(#[$doc:meta])* $name:ident { $($variant:ident),+ }) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum $name {
            $($variant),+
        }

        impl From<$name> for AircraftCategory {
            fn from(category: $name) -> Self {
                match category {
                    $($name::$variant => Self::$variant),+
                }
            }
        }

        impl $name {
            // `None` for a category not in this subset.
            pub(crate) fn from_category(category: AircraftCategory) -> Option<Self> {
                match category {
                    $(AircraftCategory::$variant => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

categories! {
    /// The categories of aircraft a private or a commercial pilot licence is
    /// issued for (Annex 1, 2.3.1 and 2.4.1).
    PrivateOrCommercialCategory { Aeroplane, Airship, Helicopter, PoweredLift }
}

categories! {
    /// The categories of aircraft an airline transport pilot licence is issued
    /// for (Annex 1, 2.6.3 to 2.6.5).
    AirlineTransportCategory { Aeroplane, Helicopter, PoweredLift }
}

/// The title of a pilot licence (Annex 1, 5.2.1 II), with the category of
/// aircraft it is for: the category is included in the title, or endorsed on
/// the licence as a category rating (2.1.1.2), and either way the holder holds
/// that licence for that category.
///
/// These are the pilot licences Annex 1, Chapter 2, establishes, but for the
/// remote pilot licences (2.11 to 2.14). Each variant admits only the
/// categories Annex 1 issues it for, so no other combination can be built. A
/// licence that is a State's own and not Annex 1's (CASA's recreational pilot
/// licence, for one) is not here: it belongs downstream, keyed by the
/// [`Licence`](crate::Licence)'s identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LicenceTitle {
    /// Private pilot licence (2.3).
    Private(PrivateOrCommercialCategory),
    /// Commercial pilot licence (2.4).
    Commercial(PrivateOrCommercialCategory),
    /// Multi-crew pilot licence, appropriate to the aeroplane category (2.5).
    MultiCrew,
    /// Airline transport pilot licence (2.6).
    AirlineTransport(AirlineTransportCategory),
    /// Glider pilot licence (2.9).
    Glider,
    /// Free balloon pilot licence (2.10).
    FreeBalloon,
}

impl LicenceTitle {
    /// The category of aircraft the licence is for.
    pub fn category(self) -> AircraftCategory {
        match self {
            Self::Private(category) | Self::Commercial(category) => category.into(),
            Self::MultiCrew => AircraftCategory::Aeroplane,
            Self::AirlineTransport(category) => category.into(),
            Self::Glider => AircraftCategory::Glider,
            Self::FreeBalloon => AircraftCategory::FreeBalloon,
        }
    }

    fn kind(self) -> LicenceKind {
        match self {
            Self::Private(_) => LicenceKind::Private,
            Self::Commercial(_) => LicenceKind::Commercial,
            Self::MultiCrew => LicenceKind::MultiCrew,
            Self::AirlineTransport(_) => LicenceKind::AirlineTransport,
            Self::Glider => LicenceKind::Glider,
            Self::FreeBalloon => LicenceKind::FreeBalloon,
        }
    }

    // The inverse of `kind` and `category`: `None` for a combination Annex 1
    // does not issue.
    fn from_parts(kind: LicenceKind, category: AircraftCategory) -> Option<Self> {
        let only = |own: AircraftCategory, title: Self| (category == own).then_some(title);
        match kind {
            LicenceKind::Private => {
                PrivateOrCommercialCategory::from_category(category).map(Self::Private)
            }
            LicenceKind::Commercial => {
                PrivateOrCommercialCategory::from_category(category).map(Self::Commercial)
            }
            LicenceKind::MultiCrew => only(AircraftCategory::Aeroplane, Self::MultiCrew),
            LicenceKind::AirlineTransport => {
                AirlineTransportCategory::from_category(category).map(Self::AirlineTransport)
            }
            LicenceKind::Glider => only(AircraftCategory::Glider, Self::Glider),
            LicenceKind::FreeBalloon => only(AircraftCategory::FreeBalloon, Self::FreeBalloon),
        }
    }
}

impl fmt::Display for LicenceTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.kind().name(), self.category())
    }
}

/// A title of licence held under a [`Licence`](crate::Licence), from the date
/// the holder was first issued it.
///
/// `date_of_first_issue` is that date, not the date of the latest document
/// (the date of such issue, 5.2.1 X): a reissued document carries a later
/// date, which would make "held the licence on date X" wrong for the dates
/// between. It is not checked against today, as the kernel has no clock. The
/// [`Pilot`](crate::Pilot) holding it checks it against their date of birth
/// and the title's minimum age (see [`MinimumAges`](crate::MinimumAges)).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "IssuedTitleData", into = "IssuedTitleData")]
pub struct IssuedTitle {
    pub title: LicenceTitle,
    pub date_of_first_issue: Date,
}

impl IssuedTitle {
    pub fn new(title: LicenceTitle, date_of_first_issue: Date) -> Self {
        Self {
            title,
            date_of_first_issue,
        }
    }
}

// The wire shape names the licence and its category side by side, as a
// person reads them, so every entry carries a category, and `try_from` refuses
// a combination Annex 1 does not issue: the typed `LicenceTitle` cannot hold
// one.
#[derive(Serialize, Deserialize)]
struct IssuedTitleData {
    licence: LicenceKind,
    category: AircraftCategory,
    #[serde(with = "crate::iso_date")]
    date_of_first_issue: Date,
}

// The licence of a `LicenceTitle` without its category, for the wire and for
// messages only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum LicenceKind {
    Private,
    Commercial,
    MultiCrew,
    AirlineTransport,
    Glider,
    FreeBalloon,
}

impl LicenceKind {
    fn name(self) -> &'static str {
        match self {
            Self::Private => "private pilot licence",
            Self::Commercial => "commercial pilot licence",
            Self::MultiCrew => "multi-crew pilot licence",
            Self::AirlineTransport => "airline transport pilot licence",
            Self::Glider => "glider pilot licence",
            Self::FreeBalloon => "free balloon pilot licence",
        }
    }
}

impl TryFrom<IssuedTitleData> for IssuedTitle {
    type Error = ValidationError;

    fn try_from(data: IssuedTitleData) -> Result<Self, Self::Error> {
        let title = LicenceTitle::from_parts(data.licence, data.category).ok_or(
            ValidationError::LicenceCategory {
                licence: data.licence.name(),
                category: data.category,
            },
        )?;
        Ok(Self::new(title, data.date_of_first_issue))
    }
}

impl From<IssuedTitle> for IssuedTitleData {
    fn from(value: IssuedTitle) -> Self {
        Self {
            licence: value.title.kind(),
            category: value.title.category(),
            date_of_first_issue: value.date_of_first_issue,
        }
    }
}
