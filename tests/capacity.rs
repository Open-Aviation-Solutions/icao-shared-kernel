// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Capacity serialisation.

use icao_shared_kernel::Capacity;

const SPELLINGS: [(Capacity, &str); 4] = [
    (Capacity::PilotInCommand, r#""pilot-in-command""#),
    (Capacity::CoPilot, r#""co-pilot""#),
    (
        Capacity::PilotInCommandUnderSupervision,
        r#""pilot-in-command-under-supervision""#,
    ),
    (Capacity::DualInstruction, r#""dual-instruction""#),
];

#[test]
fn capacities_are_spelled_as_their_names() {
    for (capacity, spelled) in SPELLINGS {
        assert_eq!(serde_json::to_string(&capacity).unwrap(), spelled);
        assert_eq!(serde_json::from_str::<Capacity>(spelled).unwrap(), capacity);
    }
}
