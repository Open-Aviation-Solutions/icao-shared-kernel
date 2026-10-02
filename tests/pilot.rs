//! Pilot and Licence construction and validation.

use icao_shared_kernel::{Licence, Pilot};
use rstest::rstest;
use time::macros::date;
use time::Date;
use uuid::Uuid;

const BORN: Date = date!(1990 - 06 - 15);

#[test]
fn pilot_creation() {
    let licence = Licence::new("AU", "CASA", "12345").unwrap();
    let pilot = Pilot::with(Uuid::new_v4(), "John Smith", BORN, vec![licence.clone()]).unwrap();
    assert_eq!(pilot.full_name(), "John Smith");
    assert_eq!(pilot.date_of_birth(), BORN);
    assert_eq!(pilot.licences, vec![licence]);
}

#[test]
fn defaults_to_no_licences() {
    let pilot = Pilot::new("John Smith", BORN).unwrap();
    assert!(pilot.licences.is_empty());
}

#[test]
fn holds_multiple_state_licences() {
    let pilot = Pilot::with(
        Uuid::new_v4(),
        "Amelia Earhart",
        BORN,
        vec![
            Licence::new("AU", "CASA", "CASA-1").unwrap(),
            Licence::new("US", "FAA", "FAA-2").unwrap(),
        ],
    )
    .unwrap();
    let authorities: Vec<_> = pilot
        .licences
        .iter()
        .map(|licence| (licence.issuing_state(), licence.issuing_authority()))
        .collect();
    assert_eq!(authorities, [("AU", "CASA"), ("US", "FAA")]);
}

#[test]
fn a_pilot_without_a_date_of_birth_is_rejected() {
    // Reg 61.345(2): the logbook records the full name and the date of birth.
    let error = serde_json::from_str::<Pilot>(r#"{"full_name": "John Smith"}"#).unwrap_err();
    assert!(error.to_string().contains("date_of_birth"), "{error}");
}

#[test]
fn a_pilot_round_trips_with_an_iso_date_of_birth() {
    let json = serde_json::to_value(Pilot::new("John Smith", BORN).unwrap()).unwrap();
    assert_eq!(json["date_of_birth"], "1990-06-15");
    let parsed: Pilot = serde_json::from_value(json).unwrap();
    assert_eq!(parsed.date_of_birth(), BORN);
}

#[test]
fn the_full_name_is_trimmed() {
    let pilot = Pilot::new("  John Smith ", BORN).unwrap();
    assert_eq!(pilot.full_name(), "John Smith");
}

#[test]
fn licence_number_is_an_alphanumeric_string() {
    // EASA-style identifiers embed a country prefix and letters.
    let licence = Licence::new("GB", "UK CAA", "UK.FCL.0A1B2").unwrap();
    assert_eq!(licence.number(), "UK.FCL.0A1B2");
}

#[test]
fn licence_number_has_no_upper_length_bound() {
    // ICAO Annex 1 sets no maximum; only non-emptiness is enforced.
    let long_number = "1".repeat(200);
    assert!(Licence::new("AU", "CASA", &long_number).is_ok());
}

#[rstest]
#[case("AU", "", "12345")] // empty authority
#[case("AU", "CASA", "")] // empty number
fn licence_validation_rejects_empty_fields(
    #[case] state: &str,
    #[case] authority: &str,
    #[case] number: &str,
) {
    assert!(Licence::new(state, authority, number).is_err());
}

#[rstest]
#[case("australia")] // not a code
#[case("aus")] // three letters
#[case("A")] // one letter
#[case("au")] // lowercase
#[case("")] // empty
fn licence_validation_rejects_bad_issuing_state(#[case] state: &str) {
    assert!(Licence::new(state, "CASA", "12345").is_err());
}

#[rstest]
#[case("".to_string())] // below the 1-character minimum
#[case("   ".to_string())] // no name once trimmed
#[case("A".repeat(101))] // above the 100-character maximum
fn pilot_validation_enforces_full_name_bounds(#[case] full_name: String) {
    assert!(Pilot::new(full_name, BORN).is_err());
}
