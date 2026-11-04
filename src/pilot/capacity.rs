// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The capacity a pilot serves in on a flight, in ICAO's terms.

use serde::{Deserialize, Serialize};

/// The capacity a pilot serves in on a flight, named by the terms ICAO Annex 1
/// defines (Fourteenth Edition, Chapter 1, 1.1 Definitions), which each
/// variant quotes.
///
/// Each variant names ICAO's term, but a record holding it is the pilot's
/// claim to that capacity *as the logging authority defines it*. It is not an
/// assertion that ICAO's definition is met. A State's definition can differ
/// from ICAO's at the edges: a flight that CASA counts as pilot in command
/// under supervision by the route in CASR 1998 reg 61.095(3), supervised by a
/// flight instructor or examiner who need not be the pilot in command, can
/// fall outside ICAO's words. What a State adds to or narrows in a term is
/// that State's, and is not held here.
///
/// The conditions in the definitions relate a pilot to the rest of the flight
/// (the supervising pilot-in-command, the instructor on board), so they are
/// checked, if at all, where the flight's record is put together, and not by
/// this type. Solo is not a capacity: Annex 1's solo flight time is a kind of
/// flight time, which a pilot-in-command can log as well.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Capacity {
    /// Pilot-in-command: "The pilot designated by the operator, or in the case
    /// of general aviation, the owner, as being in command and charged with
    /// the safe conduct of a flight." (Annex 1, 1.1)
    PilotInCommand,
    /// Co-pilot: "A licensed pilot serving in any piloting capacity other than
    /// as pilot-in-command but excluding a pilot who is on board the aircraft
    /// for the sole purpose of receiving flight instruction." (Annex 1, 1.1)
    CoPilot,
    /// Pilot-in-command under supervision: "Co-pilot performing, under the
    /// supervision of the pilot-in-command, the duties and functions of a
    /// pilot-in-command, in accordance with a method of supervision acceptable
    /// to the Licensing Authority." (Annex 1, 1.1)
    ///
    /// ICAO's pilot-in-command under supervision is a kind of co-pilot, and is
    /// recorded as this variant in place of [`Capacity::CoPilot`], so ICAO's
    /// co-pilot time is the time of both.
    PilotInCommandUnderSupervision,
    /// The capacity in which a pilot logs dual instruction time: "Flight time
    /// during which a person is receiving flight instruction from a properly
    /// authorized pilot on board the aircraft, or from a properly authorized
    /// remote pilot using the remote pilot station during a remotely piloted
    /// aircraft flight." (Annex 1, 1.1, "Dual instruction time")
    DualInstruction,
}
