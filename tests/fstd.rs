//! Device designation, qualification validity, and aggregate behaviour.

use icao_shared_kernel::{
    DeviceDesignation, DeviceQualification, FlightSimulationTrainingDevice, FstdKind,
};
use rstest::rstest;
use time::macros::date;
use time::Date;
use uuid::Uuid;

fn casa_qualification(valid_from: Date, valid_until: Date) -> DeviceQualification {
    DeviceQualification::new("AU", "CASA", "Level D", valid_from, valid_until)
        .expect("should be valid")
}

#[test]
fn with_preserves_explicit_id() {
    let id = Uuid::new_v4();
    let device = FlightSimulationTrainingDevice::with(
        id,
        FstdKind::FlightSimulator,
        DeviceDesignation::parse("CAE 7000XR B738 s/n 1234").unwrap(),
        None,
    );
    assert_eq!(device.id, id);
}

#[test]
fn unqualified_device_is_a_valid_device() {
    let device =
        FlightSimulationTrainingDevice::create(FstdKind::BasicInstrumentFlightTrainer, "Home sim")
            .unwrap();
    assert!(device.qualification.is_none());
    assert_eq!(device.to_string(), "Home sim");
}

#[rstest]
#[case(FstdKind::FlightSimulator, "flight-simulator")]
#[case(FstdKind::FlightProceduresTrainer, "flight-procedures-trainer")]
#[case(
    FstdKind::BasicInstrumentFlightTrainer,
    "basic-instrument-flight-trainer"
)]
fn kinds_are_the_annex_1_triple(#[case] kind: FstdKind, #[case] serialised: &str) {
    assert_eq!(
        serde_json::to_string(&kind).unwrap(),
        format!("\"{serialised}\"")
    );
}

#[test]
fn empty_designation_rejected() {
    assert!(DeviceDesignation::parse("").is_err());
}

#[test]
fn qualification_period_ending_before_it_starts_rejected() {
    let result = DeviceQualification::new(
        "AU",
        "CASA",
        "Level D",
        date!(2026 - 03 - 01),
        date!(2026 - 02 - 28),
    );
    assert!(result.is_err());
}

#[rstest]
#[case("au")] // lowercase
#[case("AUS")] // three letters
#[case("A")] // single letter
#[case("")] // empty
fn qualification_rejects_malformed_issuing_state(#[case] state: &str) {
    let result = DeviceQualification::new(
        state,
        "CASA",
        "Level D",
        date!(2026 - 01 - 01),
        date!(2026 - 12 - 31),
    );
    assert!(result.is_err(), "{state:?} should be rejected");
}

#[rstest]
#[case("", "Level D")] // empty authority
#[case("CASA", "")] // empty level
fn qualification_rejects_empty_fields(#[case] authority: &str, #[case] level: &str) {
    let result = DeviceQualification::new(
        "AU",
        authority,
        level,
        date!(2026 - 01 - 01),
        date!(2026 - 12 - 31),
    );
    assert!(result.is_err());
}

#[rstest]
#[case(date!(2025 - 12 - 31), false)] // day before
#[case(date!(2026 - 01 - 01), true)] // first day, inclusive
#[case(date!(2026 - 06 - 15), true)] // mid-period
#[case(date!(2026 - 12 - 31), true)] // last day, inclusive
#[case(date!(2027 - 01 - 01), false)] // day after
fn is_in_force_on_is_inclusive_at_both_ends(#[case] date: Date, #[case] expected: bool) {
    let qualification = casa_qualification(date!(2026 - 01 - 01), date!(2026 - 12 - 31));
    assert_eq!(qualification.is_in_force_on(date), expected);
}

#[test]
fn single_day_qualification_is_in_force_on_that_day() {
    let qualification = casa_qualification(date!(2026 - 05 - 05), date!(2026 - 05 - 05));
    assert!(qualification.is_in_force_on(date!(2026 - 05 - 05)));
}

#[test]
fn qualification_round_trips_through_json() {
    let qualification = casa_qualification(date!(2026 - 01 - 01), date!(2026 - 12 - 31));
    let json = serde_json::to_string(&qualification).unwrap();
    let parsed: DeviceQualification = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, qualification);
    assert_eq!(parsed.issuing_authority(), "CASA");
    assert_eq!(parsed.level(), "Level D");
}

#[test]
fn deserialising_an_invalid_period_fails() {
    let json = r#"{
        "issuing_state": "AU",
        "issuing_authority": "CASA",
        "level": "Level D",
        "valid_from": "2026-03-01",
        "valid_until": "2026-02-28"
    }"#;
    assert!(serde_json::from_str::<DeviceQualification>(json).is_err());
}
