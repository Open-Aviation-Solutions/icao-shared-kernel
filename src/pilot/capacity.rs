// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The capacity a pilot serves in on a flight, in ICAO's terms.

use serde::{Deserialize, Serialize};

/// The capacity a pilot serves in on a flight, named by the terms ICAO Annex 1
/// defines (Chapter 1, 1.1 Definitions).
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
    /// Pilot-in-command (Annex 1, 1.1 Definitions, "Pilot-in-command").
    PilotInCommand,
    /// Co-pilot (Annex 1, 1.1 Definitions, "Co-pilot").
    CoPilot,
    /// Pilot-in-command under supervision (Annex 1, 1.1 Definitions,
    /// "Pilot-in-command under supervision").
    PilotInCommandUnderSupervision,
    /// Receiving flight instruction: the capacity in which a pilot logs dual
    /// instruction time (Annex 1, 1.1 Definitions, "Dual instruction time").
    DualInstruction,
}
