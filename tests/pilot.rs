//! Pilot and Licence construction and validation.

use icao_shared_kernel::{Licence, Pilot};
use rstest::rstest;
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

#[rstest]
#[case("", "12345")] // empty authority
#[case("CASA", "")] // empty number
fn licence_validation_rejects_empty_fields(#[case] authority: &str, #[case] number: &str) {
    assert!(Licence::new(authority, number).is_err());
}

#[rstest]
#[case("".to_string())] // below the 1-character minimum
#[case("A".repeat(101))] // above the 100-character maximum
fn pilot_validation_enforces_display_name_bounds(#[case] display_name: String) {
    assert!(Pilot::new(display_name).is_err());
}
