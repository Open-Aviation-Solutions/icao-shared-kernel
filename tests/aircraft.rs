//! Aircraft parity — mirrors `tests/models/test_aircraft.py`.

use icao_shared_kernel::{Aircraft, AircraftRegistration, AircraftType};

#[test]
fn registration_renders_and_exposes_marks() {
    let reg = AircraftRegistration::new("VH", "ABC").unwrap();
    assert_eq!(reg.nationality(), "VH");
    assert_eq!(reg.registration(), "ABC");
    assert_eq!(reg.to_string(), "VH-ABC");
}

#[test]
fn aircraft_creation_and_display() {
    let aircraft = Aircraft::create("C172", "VH", "XYZ").unwrap();
    assert_eq!(aircraft.aircraft_type.as_str(), "C172");
    assert_eq!(aircraft.to_string(), "C172 VH-XYZ");
}

#[test]
fn zzzz_sentinel_accepted() {
    assert_eq!(AircraftType::parse("ZZZZ").unwrap().as_str(), "ZZZZ");
}

#[test]
fn valid_designators_accepted() {
    for value in ["C172", "B738", "R44", "ZZZZ"] {
        assert!(
            AircraftType::parse(value).is_ok(),
            "{value:?} should be valid"
        );
    }
}

#[test]
fn invalid_designators_rejected() {
    // empty, single char, >4 chars, no leading letter, lowercase, hyphen
    for value in ["", "C", "C1729", "172", "c172", "C-72"] {
        assert!(
            AircraftType::parse(value).is_err(),
            "{value:?} should be rejected"
        );
    }
}
