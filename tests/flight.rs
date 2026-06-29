//! Flight parity — mirrors `tests/models/flight/test_flight.py`.

use icao_shared_kernel::Flight;
use time::macros::date;
use uuid::Uuid;

#[test]
fn defaults_id_and_no_flight_time() {
    let flight = Flight::create(Uuid::new_v4(), date!(2026 - 04 - 19), "YSBK", "YSCN").unwrap();
    assert!(flight.flight_time.is_none());
    assert_ne!(flight.id, Uuid::nil());
}

#[test]
fn route_summary() {
    let flight = Flight::create(Uuid::new_v4(), date!(2026 - 04 - 19), "YSBK", "YSCN").unwrap();
    assert_eq!(flight.route_summary(), "YSBK-YSCN");
}

#[test]
fn display_renders_date_and_route() {
    let flight = Flight::create(Uuid::new_v4(), date!(2026 - 04 - 19), "YSBK", "YSCN").unwrap();
    assert_eq!(flight.to_string(), "Flight 2026-04-19: YSBK-YSCN");
}

#[test]
fn invalid_waypoints_rejected() {
    assert!(Flight::create(Uuid::new_v4(), date!(2026 - 04 - 19), "ybth", "YSCN").is_err());
    assert!(Flight::create(Uuid::new_v4(), date!(2026 - 04 - 19), "YSBK", "YS-BK").is_err());
}

#[test]
fn json_round_trip_preserves_value() {
    let flight = Flight::create(Uuid::new_v4(), date!(2026 - 04 - 19), "YSBK", "YSCN").unwrap();
    let json = serde_json::to_string(&flight).unwrap();
    let back: Flight = serde_json::from_str(&json).unwrap();
    assert_eq!(back, flight);
}
