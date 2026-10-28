// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Capacity serialisation.

use icao_shared_kernel::Capacity;
use rstest::rstest;

// Exhaustive, with no wildcard, so that a new variant does not compile until
// its spelling is written here; add it to the test's values as well.
fn spelling(capacity: Capacity) -> &'static str {
    match capacity {
        Capacity::PilotInCommand => r#""pilot-in-command""#,
        Capacity::CoPilot => r#""co-pilot""#,
        Capacity::PilotInCommandUnderSupervision => r#""pilot-in-command-under-supervision""#,
        Capacity::DualInstruction => r#""dual-instruction""#,
    }
}

#[rstest]
fn capacities_are_spelled_as_their_names(
    #[values(
        Capacity::PilotInCommand,
        Capacity::CoPilot,
        Capacity::PilotInCommandUnderSupervision,
        Capacity::DualInstruction
    )]
    capacity: Capacity,
) {
    let spelled = spelling(capacity);
    assert_eq!(serde_json::to_string(&capacity).unwrap(), spelled);
    assert_eq!(serde_json::from_str::<Capacity>(spelled).unwrap(), capacity);
}

#[rstest]
#[case::solo_is_not_a_capacity(r#""solo""#)]
#[case::student_is_not_icaos_term(r#""student""#)]
#[case::snake_case(r#""pilot_in_command""#)]
fn other_spellings_are_refused(#[case] spelled: &str) {
    assert!(serde_json::from_str::<Capacity>(spelled).is_err());
}
