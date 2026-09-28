//! The flight rules a flight is conducted under.

use serde::{Deserialize, Serialize};

/// The flight rules a flight is flown under, as a flight plan states them (ICAO
/// Doc 4444, Appendix 2, Item 8, "Flight rules"): the instrument flight rules,
/// the visual flight rules, or one of them initially, followed by one or more
/// changes of flight rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlightRules {
    /// Under the IFR throughout (Item 8 `I`).
    Ifr,
    /// Under the VFR throughout (Item 8 `V`).
    Vfr,
    /// Initially under the IFR, followed by one or more changes of flight
    /// rules (Item 8 `Y`).
    InitiallyIfr,
    /// Initially under the VFR, followed by one or more changes of flight
    /// rules (Item 8 `Z`).
    InitiallyVfr,
}
