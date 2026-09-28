//! The flight rules a flight is conducted under.

use serde::{Deserialize, Serialize};

/// The flight rules a flight is flown under, as a flight plan states them (ICAO
/// Doc 4444, Appendix 2, Item 8, "Flight rules"): the instrument flight rules,
/// the visual flight rules, or one then the other.
///
/// Two of the four say which came first, as Item 8 does: a flight that changes
/// from one to the other is planned with the point of change in its route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlightRules {
    /// Under the IFR throughout (Item 8 `I`).
    Ifr,
    /// Under the VFR throughout (Item 8 `V`).
    Vfr,
    /// Under the IFR first, then the VFR (Item 8 `Y`).
    IfrThenVfr,
    /// Under the VFR first, then the IFR (Item 8 `Z`).
    VfrThenIfr,
}
