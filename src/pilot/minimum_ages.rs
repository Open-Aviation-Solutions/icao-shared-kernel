// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The minimum age, in years, for each title of pilot licence.

use time::Date;

use super::licence_title::LicenceTitle;

/// The minimum age, in whole years, at which each title of pilot licence may
/// be first issued.
///
/// [`Default`] is Annex 1's: the applicant "shall be not less than" the age
/// for the licence (2.3.1.1, 2.4.1.1, 2.5.1.1, 2.6.1.1, 2.9.1.1 and 2.10.1.1).
/// The age is the licence's, whatever its category: an airline transport pilot
/// licence for helicopters has the same minimum as one for aeroplanes. A State
/// whose ages differ from Annex 1's supplies its own table, changing only the
/// fields that differ:
///
/// ```
/// use icao_shared_kernel::MinimumAges;
///
/// let ages = MinimumAges { commercial: 19, ..MinimumAges::default() };
/// assert_eq!(ages.private, 17);
/// ```
///
/// Annex 1 sets the age as a requirement on the applicant. The kernel checks
/// it against the date the title was first issued, which comes after the
/// application, so the check can miss an applicant who was too young when
/// they applied, though not one who was too young when the title was issued.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinimumAges {
    /// Private pilot licence (2.3.1.1).
    pub private: u8,
    /// Commercial pilot licence (2.4.1.1).
    pub commercial: u8,
    /// Multi-crew pilot licence (2.5.1.1).
    pub multi_crew: u8,
    /// Airline transport pilot licence (2.6.1.1).
    pub airline_transport: u8,
    /// Glider pilot licence (2.9.1.1).
    pub glider: u8,
    /// Free balloon pilot licence (2.10.1.1).
    pub free_balloon: u8,
}

impl Default for MinimumAges {
    /// Annex 1's minimum ages.
    fn default() -> Self {
        Self {
            private: 17,
            commercial: 18,
            multi_crew: 18,
            airline_transport: 21,
            glider: 16,
            free_balloon: 16,
        }
    }
}

impl MinimumAges {
    // Exhaustive on the title, so that a new title does not compile until it
    // has an age.
    pub(crate) fn for_title(&self, title: LicenceTitle) -> u8 {
        match title {
            LicenceTitle::Private(_) => self.private,
            LicenceTitle::Commercial(_) => self.commercial,
            LicenceTitle::MultiCrew => self.multi_crew,
            LicenceTitle::AirlineTransport(_) => self.airline_transport,
            LicenceTitle::Glider => self.glider,
            LicenceTitle::FreeBalloon => self.free_balloon,
        }
    }
}

/// Whole years completed on `on` by someone born on `born`; negative if `on`
/// is before `born`.
///
/// Annex 1 does not say how age is counted. A year is complete on the
/// birthday, compared by month and day: someone born on 29 February completes
/// a year on 1 March in a year that is not a leap year, so is never treated as
/// older than they are.
pub(crate) fn age_on(born: Date, on: Date) -> i32 {
    let years = on.year() - born.year();
    if (u8::from(on.month()), on.day()) < (u8::from(born.month()), born.day()) {
        years - 1
    } else {
        years
    }
}
