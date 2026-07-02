//! Flight aggregate construction, rendering, derived duration, and serialisation.

use icao_shared_kernel::{Coordinate, Flight, FlightDuration, SignificantPoint};
use rstest::rstest;
use rust_decimal_macros::dec;
use time::macros::utc_datetime;
use uuid::Uuid;

#[test]
fn defaults_id_and_no_movement_times() {
    let flight = Flight::create(Uuid::new_v4(), "YSBK", "YSCN").unwrap();
    assert!(flight.first_movement.is_none());
    assert!(flight.last_movement.is_none());
    assert_ne!(flight.id, Uuid::nil());
}

#[test]
fn route_summary() {
    let flight = Flight::create(Uuid::new_v4(), "YSBK", "YSCN").unwrap();
    assert_eq!(flight.route_summary(), "YSBK-YSCN");
}

#[test]
fn duration_is_none_without_both_movements() {
    let flight = Flight::create(Uuid::new_v4(), "YSBK", "YSCN").unwrap();
    assert_eq!(flight.duration(), None);
}

#[test]
fn duration_derived_from_movements() {
    let flight = Flight::new(
        Uuid::new_v4(),
        SignificantPoint::designator("YSBK").unwrap(),
        SignificantPoint::designator("YSCN").unwrap(),
        Some(utc_datetime!(2026-04-19 03:00)),
        Some(utc_datetime!(2026-04-19 04:30)),
    );
    assert_eq!(flight.duration(), Some(FlightDuration::new(90)));
}

#[test]
fn display_without_movement_time_omits_date() {
    let flight = Flight::create(Uuid::new_v4(), "YSBK", "YSCN").unwrap();
    assert_eq!(flight.to_string(), "Flight: YSBK-YSCN");
}

#[test]
fn display_with_movement_time_shows_date() {
    let flight = Flight::new(
        Uuid::new_v4(),
        SignificantPoint::designator("YSBK").unwrap(),
        SignificantPoint::designator("YSCN").unwrap(),
        Some(utc_datetime!(2026-04-19 03:00)),
        None,
    );
    assert_eq!(flight.to_string(), "Flight 2026-04-19: YSBK-YSCN");
}

#[rstest]
#[case("ybth", "YSCN")] // invalid departure designator
#[case("YSBK", "YS-BK")] // invalid arrival designator
fn invalid_designators_rejected(#[case] departure: &str, #[case] arrival: &str) {
    assert!(Flight::create(Uuid::new_v4(), departure, arrival).is_err());
}

#[test]
fn coordinate_endpoint_supported() {
    let coordinate = Coordinate::new(dec!(-33.9461), dec!(151.1772)).unwrap();
    let flight = Flight::new(
        Uuid::new_v4(),
        SignificantPoint::Coordinate(coordinate),
        SignificantPoint::designator("YSCN").unwrap(),
        None,
        None,
    );
    assert_eq!(flight.route_summary(), "-33.9461,151.1772-YSCN");
}

#[test]
fn json_round_trip_preserves_value() {
    let flight = Flight::new(
        Uuid::new_v4(),
        SignificantPoint::designator("YSBK").unwrap(),
        SignificantPoint::Coordinate(Coordinate::new(dec!(-33.9461), dec!(151.1772)).unwrap()),
        Some(utc_datetime!(2026-04-19 03:00)),
        Some(utc_datetime!(2026-04-19 04:30)),
    );
    let json = serde_json::to_string(&flight).unwrap();
    let back: Flight = serde_json::from_str(&json).unwrap();
    assert_eq!(back, flight);
}
