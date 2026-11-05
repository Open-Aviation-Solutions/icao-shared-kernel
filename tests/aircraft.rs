// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Aircraft type, registration, and aggregate behaviour.

use icao_shared_kernel::{Aircraft, AircraftRegistration, AircraftType};
use rstest::rstest;
use uuid::Uuid;

#[test]
fn with_preserves_explicit_id() {
    let id = Uuid::new_v4();
    let aircraft = Aircraft::with(
        id,
        AircraftType::parse("C172").unwrap(),
        AircraftRegistration::new("VH", "XYZ").unwrap(),
    );
    assert_eq!(aircraft.id, id);
}

#[test]
fn registration_renders_and_exposes_marks() {
    let reg = AircraftRegistration::new("VH", "ABC").unwrap();
    assert_eq!(reg.nationality(), "VH");
    assert_eq!(reg.registration(), "ABC");
    assert_eq!(reg.to_string(), "VH-ABC");
}

#[test]
fn aircraft_creation_and_display() {
    let aircraft = Aircraft::new(
        AircraftType::parse("C172").unwrap(),
        AircraftRegistration::new("VH", "XYZ").unwrap(),
    );
    assert_eq!(aircraft.aircraft_type.as_str(), "C172");
    assert_eq!(aircraft.to_string(), "C172 VH-XYZ");
}

#[rstest]
#[case("C172")]
#[case("B738")]
#[case("R44")]
#[case("ZZZZ")] // sentinel for an unassigned designator
fn valid_designators_accepted(#[case] value: &str) {
    let designator = AircraftType::parse(value).expect("should be valid");
    assert_eq!(designator.as_str(), value);
}

#[rstest]
#[case("")] // empty
#[case("C")] // single char
#[case("C1729")] // too long
#[case("172")] // no leading letter
#[case("c172")] // lowercase
#[case("C-72")] // non-alphanumeric
fn invalid_designators_rejected(#[case] value: &str) {
    assert!(
        AircraftType::parse(value).is_err(),
        "{value:?} should be rejected"
    );
}

#[rstest]
#[case("", "ABC")] // nationality too short
#[case("VHX", "ABC")] // nationality too long
#[case("VH", "")] // registration too short
#[case("VH", "ABCDEF")] // registration too long
fn registration_marks_out_of_bounds_refused(#[case] nationality: &str, #[case] registration: &str) {
    assert!(AircraftRegistration::new(nationality, registration).is_err());

    let json = format!(r#"{{"nationality":"{nationality}","registration":"{registration}"}}"#);
    assert!(serde_json::from_str::<AircraftRegistration>(&json).is_err());
}

#[rstest]
#[case("N", "1")]
#[case("VH", "ABCDE")]
fn registration_marks_at_bounds_accepted(#[case] nationality: &str, #[case] registration: &str) {
    assert!(AircraftRegistration::new(nationality, registration).is_ok());
}
