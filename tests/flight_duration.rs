//! FlightDuration decomposition, formatting, and serialisation.

use icao_shared_kernel::FlightDuration;
use rstest::rstest;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[rstest]
#[case(0, 0, 0, "0:00")]
#[case(30, 0, 30, "0:30")]
#[case(60, 1, 0, "1:00")]
#[case(90, 1, 30, "1:30")]
#[case(135, 2, 15, "2:15")]
#[case(720, 12, 0, "12:00")]
#[case(1439, 23, 59, "23:59")]
fn properties(
    #[case] total: i64,
    #[case] hours: i64,
    #[case] minutes: i64,
    #[case] rendered: &str,
) {
    let ft = FlightDuration::new(total);
    assert_eq!(ft.total_minutes(), total);
    assert_eq!(ft.hours(), hours);
    assert_eq!(ft.minutes(), minutes);
    assert_eq!(ft.to_string(), rendered);
}

#[rstest]
#[case(1, 30, 90)]
#[case(0, 45, 45)]
#[case(2, 0, 120)]
fn from_hours_minutes(#[case] hours: i64, #[case] minutes: i64, #[case] expected_total: i64) {
    assert_eq!(
        FlightDuration::from_hours_minutes(hours, minutes).total_minutes(),
        expected_total
    );
}

#[rstest]
#[case(0, dec!(0))]
#[case(30, dec!(0.5))]
#[case(60, dec!(1.0))]
#[case(90, dec!(1.5))]
#[case(135, dec!(2.25))]
fn decimal_hours(#[case] total: i64, #[case] expected: Decimal) {
    assert_eq!(FlightDuration::new(total).decimal_hours(), expected);
}

#[test]
fn serialises_as_int() {
    let ft = FlightDuration::new(90);
    assert_eq!(serde_json::to_string(&ft).unwrap(), "90");
    let back: FlightDuration = serde_json::from_str("90").unwrap();
    assert_eq!(back, ft);
}
