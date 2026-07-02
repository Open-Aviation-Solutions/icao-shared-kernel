//! Coordinate range validation and SignificantPoint (untagged) serialisation.

use icao_shared_kernel::{Coordinate, SignificantPoint, Waypoint};
use rstest::rstest;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[rstest]
#[case(dec!(-33.9461), dec!(151.1772))]
#[case(dec!(90), dec!(180))]
#[case(dec!(-90), dec!(-180))]
#[case(dec!(0), dec!(0))]
fn valid_coordinates_accepted(#[case] lat: Decimal, #[case] lon: Decimal) {
    assert!(Coordinate::new(lat, lon).is_ok());
}

#[rstest]
#[case(dec!(90.1), dec!(0))] // latitude too high
#[case(dec!(-90.1), dec!(0))] // latitude too low
#[case(dec!(0), dec!(180.1))] // longitude too high
#[case(dec!(0), dec!(-180.1))] // longitude too low
fn out_of_range_coordinates_rejected(#[case] lat: Decimal, #[case] lon: Decimal) {
    assert!(Coordinate::new(lat, lon).is_err());
}

#[test]
fn designator_serialises_as_bare_string() {
    let point = SignificantPoint::Designator(Waypoint::parse("YSBK").unwrap());
    assert_eq!(serde_json::to_string(&point).unwrap(), "\"YSBK\"");
    let back: SignificantPoint = serde_json::from_str("\"YSBK\"").unwrap();
    assert_eq!(back, point);
}

#[test]
fn coordinate_serialises_as_object() {
    let point =
        SignificantPoint::Coordinate(Coordinate::new(dec!(-33.9461), dec!(151.1772)).unwrap());
    let json = serde_json::to_string(&point).unwrap();
    let back: SignificantPoint = serde_json::from_str(&json).unwrap();
    assert_eq!(back, point);
    // The untagged form is an object, distinguishable from the string designator.
    assert!(json.starts_with('{'));
}
