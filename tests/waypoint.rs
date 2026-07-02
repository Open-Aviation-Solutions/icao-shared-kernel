//! Waypoint designator validation.

use icao_shared_kernel::{DomainError, Waypoint};
use rstest::rstest;

#[rstest]
#[case("YSBK")]
#[case("AB")]
#[case("ABCDE")]
#[case("RIVET")]
#[case("DCT01")]
#[case("WAY99")]
fn valid_codes_accepted(#[case] value: &str) {
    let waypoint = Waypoint::parse(value).expect("should be valid");
    assert_eq!(waypoint.as_str(), value);
}

#[rstest]
#[case("ybth")] // lowercase
#[case("Ybth")] // mixed case
#[case("A")] // too short
#[case("ABCDEF")] // too long
#[case("")] // empty
#[case("YS-BK")] // non-alphanumeric
fn invalid_codes_rejected(#[case] value: &str) {
    assert!(
        Waypoint::parse(value).is_err(),
        "{value:?} should be rejected"
    );
}

#[rstest]
#[case("A", "Waypoint 'A' must be 2-5 characters long")]
#[case("ybth", "Waypoint 'ybth' must be uppercase alphanumeric")]
fn error_messages(#[case] value: &str, #[case] expected: &str) {
    assert_eq!(Waypoint::parse(value).unwrap_err().to_string(), expected);
}

#[test]
fn all_digit_code_rejected() {
    // A designator with no letters is rejected: it must contain at least one
    // A–Z character, not only digits.
    assert!(matches!(
        Waypoint::parse("12345"),
        Err(DomainError::WaypointCharset(_))
    ));
}
