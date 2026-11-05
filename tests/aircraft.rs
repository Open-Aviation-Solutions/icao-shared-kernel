// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Aircraft type, registration, and aggregate behaviour.

use icao_shared_kernel::{
    Aircraft, AircraftRegistration, AircraftType, NationalityMark, ValidationError,
};
use rstest::rstest;
use uuid::Uuid;

#[test]
fn with_preserves_explicit_id() {
    let id = Uuid::new_v4();
    let aircraft = Aircraft::with(
        id,
        AircraftType::parse("C172").unwrap(),
        AircraftRegistration::new(NationalityMark::parse("VH").unwrap(), "XYZ").unwrap(),
    );
    assert_eq!(aircraft.id, id);
}

#[test]
fn registration_renders_and_exposes_marks() {
    let reg = AircraftRegistration::new(NationalityMark::parse("VH").unwrap(), "ABC").unwrap();
    assert_eq!(reg.nationality().as_str(), "VH");
    assert_eq!(reg.registration(), "ABC");
    assert_eq!(reg.to_string(), "VH-ABC");
}

#[test]
fn aircraft_creation_and_display() {
    let aircraft = Aircraft::new(
        AircraftType::parse("C172").unwrap(),
        AircraftRegistration::new(NationalityMark::parse("VH").unwrap(), "XYZ").unwrap(),
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
#[case("")] // too short
#[case("VHXY")] // too long
#[case("vh")] // lowercase
#[case("V-")] // punctuation
#[case("V ")] // whitespace
#[case("999")] // no letter
fn nationality_mark_refused(#[case] value: &str) {
    assert!(matches!(
        NationalityMark::parse(value),
        Err(ValidationError::NationalityMark(_))
    ));
}

#[rstest]
#[case("N")]
#[case("VH")]
#[case("A9C")]
#[case("9XR")]
fn nationality_mark_at_bounds_accepted(#[case] value: &str) {
    assert_eq!(NationalityMark::parse(value).unwrap().as_str(), value);
}

#[rstest]
#[case("")] // registration too short
#[case("ABCDEF")] // registration too long
#[case("abc")] // lowercase
#[case("A-B")] // punctuation
#[case("AB ")] // whitespace
fn registration_mark_refused(#[case] registration: &str) {
    let nationality = NationalityMark::parse("VH").unwrap();
    assert!(matches!(
        AircraftRegistration::new(nationality, registration),
        Err(ValidationError::RegistrationMark(_))
    ));

    let json = format!(r#"{{"nationality":"VH","registration":"{registration}"}}"#);
    assert!(serde_json::from_str::<AircraftRegistration>(&json).is_err());
}

#[test]
fn registration_mark_at_bounds_accepted() {
    let nationality = NationalityMark::parse("N").unwrap();
    assert!(AircraftRegistration::new(nationality, "ABCDE").is_ok());
}

#[rstest]
#[case("")]
#[case("VHXY")]
#[case("vh")]
#[case("V-")]
#[case("999")]
fn bad_nationality_mark_from_json_refused(#[case] nationality: &str) {
    let json = format!(r#"{{"nationality":"{nationality}","registration":"ABC"}}"#);
    assert!(serde_json::from_str::<AircraftRegistration>(&json).is_err());
}

#[test]
fn registration_json_shape_is_unchanged() {
    let json = r#"{"nationality":"VH","registration":"ABC"}"#;
    let reg: AircraftRegistration = serde_json::from_str(json).unwrap();
    assert_eq!(reg.to_string(), "VH-ABC");
    assert_eq!(serde_json::to_string(&reg).unwrap(), json);
}

#[rstest]
#[case("VH", "ABC", "VH-ABC")]
#[case("A9C", "BCD", "A9C-BCD")]
#[case("N", "12345", "N12345")]
#[case("9XR", "1", "9XR1")]
fn registration_hyphen_only_before_a_letter(
    #[case] nationality: &str,
    #[case] registration: &str,
    #[case] expected: &str,
) {
    let reg = AircraftRegistration::new(NationalityMark::parse(nationality).unwrap(), registration)
        .unwrap();
    assert_eq!(reg.to_string(), expected);
}
