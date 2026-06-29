//! Pilot and Licence parity — mirrors `tests/models/test_pilot.py`.

use icao_shared_kernel::{Licence, Pilot};
use uuid::Uuid;

#[test]
fn pilot_creation() {
    let licence = Licence::new("CASA", "12345").unwrap();
    let pilot = Pilot::with(Uuid::new_v4(), "John Smith", None, vec![licence.clone()]).unwrap();
    assert_eq!(pilot.display_name(), "John Smith");
    assert_eq!(pilot.licences, vec![licence]);
    assert_eq!(pilot.legal_name(), None);
}

#[test]
fn defaults_to_no_licences() {
    let pilot = Pilot::new("John Smith").unwrap();
    assert!(pilot.licences.is_empty());
}

#[test]
fn holds_multiple_state_licences() {
    let pilot = Pilot::with(
        Uuid::new_v4(),
        "Amelia",
        None,
        vec![
            Licence::new("CASA", "CASA-1").unwrap(),
            Licence::new("FAA", "FAA-2").unwrap(),
        ],
    )
    .unwrap();
    let authorities: Vec<_> = pilot
        .licences
        .iter()
        .map(|licence| licence.issuing_authority())
        .collect();
    assert_eq!(authorities, ["CASA", "FAA"]);
}

#[test]
fn with_legal_name() {
    let pilot = Pilot::with(
        Uuid::new_v4(),
        "Johnny",
        Some("Johnathan Michael Smith".to_string()),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(pilot.display_name(), "Johnny");
    assert_eq!(pilot.legal_name(), Some("Johnathan Michael Smith"));
}

#[test]
fn licence_number_is_a_string() {
    let licence = Licence::new("EASA", "UK.FCL.0A1B2").unwrap();
    assert_eq!(licence.number(), "UK.FCL.0A1B2");
}

#[test]
fn licence_validation_rejects_empty_fields() {
    assert!(Licence::new("", "12345").is_err());
    assert!(Licence::new("CASA", "").is_err());
}

#[test]
fn pilot_validation_enforces_display_name_bounds() {
    assert!(Pilot::new("").is_err());
    assert!(Pilot::new("A".repeat(101)).is_err());
}
