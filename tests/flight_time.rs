//! FlightTime parity — mirrors `tests/models/flight/test_flight_time.py`.

use icao_shared_kernel::FlightTime;
use rust_decimal_macros::dec;

#[test]
fn properties() {
    let cases = [
        (0i64, 0i64, 0i64, "0:00"),
        (30, 0, 30, "0:30"),
        (60, 1, 0, "1:00"),
        (90, 1, 30, "1:30"),
        (135, 2, 15, "2:15"),
        (720, 12, 0, "12:00"),
        (1439, 23, 59, "23:59"),
    ];
    for (total, hours, minutes, rendered) in cases {
        let ft = FlightTime::new(total);
        assert_eq!(ft.total_minutes(), total);
        assert_eq!(ft.hours(), hours);
        assert_eq!(ft.minutes(), minutes);
        assert_eq!(ft.to_string(), rendered);
    }
}

#[test]
fn decimal_hours() {
    let cases = [
        (0i64, dec!(0)),
        (30, dec!(0.5)),
        (60, dec!(1.0)),
        (90, dec!(1.5)),
        (135, dec!(2.25)),
    ];
    for (total, expected) in cases {
        assert_eq!(FlightTime::new(total).decimal_hours(), expected);
    }
}

#[test]
fn serialises_as_int() {
    let ft = FlightTime::new(90);
    assert_eq!(serde_json::to_string(&ft).unwrap(), "90");
    let back: FlightTime = serde_json::from_str("90").unwrap();
    assert_eq!(back, ft);
}
