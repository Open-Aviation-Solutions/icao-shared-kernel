//! FSTD session route, duration, and the per-session simulated type.

use icao_shared_kernel::{AircraftType, FlightRules, FstdSession, SignificantPoint};
use time::macros::utc_datetime;
use uuid::Uuid;

fn point(designator: &str) -> SignificantPoint {
    SignificantPoint::designator(designator).expect("should be valid")
}

#[test]
fn with_preserves_explicit_id() {
    let id = Uuid::new_v4();
    let session = FstdSession::with(id, Uuid::new_v4(), None, None, None, None, None);
    assert_eq!(session.id, id);
}

#[test]
fn duration_derived_from_timestamps() {
    let session = FstdSession::new(
        Uuid::new_v4(),
        None,
        None,
        None,
        Some(utc_datetime!(2026-05-01 01:00)),
        Some(utc_datetime!(2026-05-01 02:30)),
    );
    assert_eq!(session.duration().unwrap().total_minutes(), 90);
}

#[test]
fn duration_is_none_without_both_timestamps() {
    let session = FstdSession::new(
        Uuid::new_v4(),
        None,
        None,
        None,
        Some(utc_datetime!(2026-05-01 01:00)),
        None,
    );
    assert!(session.duration().is_none());
}

#[test]
fn route_summary_needs_both_endpoints() {
    let device_id = Uuid::new_v4();
    let routed = FstdSession::new(
        device_id,
        None,
        Some(point("YSBK")),
        Some(point("YSCN")),
        None,
        None,
    );
    assert_eq!(routed.route_summary().unwrap(), "YSBK-YSCN");

    let one_ended = FstdSession::new(device_id, None, Some(point("YSBK")), None, None, None);
    assert!(one_ended.route_summary().is_none());
}

#[test]
fn manoeuvre_only_session_records_nothing_but_the_device() {
    let device_id = Uuid::new_v4();
    let session = FstdSession::new(device_id, None, None, None, None, None);
    assert_eq!(session.device_id, device_id);
    assert!(session.simulated_aircraft_type.is_none());
    assert_eq!(session.to_string(), "FSTD session");
}

#[test]
fn one_device_can_simulate_different_types_across_sessions() {
    // An unqualified personal device is not bound to a single simulated
    // aircraft, so the type is recorded per session rather than on the device.
    let device_id = Uuid::new_v4();
    let cessna = FstdSession::create(device_id, "C172").unwrap();
    let boeing = FstdSession::create(device_id, "B738").unwrap();

    assert_eq!(cessna.device_id, boeing.device_id);
    assert_eq!(
        cessna.simulated_aircraft_type.unwrap(),
        AircraftType::parse("C172").unwrap()
    );
    assert_eq!(
        boeing.simulated_aircraft_type.unwrap(),
        AircraftType::parse("B738").unwrap()
    );
}

#[test]
fn create_rejects_an_invalid_type_designator() {
    assert!(FstdSession::create(Uuid::new_v4(), "c172").is_err());
}

#[test]
fn display_includes_type_and_route_when_known() {
    let session = FstdSession::new(
        Uuid::new_v4(),
        Some(AircraftType::parse("B738").unwrap()),
        Some(point("YSSY")),
        Some(point("YMML")),
        None,
        None,
    );
    assert_eq!(session.to_string(), "FSTD session (B738): YSSY-YMML");
}

#[test]
fn session_round_trips_through_json() {
    let session = FstdSession::new(
        Uuid::new_v4(),
        Some(AircraftType::parse("C172").unwrap()),
        Some(point("YSBK")),
        Some(point("YSCN")),
        Some(utc_datetime!(2026-05-01 01:00)),
        Some(utc_datetime!(2026-05-01 02:00)),
    )
    .with_flight_rules(Some(FlightRules::Ifr));
    let json = serde_json::to_string(&session).unwrap();
    let parsed: FstdSession = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, session);
}

#[test]
fn a_session_recorded_without_flight_rules_reads_as_none() {
    let session = FstdSession::create(Uuid::new_v4(), "C172").unwrap();
    let mut json = serde_json::to_value(&session).unwrap();
    json.as_object_mut().unwrap().remove("flight_rules");
    let back: FstdSession = serde_json::from_value(json).unwrap();
    assert_eq!(back.flight_rules, None);
}
