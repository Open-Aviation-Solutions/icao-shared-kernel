// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! FlightDuration decomposition, formatting, and serialisation.

use icao_shared_kernel::{FlightDuration, ValidationError};
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
    let ft = FlightDuration::new(total).unwrap();
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
        FlightDuration::from_hours_minutes(hours, minutes)
            .unwrap()
            .total_minutes(),
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
    assert_eq!(
        FlightDuration::new(total).unwrap().decimal_hours(),
        expected
    );
}

#[test]
fn serialises_as_int() {
    let ft = FlightDuration::new(90).unwrap();
    assert_eq!(serde_json::to_string(&ft).unwrap(), "90");
    let back: FlightDuration = serde_json::from_str("90").unwrap();
    assert_eq!(back, ft);
}

#[rstest]
#[case(-1)]
#[case(-90)]
#[case(i64::MIN)]
fn refuses_a_negative_value(#[case] total: i64) {
    let expected = Err(ValidationError::FlightDuration(total));
    assert_eq!(FlightDuration::new(total), expected);
    assert_eq!(FlightDuration::try_from(total), expected);
    assert_eq!(
        serde_json::from_str::<FlightDuration>(&total.to_string())
            .unwrap_err()
            .to_string(),
        ValidationError::FlightDuration(total).to_string()
    );
}

#[test]
fn from_hours_minutes_refuses_a_negative_total() {
    assert_eq!(
        FlightDuration::from_hours_minutes(1, -90),
        Err(ValidationError::FlightDuration(-30))
    );
    assert_eq!(
        FlightDuration::from_hours_minutes(1, -60)
            .unwrap()
            .total_minutes(),
        0
    );
}

#[test]
fn from_hours_minutes_refuses_overflow_instead_of_wrapping() {
    assert_eq!(
        FlightDuration::from_hours_minutes(i64::MAX, 0),
        Err(ValidationError::FlightDurationOverflow {
            hours: i64::MAX,
            minutes: 0
        })
    );
    assert!(FlightDuration::from_hours_minutes(0, i64::MAX).is_ok());
    assert!(
        FlightDuration::from_hours_minutes(0, i64::MAX)
            .unwrap()
            .total_minutes()
            == i64::MAX
    );
    assert!(FlightDuration::from_hours_minutes(1, i64::MAX).is_err());
}
