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

/// The categories of aircraft a private or a commercial pilot licence is issued
/// for (Annex 1, 2.3.1 and 2.4.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrivateOrCommercialCategory {
    Aeroplane,
    Airship,
    Helicopter,
    PoweredLift,
}

/// The categories of aircraft an airline transport pilot licence is issued for
/// (Annex 1, 2.6.3 to 2.6.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AirlineTransportCategory {
    Aeroplane,
    Helicopter,
    PoweredLift,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            Self::Private(category) | Self::Commercial(category) => match category {
                PrivateOrCommercialCategory::Aeroplane => AircraftCategory::Aeroplane,
                PrivateOrCommercialCategory::Airship => AircraftCategory::Airship,
                PrivateOrCommercialCategory::Helicopter => AircraftCategory::Helicopter,
                PrivateOrCommercialCategory::PoweredLift => AircraftCategory::PoweredLift,
            },
            Self::MultiCrew => AircraftCategory::Aeroplane,
            Self::AirlineTransport(category) => match category {
                AirlineTransportCategory::Aeroplane => AircraftCategory::Aeroplane,
                AirlineTransportCategory::Helicopter => AircraftCategory::Helicopter,
                AirlineTransportCategory::PoweredLift => AircraftCategory::PoweredLift,
            },
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
        use AircraftCategory as C;
        let private_or_commercial = match category {
            C::Aeroplane => Some(PrivateOrCommercialCategory::Aeroplane),
            C::Airship => Some(PrivateOrCommercialCategory::Airship),
            C::Helicopter => Some(PrivateOrCommercialCategory::Helicopter),
            C::PoweredLift => Some(PrivateOrCommercialCategory::PoweredLift),
            C::FreeBalloon | C::Glider => None,
        };
        let airline_transport = match category {
            C::Aeroplane => Some(AirlineTransportCategory::Aeroplane),
            C::Helicopter => Some(AirlineTransportCategory::Helicopter),
            C::PoweredLift => Some(AirlineTransportCategory::PoweredLift),
            C::Airship | C::FreeBalloon | C::Glider => None,
        };
        match kind {
            LicenceKind::Private => private_or_commercial.map(Self::Private),
            LicenceKind::Commercial => private_or_commercial.map(Self::Commercial),
            LicenceKind::MultiCrew => (category == C::Aeroplane).then_some(Self::MultiCrew),
            LicenceKind::AirlineTransport => airline_transport.map(Self::AirlineTransport),
            LicenceKind::Glider => (category == C::Glider).then_some(Self::Glider),
            LicenceKind::FreeBalloon => (category == C::FreeBalloon).then_some(Self::FreeBalloon),
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
/// between. It is not checked against today, as the kernel has no clock, nor
/// against the holder's date of birth: Annex 1 sets a minimum age for each
/// licence (2.3.1.1, 2.4.1.1, 2.5.1.1, 2.6.1.1, 2.9.1.1, 2.10.1.1), but a State
/// may differ from a Standard, so an age rule is for a consumer to apply.
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
